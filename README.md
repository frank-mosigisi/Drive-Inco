# DriveInco Cab Services Smart Contract

The smart contract is currently deployed and initialized on:

    maryjane.mememan.testnet


### Create a new account;

    near deploy --account-id maryjane.mememan.testnet --wasmFile target/wasm32-unknown-unknown/release/rust_template.wasm

### Getting instructions for interacting with the smart Contract

    near view maryjane.mememan.testnet welcome --account-id mememan.testnet

### Calling for the available routes ;

    near call maryjane.mememan.testnet destination --account-id mememan.testnet

### Making an cab order ;

    near call maryjane.mememan.testnet order '{"cab_number": 2 , "driver_choice": "mary"}' --account-id mememan.testnet

### Ask for your fare after trip ;

    near view maryjane.mememan.testnet charges '{"cab_number": 2 }' --account-id mememan.testnet

### Making a higher payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 2 }' --account-id mememan.testnet --deposit 5

### Making the exact payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 2 }' --account-id mememan.testnet --deposit 2

### Making a lesser payment ;

    near call maryjane.mememan.testnet payement '{"cab_number": 2}' --account-id mememan.testnet --deposit 1
