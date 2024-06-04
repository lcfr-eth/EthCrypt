use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "EthCrypt", 
    version = "1.0", 
    about = "Encrypt/Decrypt text & files using ETH keypairs", 
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Test,
    Pubkey,
    Encrypt {
        #[arg(help = "Public key in hex format")]
        public_key_hex: String,
        #[arg(help = "Message to encrypt")]
        message: String,
    },
    Decrypt {
        #[arg(help = "Encrypted message in hex format")]
        encrypted_hex: String,
    },
    EncryptFile {
        #[arg(help = "Public key in hex format")]
        public_key_hex: String,
        #[arg(help = "Input file path")]
        input_file: String,
        #[arg(help = "Output file path")]
        output_file: String,
    },
    DecryptFile {
        #[arg(help = "Input file path")]
        input_file: String,
        #[arg(help = "Output file path")]
        output_file: String,
    },
}