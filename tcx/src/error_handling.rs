use crate::filemanager::KEYSTORE_MAP;

use std::{cell::RefCell, mem, panic};

use anyhow::{format_err, Error, Result};
use std::backtrace::Backtrace;

thread_local! {
    pub static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
}

#[cfg_attr(tarpaulin, skip)]
#[allow(irrefutable_let_patterns)]
fn notify_err(err: Error) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(err);
    });
}

fn lock_all_keystore() {
    let mut map = KEYSTORE_MAP.write();
    for ks in map.values_mut() {
        ks.lock();
    }
}

/// catch any error and format to string
/// ref: <https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/controlling-panics-with-std-panic.html>
#[cfg_attr(tarpaulin, skip)]
pub unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(f: F) -> T {
    match panic::catch_unwind(f) {
        Ok(rv) => {
            lock_all_keystore();
            rv.map_err(notify_err).unwrap_or_else(|_| mem::zeroed())
        }
        Err(err) => {
            lock_all_keystore();
            use std::any::Any;
            let err = &*err as &dyn Any;
            let msg = match err.downcast_ref::<&str>() {
                Some(s) => *s,
                None => match err.downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };
            notify_err(format_err!("{}", msg));
            mem::zeroed()
        }
    }
}
