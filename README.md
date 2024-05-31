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
