## cmmand and execute result

### ./hd-wallet init_token_core_x ../test_data

### ./hd-wallet hd_store_import yubo "sugar car august uniform romance upset burger mesh polar scissors frame mention" ""
`wallet imported WalletResult { id: "b2736cc9-fe1b-445e-8d31-cb69fce07017", name: "yubo", source: "MNEMONIC", accounts: [], created_at: 1676037405 }`

### ./hd-wallet export_mnemonic b2736cc9-fe1b-445e-8d31-cb69fce07017 ""
`export_mnemonic KeystoreCommonExportResult { id: "b2736cc9-fe1b-445e-8d31-cb69fce07017", r#type: Mnemonic, value: "sugar car august uniform romance upset burger mesh polar scissors frame mention" }`

### ./hd-wallet keystore_common_derive b2736cc9-fe1b-445e-8d31-cb69fce07017 "" DEEPER "" ""
`keystore_common_derive AccountsResponse { accounts: [AccountResponse { chain_type: "DEEPER", address: "5HWaC8SdPFFq6uSrWZD1bHKhEg43JjzARFLvuf4CJ1zN5HMF", path: "", extended_xpub_key: "" }] }`

### ./hd-wallet keystore_common_accounts b2736cc9-fe1b-445e-8d31-cb69fce07017
`keystore_common_accounts AccountsResponse { accounts: [AccountResponse { chain_type: "DEEPER", address: "5HWaC8SdPFFq6uSrWZD1bHKhEg43JjzARFLvuf4CJ1zN5HMF", path: "", extended_xpub_key: "" }] }`

### ./hd-wallet tx_input DEEPER 1234567890
`input "0a0a31323334353637383930"`

### ./hd-wallet sign_tx b2736cc9-fe1b-445e-8d31-cb69fce07017 "" DEEPER 5HWaC8SdPFFq6uSrWZD1bHKhEg43JjzARFLvuf4CJ1zN5HMF 0a0a31323334353637383930
`sign_tx result "0a8401307830313538393835383666646539383364663532393431373430303632363032616630613030616530333166373530613666363037323435343237656264313466376362306561646566626236663436323833366365313666306631326533663331633331656139383162316630633764386138363330666263396366666166323836"`

### ./hd-wallet tx_output DEEPER 0a8401307830313538393835383666646539383364663532393431373430303632363032616630613030616530333166373530613666363037323435343237656264313466376362306561646566626236663436323833366365313666306631326533663331633331656139383162316630633764386138363330666263396366666166323836
`DEEPER output "5898586fde983df52941740062602af0a00ae031f750a6f607245427ebd14f7cb0eadefbb6f462836ce16f0f12e3f31c31ea981b1f0c7d8a8630fbc9cffaf286"`
