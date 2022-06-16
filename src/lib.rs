use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

mod info;
/* This is a cab riding service contract where clients can order a cab, take a ride and pay fare after a sucessfull ride*/

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // A list of car choices to various destinations and the price for each vehicle
    // cab numbers are used as keys.
    drivers_available: HashMap<String, f32>,
    cab_allocation: HashMap<u8, Client>,
}
/*
    This is all the client's info which includes cab driver used, destination and also cost of the trip in NEAR
*/
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Client {
    number: u8,
    destination: String,
    cost: f32,
}

// Initialization function for the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            drivers_available: info::cab_fares(),
            cab_allocation: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    //Welcome message and available contract functions available for use
    pub fn welcome(&self) {
        info::help();
    }
    // Call to get the rides available
    pub fn destination() {
        info::destinations();
    }
    /*
        Order function a client calls to book a cab ride. The client gives his desination and cab driver.
        This information is used to initialize a client object
    */
    pub fn order(&mut self, cab_number: u8, driver_choice: String) {
        let driver_choice = driver_choice.to_lowercase();
        // Check if cab is available under cab_fares
        if self.drivers_available.contains_key(&driver_choice) {
            env::log_str(format!("Your cab order has been confirmed. Your driver will be {} .Have a nice trip" ,&driver_choice).as_ref());
            //we calculate the cost of the cab ride based on driver_choice by the customer
            let cost: f32 = self.drivers_available[&driver_choice];
            let client_new = Client {
                number: cab_number,
                destination: driver_choice,
                cost: cost,
            };
            // call the add_client method passing the generated Client and the cab_number as key
            self.add_client(client_new.clone(), cab_number);
            // If the cab does not exist on the list then u get a message telling u so
        } else {
            env::log_str("The ride requested is not available, request again please");
        };
    }

    // Called to give total cab fare for the trip
    pub fn charges(&self, cab_number: u8) {
        log!(
            "We hope you had an excellent trip. Your cab fare will be {} near",
            self.cab_allocation[&cab_number].cost
        );
    }

    // non pub-function to add the newly generated client to the cab_allocation hashmap
    #[result_serializer(borsh)]
    fn add_client(&mut self, client: Client, cab: u8) {
        self.cab_allocation.insert(cab, client);
    }

    // A payable function where clients pay for their cab fares
    #[payable]
    pub fn payement(&mut self, cab_number: u8) ->String {
        // Assign the attached near and the fare cost
        let tokens = env::attached_deposit();
        let charge = self.cab_allocation[&cab_number].cost;
        log!("deposited {} ", tokens);
        // convert unsigned integer to float and yocto to near
        let tokens_paid = to_near(tokens);
        log!("cost: {}, token near {}", charge, tokens_paid);
        // if checks to compare the tokens recieved to the expected fare charges
        if tokens_paid < charge {
            return "Insufficient cab fare paid.kindly pay the correct amount".to_string()
        }
        if tokens_paid - charge > 0.0 {
            let log_message = format!("You paid more by {} Near. Thanks for the tip", (tokens_paid - charge));
            return log_message.to_string()    
        }
        else {
            return "Payement received succesfully. Have a nice day and book a ride with us next time".to_string()
        }
    }
}

fn to_near(yocto: u128) -> f32 {
    (yocto as f32) / 1_000_000_000_000_000_000_000_000.0
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "maryjane.mememan.testnet".to_string(),
            signer_account_id: "ink_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "mememan.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn paid_correct_amount() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = 2000000000000000000000000; 
        context.is_view = false;
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        contract.order(2, "mary".to_string());
        let response = contract.payement(2);
        assert_eq!("Payement received succesfully. Have a nice day and book a ride with us next time".to_string(), response)
    }

    #[test]
    fn paid_more() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = 5000000000000000000000000;
        context.is_view = false;
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        contract.order(2, "mary".to_string());
        let response = contract.payement(2);
        assert_eq!("You paid more by 2.9999995 Near. Thanks for the tip".to_string(), response)
    }

    #[test]
    fn pay_less_amount() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = 0000000000000000000000000; 
        context.is_view = false;
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        contract.order(2, "mary".to_string());
        let response = contract.payement(2);
        assert_eq!("Insufficient cab fare paid.kindly pay the correct amount".to_string(), response)
    }
    #[test]
    fn test_hash_map() {
        let contract: Contract = Contract::default();
        let bools = contract.drivers_available.contains_key(&"mary".to_string());
        log!("{}", bools);
        assert_eq!(true, bools)
    }
}