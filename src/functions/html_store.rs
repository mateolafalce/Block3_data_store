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
use decenwser::state::HTML;

pub fn html_store(
    program: &Program,
    html: String,
    web_name: String
) -> Result<()> {
    // Find the program address for the main account using the web_name as input
    let (main_account, _bump) = Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    // Find the program address for the Decenwser program
    let (decenwser, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let account: DecenwserAccount = program.account(decenwser)?;
    let (html_store, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    // Send a transaction to the Decenwser program, creating an HTML account and storing the provided HTML data in it
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::HtmlStore {
            main_account,
            decenwser,
            html_store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::HtmlStore {
            html
        })
        .send()?;
    let html_account: HTML = program.account(html_store)?;
    // Print out some information about the transaction and account data
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", html_store);
    println!("------------------------------------------------------------");
    println!("Html data: {}", html_account.html);
    println!("------------------------------------------------------------");
    println!("Total updates: {}", account.total_updates);
    println!("------------------------------------------------------------");
    Ok(()) // Return a success result
}
