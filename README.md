ICP Token Wallet
This project implements a basic token wallet for the Internet Computer Protocol (ICP) blockchain using Rust. The wallet supports sending and receiving IRCRC2 tokens and displaying token balances.
Setup Instructions

Install Rust and Cargo (the Rust package manager):
Copycurl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install the DFINITY Canister SDK:
Copysh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"

Clone this repository:
Copygit clone https://github.com/your-username/icp-token-wallet.git
cd icp-token-wallet

Build the project:
Copycargo build

Deploy to a local ICP test network:
Copydfx start --background
dfx deploy


Testing Instructions

Run the unit tests:
Copycargo test

For manual testing, you can use the dfx command-line tool to interact with the deployed canister:

To send tokens:
Copydfx canister call token_wallet send_tokens '(principal "receiver-principal-id", 100)'

To check balance:
Copydfx canister call token_wallet get_balance '(p
