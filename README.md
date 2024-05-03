### EVM Token Tools [Rust]

1. install rustup 

    https://www.rust-lang.org/tools/install

2. add to .env 

    (see .env.template)

3. run these scripts 

    cargo run --bin SCRIPT_NAME

```
cargo run --bin erc20_approve -- --token-address 0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14 --approved-address 0x8DAE5766bC47376cc7c14cbAA0AeE511829F5dDe --amount 10000000000

```





#### Sample .env file 

```

RPC_URL="https://mainnet.g.alchemy.com/v2/xxxx"

PRIVATE_KEY="65xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxd6"

CHAIN_ID=5



```
