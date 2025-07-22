use soroban_sdk::{ testutils::Address as _, Address, Env, String };
use fungible_contract::contract::{ AuraCoin, AuraCoinClient };

fn main() {
    println!("🚀 AuraCoin Contract Interactive Test");
    println!("=====================================\n");

    // Setup environment
    let env = Env::default();
    let owner = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    println!("📋 Setup:");
    println!("  Owner: {}", owner);
    println!("  User1: {}", user1);
    println!("  User2: {}", user2);
    println!();

    // Deploy contract
    let contract_addr = env.register(AuraCoin, (&owner,));
    let client = AuraCoinClient::new(&env, &contract_addr);
    
    println!("✅ Contract deployed at: {}", contract_addr);
    println!();

    // Test 1: Initial state
    println!("🔍 Test 1: Initial State");
    println!("  Name: {}", client.name());
    println!("  Symbol: {}", client.symbol());
    println!("  Decimals: {}", client.decimals());
    println!("  Total Supply: {}", client.total_supply());
    println!("  Owner: {}", client.get_owner().unwrap());
    println!();

    // Test 2: Minting
    println!("🪙 Test 2: Minting Tokens");
    env.mock_all_auths();
    client.mint(&user1, &1000);
    client.mint(&user2, &500);
    
    println!("  Minted 1000 AURA to User1");
    println!("  Minted 500 AURA to User2");
    println!("  User1 Balance: {}", client.balance(&user1));
    println!("  User2 Balance: {}", client.balance(&user2));
    println!("  Total Supply: {}", client.total_supply());
    println!();

    // Test 3: Transfers
    println!("💸 Test 3: Token Transfers");
    env.mock_all_auths();
    client.transfer(&user1, &user2, &300);
    
    println!("  User1 transferred 300 AURA to User2");
    println!("  User1 Balance: {}", client.balance(&user1));
    println!("  User2 Balance: {}", client.balance(&user2));
    println!();

    // Test 4: Approvals and Delegated Transfers
    println!("🔐 Test 4: Approvals and Delegated Transfers");
    let spender = Address::generate(&env);
    let expiration_ledger = env.ledger().sequence() + 100;
    
    env.mock_all_auths();
    client.approve(&user1, &spender, &200, &expiration_ledger);
    println!("  User1 approved Spender to spend 200 AURA");
    println!("  Allowance: {}", client.allowance(&user1, &spender));
    
    env.mock_all_auths();
    client.transfer_from(&spender, &user1, &user2, &150);
    println!("  Spender transferred 150 AURA from User1 to User2");
    println!("  User1 Balance: {}", client.balance(&user1));
    println!("  User2 Balance: {}", client.balance(&user2));
    println!("  Remaining Allowance: {}", client.allowance(&user1, &spender));
    println!();

    // Test 5: Burning
    println!("🔥 Test 5: Token Burning");
    env.mock_all_auths();
    client.burn(&user2, &100);
    
    println!("  User2 burned 100 AURA");
    println!("  User2 Balance: {}", client.balance(&user2));
    println!("  Total Supply: {}", client.total_supply());
    println!();

    // Test 6: Pausing
    println!("⏸️ Test 6: Contract Pausing");
    println!("  Contract Paused: {}", client.paused());
    
    env.mock_all_auths();
    client.pause(&owner);
    println!("  Owner paused the contract");
    println!("  Contract Paused: {}", client.paused());
    
    env.mock_all_auths();
    client.unpause(&owner);
    println!("  Owner unpaused the contract");
    println!("  Contract Paused: {}", client.paused());
    println!();

    // Final Summary
    println!("📊 Final Summary:");
    println!("  Total Supply: {}", client.total_supply());
    println!("  User1 Balance: {}", client.balance(&user1));
    println!("  User2 Balance: {}", client.balance(&user2));
    println!("  Contract Paused: {}", client.paused());
    println!();

    println!("🎉 All tests completed successfully!");
} 