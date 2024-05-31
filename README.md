# Cross-Chain Oracle for Bitcoin and Solana

This project implements a cross-chain oracle that bridges Bitcoin and Solana blockchains using zero-knowledge proofs (ZKPs). The oracle monitors Bitcoin ordinals and verifies them on Solana, allowing for secure and private cross-chain communication.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Oracle Service](#oracle-service)
- [Solana Smart Contract](#solana-smart-contract)
- [Client Interaction](#client-interaction)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Ensure you have the following tools and libraries installed:

- Python 3.x
- Bitcoin Core (full node)
- Solana CLI
- Anchor Framework
- Node.js

## Setup

### Bitcoin Core

1. **Install Bitcoin Core**:
   Download and install Bitcoin Core from the official [Bitcoin website](https://bitcoin.org/en/download).

2. **Configure Bitcoin Core**:
   Edit the `bitcoin.conf` file to enable RPC:
   ```conf
   server=1
   rpcuser=yourusername
   rpcpassword=yourpassword
   rpcallowip=127.0.0.1
3. Start Bitcoin Core:

bitcoind -daemon

Oracle Service

1. Install Required Libraries:

pip install python-bitcoinrpc pycryptodome

2. Create Oracle Service
3. Save the following code as oracle_service.py:

from bitcoinrpc.authproxy import AuthServiceProxy
from Crypto.Hash import SHA256
from Crypto.PublicKey import ECC
from Crypto.Signature import DSS
import json

# Connect to Bitcoin node
bitcoin_rpc = AuthServiceProxy("http://yourusername:yourpassword@127.0.0.1:8332")

def monitor_bitcoin_for_ordinal(ordinal_id):
    transactions = bitcoin_rpc.listtransactions()
    for tx in transactions:
        if ordinal_id in json.dumps(tx):
            return tx
    return None

def validate_ordinal(tx):
    return True  # Implement your validation logic

def generate_zkp(tx):
    hash_obj = SHA256.new(json.dumps(tx).encode('utf-8'))
    key = ECC.generate(curve='P-256')
    signer = DSS.new(key, 'fips-186-3')
    signature = signer.sign(hash_obj)
    return key.public_key().export_key(format='DER'), signature

def send_to_solana(pubkey, signature):
    pass  # Implement data transmission logic

ordinal_id = "specific_ordinal_id"
while True:
    tx = monitor_bitcoin_for_ordinal(ordinal_id)
    if tx and validate_ordinal(tx):
        pubkey, signature = generate_zkp(tx)
        send_to_solana(pubkey, signature)


Solana Smart Contract

1. Install Solana CLI:

sh -c "$(curl -sSfL https://release.solana.com/v1.8.0/install)"

2. Install Anchor Framework:

cargo install --git https://github.com/project-serum/anchor --tag v0.19.0 anchor-cli --locked

3. Create Anchor Project:

anchor init cross_chain_oracle
cd cross_chain_oracle

4. Define Smart Contract:
Edit the lib.rs file in programs/cross_chain_oracle/src:

use anchor_lang::prelude::*;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::Pubkey;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWpmyoMaMdHk4Zr1Dz4V7gAa");

#[program]
pub mod cross_chain_oracle {
    use super::*;

    pub fn verify_and_update(ctx: Context<VerifyAndUpdate>, pubkey: Vec<u8>, signature: Vec<u8>, tx_hash: Vec<u8>) -> ProgramResult {
        let pubkey = Pubkey::new(&pubkey);
        let signature = Signature::new(&signature);
        let tx_hash = sha256::Hash::hash(&tx_hash);
        require!(pubkey.verify(&tx_hash, &signature), CustomError::InvalidZKP);

        let nft = &mut ctx.accounts.nft;
        nft.metadata = "New metadata based on verified ordinal".to_string();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyAndUpdate<'info> {
    #[account(mut)]
    pub nft: Account<'info, NFT>,
}

#[account]
pub struct NFT {
    pub metadata: String,
}

#[error]
pub enum CustomError {
    #[msg("Invalid Zero-Knowledge Proof")]
    InvalidZKP,
}


5. Build and Deploy Smart Contract:

anchor build
anchor deploy


Client Interaction

1. Create Client Script:
   
Save the following code as client.js:

const anchor = require('@project-serum/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');

async function main() {
    anchor.setProvider(anchor.AnchorProvider.env());
    const programId = new anchor.web3.PublicKey('Fg6PaFpoGXkYsidMpWxqSWpmyoMaMdHk4Zr1Dz4V7gAa');
    const program = new anchor.Program(idl, programId);
    const nftPublicKey = new PublicKey('Your NFT PublicKey Here');
    const response = await fetch('http://localhost:5000/get_proof');
    const { pubkey, signature, tx_hash } = await response.json();

    const tx = await program.rpc.verifyAndUpdate(pubkey, signature, tx_hash, {
        accounts: {
            nft: nftPublicKey,
        },
    });

    console.log('Transaction signature', tx);
}

main().catch(err => {
    console.error(err);
});

Testing.

1. Deploy the Oracle Service:

python oracle_service.py

2. Run the Client Script:

node client.js
License
This project is licensed under the MIT License.




