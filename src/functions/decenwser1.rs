use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use decenwser::state::DecenwserAccount;

pub fn decenwser1(
    program: &Program
) -> Result<()> {
    // Find the program address for the Decenwser program using the program ID and a byte string
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::Decenwser {
            decenwser, // PDA address for the Decenwser program
            signer: program.payer(), // Account used to pay for the transaction
            system_program: system_program::ID, // System program account
        })
        .args(decenwser::instruction::Decenwser {})
        .send()?; // Send the transaction and receive the resulting signature
    let account: DecenwserAccount = program.account(decenwser)?;
    // Print information about the transaction and the Decenwser account
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", decenwser);
    println!("------------------------------------------------------------");
    println!("Bump: {}", account.bump_original);
    println!("------------------------------------------------------------");
    println!("Total updates: {}", account.total_updates);
    println!("------------------------------------------------------------");
    println!("Pages online: {}", account.pages_online);
    println!("------------------------------------------------------------");
    Ok(()) // Return Ok if the function was executed successfully
}
