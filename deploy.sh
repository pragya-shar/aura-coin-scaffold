#!/bin/bash

echo "🚀 Deploying AuraCoin Contract to Stellar Testnet"
echo "=================================================="

# Set network
echo "📡 Setting up testnet network..."
soroban network add testnet --rpc-url https://soroban-testnet.stellar.org --network-passphrase "Test SDF Network ; September 2015" 2>/dev/null || true

# Generate key if not exists
if [ ! -f ".stellar/identity/alice.toml" ]; then
    echo "🔑 Generating new key pair..."
    soroban keys generate alice
fi

# Get the public key
echo "🔍 Getting public key..."
PUBLIC_KEY=$(soroban keys show alice | grep -o 'G[A-Z0-9]*')
echo "Public Key: $PUBLIC_KEY"

# Fund the account
echo "💰 Funding account..."
curl "https://friendbot.stellar.org/?addr=$PUBLIC_KEY" > /dev/null 2>&1
sleep 3

# Build the contract
echo "🔨 Building contract..."
cd contracts/fungible
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Deploy the contract with constructor arguments
echo "📦 Deploying contract..."
CONTRACT_ID=$(soroban contract deploy --network testnet --source-account alice --wasm target/wasm32-unknown-unknown/release/fungible_contract.wasm -- --owner $PUBLIC_KEY)

echo "✅ Contract deployed with ID: $CONTRACT_ID"

echo "🎉 AuraCoin contract successfully deployed and initialized!"
echo "Contract ID: $CONTRACT_ID"
echo "Owner: $PUBLIC_KEY"
echo ""
echo "📝 Next steps:"
echo "1. Add this contract ID to your web app"
echo "2. Test minting tokens: soroban contract invoke --id $CONTRACT_ID --network testnet --source alice -- mint --account $PUBLIC_KEY --amount 1000"
echo "3. Check balance: soroban contract invoke --id $CONTRACT_ID --network testnet --source alice -- balance --id $PUBLIC_KEY" 