#!/bin/bash

echo "🚀 Deploying AuraCoin Contract with Your Wallet as Owner"
echo "========================================================"

# Your wallet address
OWNER_ADDRESS="GCYXOOV2VEQ2XXYO2DHLJ6JRZFAPEZKYOO5EUPWSPMELTW4IKJW3WGEI"

echo "👤 Owner Address: $OWNER_ADDRESS"

# Set network
echo "📡 Setting up testnet network..."
soroban network add testnet --rpc-url https://soroban-testnet.stellar.org --network-passphrase "Test SDF Network ; September 2015" 2>/dev/null || true

# Generate owner key if not exists
if [ ! -f ".stellar/identity/owner.toml" ]; then
    echo "🔑 Generating owner key pair..."
    soroban keys generate owner
fi

# Build the contract
echo "🔨 Building contract..."
cd contracts/fungible
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Deploy the contract with your address as owner
echo "📦 Deploying contract with your address as owner..."
CONTRACT_ID=$(soroban contract deploy --network testnet --source-account owner --wasm target/wasm32-unknown-unknown/release/fungible_contract.wasm -- --owner $OWNER_ADDRESS)

echo "✅ Contract deployed with ID: $CONTRACT_ID"

# Test the contract
echo "🧪 Testing contract..."
echo "Token Name: $(soroban contract invoke --id $CONTRACT_ID --network testnet --source owner -- name)"
echo "Token Symbol: $(soroban contract invoke --id $CONTRACT_ID --network testnet --source owner -- symbol)"
echo "Owner: $(soroban contract invoke --id $CONTRACT_ID --network testnet --source owner -- get_owner)"

echo ""
echo "🎉 AuraCoin contract successfully deployed with your wallet as owner!"
echo "Contract ID: $CONTRACT_ID"
echo "Owner: $OWNER_ADDRESS"
echo ""
echo "📝 Next steps:"
echo "1. Update the contract ID in your web app: $CONTRACT_ID"
echo "2. Test minting tokens with your wallet"
echo "3. Connect your wallet to the web app to see your AuraCoin balance"
echo ""
echo "🔗 Contract Explorer: https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID" 