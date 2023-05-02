use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use decenwser::state::DecenwserAccount;
use decenwser::state::CSS;

pub fn css_store(
    program: &Program,
    css: String,
    web_name: String
) -> Result<()> {
    // Find the PDA for the main account associated with the given web name.
    let (main_account, _bump) = Pubkey::find_program_address(
        &[&hash(web_name.as_bytes()).to_bytes()],
        &program.id()
    );
    // Find the PDA for the Decenwser program.
    let (decenwser, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[b"Decenwser"],
        &program.id()
    );
    // Get the `DecenwserAccount` associated with the Decenwser program.
    let account: DecenwserAccount = program.account(decenwser)?;
    // Find the PDA for the CSS store associated with the current number of updates.
    let (css_store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[&account.total_updates.to_le_bytes()], // Seed for the PDA
        &program.id() // Program ID
    );
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::CssStore {
            main_account, // Main account associated with the given web name
            decenwser, // PDA for the Decenwser program
            css_store, // PDA for the CSS store associated with the current number of updates
            signer: program.payer(), // Signer of the transaction
            system_program: system_program::ID, // ID of the system program
        })
        .args(decenwser::instruction::CssStore {
            css // CSS data to store in the CSS store account
        })
        .send()?;
    let css_account: CSS = program.account(css_store)?; // Get the `CSS` account associated with the CSS store PDA.
    // Log some information to the console.
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", css_store);
    println!("------------------------------------------------------------");
    println!("Css data: {}", css_account.css);
    println!("------------------------------------------------------------");
    println!("Total updates: {}", account.total_updates);
    println!("------------------------------------------------------------");
    Ok(()) // Return `Ok(())` to indicate success.
}
