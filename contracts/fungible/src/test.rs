#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ AuraCoin, AuraCoinClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(AuraCoin, (Address::generate(&env),));
    let client = AuraCoinClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "AuraCoin"));
}

// Add more tests bellow
