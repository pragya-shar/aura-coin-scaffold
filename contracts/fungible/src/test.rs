#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ AuraCoin, AuraCoinClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(AuraCoin, (Address::generate(&env),));
    let client = AuraCoinClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "Aura Coin"));
    assert_eq!(client.symbol(), String::from_str(&env, "AURA"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn test_mint() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Initial balance should be 0
    assert_eq!(client.balance(&recipient), 0);
    
    // Mint 1000 tokens to recipient (owner-only function)
    let mint_amount = 1000;
    env.mock_all_auths();
    client.mint(&recipient, &mint_amount);
    
    // Check balance after minting
    assert_eq!(client.balance(&recipient), mint_amount);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Mint tokens to sender (owner-only function)
    let initial_amount = 1000;
    env.mock_all_auths();
    client.mint(&sender, &initial_amount);
    
    // Transfer 500 tokens from sender to recipient
    let transfer_amount = 500;
    env.mock_all_auths();
    client.transfer(&sender, &recipient, &transfer_amount);
    
    // Check balances after transfer
    assert_eq!(client.balance(&sender), initial_amount - transfer_amount);
    assert_eq!(client.balance(&recipient), transfer_amount);
}

#[test]
fn test_burn() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let account = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Mint tokens to account (owner-only function)
    let initial_amount = 1000;
    env.mock_all_auths();
    client.mint(&account, &initial_amount);
    
    // Burn 300 tokens from account
    let burn_amount = 300;
    env.mock_all_auths();
    client.burn(&account, &burn_amount);
    
    // Check balance after burning
    assert_eq!(client.balance(&account), initial_amount - burn_amount);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Mint tokens to sender (owner-only function)
    env.mock_all_auths();
    client.mint(&sender, &1000);
    
    // Initially not paused
    assert_eq!(client.paused(), false);
    
    // Pause the contract (owner-only function)
    env.mock_all_auths();
    client.pause(&owner);
    assert_eq!(client.paused(), true);
    
    // Unpause the contract (owner-only function)
    env.mock_all_auths();
    client.unpause(&owner);
    assert_eq!(client.paused(), false);
    
    // Transfer should work again
    env.mock_all_auths();
    client.transfer(&sender, &recipient, &100);
    assert_eq!(client.balance(&recipient), 100);
}

#[test]
fn test_ownership() {
    let env = Env::default();
    let owner = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Check initial owner
    assert_eq!(client.get_owner(), Some(owner));
}

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Mint tokens to 'from' account (owner-only function)
    env.mock_all_auths();
    client.mint(&from, &1000);
    
    // Approve spender to spend 500 tokens from 'from' account
    let approve_amount = 500;
    let expiration_ledger = env.ledger().sequence() + 100;
    env.mock_all_auths();
    client.approve(&from, &spender, &approve_amount, &expiration_ledger);
    
    // Check allowance
    assert_eq!(client.allowance(&from, &spender), approve_amount);
    
    // Transfer 300 tokens using transfer_from
    let transfer_amount = 300;
    env.mock_all_auths();
    client.transfer_from(&spender, &from, &to, &transfer_amount);
    
    // Check balances and allowance
    assert_eq!(client.balance(&from), 1000 - transfer_amount);
    assert_eq!(client.balance(&to), transfer_amount);
    assert_eq!(client.allowance(&from, &spender), approve_amount - transfer_amount);
}

#[test]
fn test_total_supply() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    // Initial total supply should be 0
    assert_eq!(client.total_supply(), 0);
    
    // Mint tokens to multiple recipients (owner-only function)
    env.mock_all_auths();
    client.mint(&recipient1, &1000);
    env.mock_all_auths();
    client.mint(&recipient2, &500);
    
    // Check total supply
    assert_eq!(client.total_supply(), 1500);
    
    // Burn some tokens
    env.mock_all_auths();
    client.burn(&recipient1, &200);
    
    // Check total supply after burning
    assert_eq!(client.total_supply(), 1300);
}
