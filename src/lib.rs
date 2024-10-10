// src/lib.rs
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_program}; // Import system_program
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

// Import the Turbin3 program structures
mod programs;
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_completion() {
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Define our accounts
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        // Create PDA for the prereq account
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"iamma00".to_vec(), // Your GitHub username
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create the transaction to invoke the "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print the transaction output
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
