# Commands:

```sh
cargo build --target wasm32-unknown-unknown --release

near login

near create-account voting-test.v-rokas.testnet --masterAccount v-rokas.testnet --contractName voting-test --initialBalance 10

near deploy --wasmFile target/wasm32-unknown-unknown/release/contract.wasm --accountId voting-test.v-rokas.testnet

near send v-rokas.testnet voting-test.v-rokas.testnet 20

near call voting-test.v-rokas.testnet get_total_votes --accountId v-rokas.testnet

near call voting-test.v-rokas.testnet vote '{"choice": "choice_1"}' --accountId v-rokas.testnet
```
