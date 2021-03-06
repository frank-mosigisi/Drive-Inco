# DriveInco Cab Services Smart Contract
## About 

This is a cab service contract where clients can book and pay for a cab using Near tokens.They can check available cabs at the moment, book a cab, travel and also pay for their trip when it is done.
This can all be done securely and anonymously by leveraging on the characteristics of the Near Blockchain.

The smart contract is currently deployed and initialized on:

    maryjane.mememan.testnet

### Create a new account;

    near deploy --account-id maryjane.mememan.testnet --wasmFile target/wasm32-unknown-unknown/release/rust_template.wasm

### Getting instructions for interacting with the smart Contract

    near view maryjane.mememan.testnet welcome --account-id YOUR_ACCOUNT_ID_HERE

### Calling for the available routes ;

    near call maryjane.mememan.testnet destination --account-id YOUR_ACCOUNT_ID_HERE

### Making an cab order ;

    near call maryjane.mememan.testnet order '{"cab_number": 3 , "driver_choice": "kiki"}' --account-id YOUR_ACCOUNT_ID_HERE

### Ask for your trip charges ;

    near view maryjane.mememan.testnet charges '{"cab_number": 3 }' --account-id YOUR_ACCOUNT_ID_HERE

### Making a higher payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 3 }' --account-id YOUR_ACCOUNT_ID_HERE --deposit 5

### Making the exact payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 3 }' --account-id YOUR_ACCOUNT_ID_HERE --deposit 3

### Making a lesser payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 3}' --account-id YOUR_ACCOUNT_ID_HERE --deposit 1

Built with the [Near Rust Template ](https://github.com/near/near-sdk-rs#pre-requisites)

<!-- 8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release` -->

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
