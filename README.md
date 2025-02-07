
# Libra Wallet

Libra Wallet is a Rust-based cryptocurrency wallet designed to operate on the Aptos blockchain network. It provides essential functionalities such as key management, transaction signing, and interaction with the blockchain.

## Features

-   **Secure Key Management**: Generate and manage cryptographic keys securely.
    
-   **Transaction Signing**: Sign transactions for execution on the Aptos blockchain.
    
-   **Configurable Dependencies**: Uses various libraries for cryptographic operations and blockchain interactions.
    
-   **Command-line Interface**: Built with `clap` for ease of use.
    
-   **Cross-platform Compatibility**: Supports multiple operating systems.
    

## Installation

### Prerequisites

Ensure you have the following installed:

-   Rust (latest stable version)
    
-   Cargo package manager
    

### Build and Install

```
# Clone the repository
git clone https://github.com/luong0484/libra-wallet.git
cd libra-wallet

# Build the project
cargo build --release

# Run the wallet
./target/release/libra-wallet
```

## Usage

You can use the command-line interface to interact with the Libra Wallet. Run the following command to see available options:

```
libra-wallet --help
```

## Dependencies

The project relies on the following key dependencies:

-   `dialoguer` (0.10.4): For interactive CLI prompts.
    
-   `clap` (4.2.4): Command-line argument parsing.
    
-   `serde_yaml`, `serde_json`, `serde`: Serialization and deserialization.
    
-   `hex`, `anyhow`: Utility libraries.
    
-   `blst`: Cryptographic library for BLS signatures.
    
-   `zapatos-*`: A suite of tools for blockchain interaction.
    
-   `ol-keys`, `diem-wallet`: Libra and Diem wallet libraries from GitHub.
    

## Configuration

The wallet configuration is managed via YAML files stored in the system's config directory. You can modify these settings to customize wallet behavior.

## License

This project is licensed under the same terms as the Aptos ecosystem. See the LICENSE file for details.
