## cmmand and execute result

### ./hd-wallet private_key_store_import "{\"method\": \"private_key_store_import\", \"param\": { \"private_key\": \"9ef99d4d5647abb965b58ad7a0f040eaa2aa5affbfc3d1aff2e9d442ba109175\", \"name\": \"test\", \"overwrite\": true, \"password\": \"\", \"password_hint\": \"\", \"encoding\": \"ETHEREUM\" } }"

`
"{\"id\":\"51d39c38-994f-448a-a2bd-2cd97cafb0fc\",\"name\":\"test\",\"source\":\"PRIVATE\",\"accounts\":[],\"created_at\":1677562559}"
`

### ./hd-wallet keystore_common_derive "{\"method\": \"keystore_common_derive\", \"param\": { \"id\": \"51d39c38-994f-448a-a2bd-2cd97cafb0fc\", \"password\": \"\", \"derivations\": [{ \"chain_type\": \"ETHEREUM\", \"path\": \"\", \"network\": \"\", \"seg_wit\": \"\", \"chain_id\": \"\", \"curve\": \"\" }] } }"

`
"{\"accounts\":[{\"chain_type\":\"ETHEREUM\",\"address\":\"0x262AEa7144448C607ed7C28ab06Dd38dB658c689\",\"path\":\"\",\"extended_xpub_key\":\"\"}]}"
`


### ./hd-wallet sign_tx "{\"method\": \"sign_tx\", \"param\": { \"id\": \"51d39c38-994f-448a-a2bd-2cd97cafb0fc\", \"chain_type\": \"ETHEREUM\", \"address\": \"0x262AEa7144448C607ed7C28ab06Dd38dB658c689\", \"input\": { \"nonce\": \"1\", \"to\": \"0x362aea7144448c607ed7c28ab06dd38db658c689\", \"value\": \"1000000000000000\", \"gas_price\": \"1000000000000000\", \"gas\": \"21000\", \"data\": \"\", \"network\": \"GOERLI\" }, \"key\": { \"Password\": \"\" } } }"


`{\"signature\":\"f870018810000000000000008302100094362aea7144448c607ed7c28ab06dd38db658c689881000000000000000802da05da979b101bff93cef9d45cddafe0c4da9d9e1b27563dc28566016dfa7788134a00699a98f3cbfe618f3fe0ab02d6ac9516c97ea5474bfd1afa920a22c5ebb1dd0\"}"
`