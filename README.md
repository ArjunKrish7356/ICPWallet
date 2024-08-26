# ICP Token Wallet

This project implements a basic token wallet for the Internet Computer Protocol (ICP) blockchain using Rust. The wallet supports sending and receiving IRCRC2 tokens and displaying token balances.

## Setup Instructions

1. Install Rust and Cargo (the Rust package manager):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install the DFINITY Canister SDK:
   ```
   sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
   ```

3. Clone this repository:
   ```
   git clone https://github.com/your-username/icp-token-wallet.git
   cd icp-token-wallet
   ```

4. Build the project:
   ```
   cargo build
   ```

5. Deploy to a local ICP test network:
   ```
   dfx start --background
   dfx deploy
   ```

## Testing Instructions

1. Run the unit tests:
   ```
   cargo test
   ```

2. For manual testing, you can use the `dfx` command-line tool to interact with the deployed canister:

   - To send tokens:
     ```
     dfx canister call token_wallet send_tokens '(principal "receiver-principal-id", 100)'
     ```

   - To check balance:
     ```
     dfx canister call token_wallet get_balance '(principal "account-principal-id")'
     ```
     
