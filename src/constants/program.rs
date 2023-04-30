use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
    Client, Cluster, Program
};
use anyhow::{Ok, Result};
use std::rc::Rc;
use std::str::FromStr;
use crate::constants::program_id;

pub fn program() -> Result<Program> {
    // Connect to the Solana Devnet cluster and authenticate with a keypair
    let program: Program = Client::new(
        Cluster::Devnet, // Specify the Devnet cluster
        Rc::new(
            read_keypair_file(&*shellexpand::tilde(
                "C:/Users/Mateo/.config/solana/id.json",
            ))
            .expect("Example requires a keypair file"), // Load the keypair from a file
        ),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap()); // Retrieve the program ID from a constant
    Ok(program)
}
