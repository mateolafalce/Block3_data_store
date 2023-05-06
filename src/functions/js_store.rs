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
use decenwser::state::{
    DecenwserAccount,
    JS
};

pub fn js_store(
    program: &Program,
    js: String,
    web_name: String
) -> Result<()> {
    let (main_account, _bump) = Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    // Generate a program-derived address for the Decenwser program account.
    let (decenwser, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    // Get the Decenwser account from the program's accounts.
    let account: DecenwserAccount = program.account(decenwser)?;
    // Generate a program-derived address for the JavaScript store account.
    let (js_store, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    // Send a transaction to the Decenwser program to store the JavaScript code.
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::JsStore {
            main_account,
            decenwser,
            js_store,
            signer: program.payer(), // The payer of the transaction.
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::JsStore {
            js // The JavaScript code to be stored.
        })
        .send()?;
    // Get the JavaScript store account from the program's accounts.
    let js_account: JS = program.account(js_store)?;
    // Print information about the transaction and the stored JavaScript code.
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", js_store);
    println!("------------------------------------------------------------");
    println!("Js data: {}", js_account.js);
    println!("------------------------------------------------------------");
    println!("Total updates: {}", account.total_updates);
    println!("------------------------------------------------------------");
    Ok(())
}
