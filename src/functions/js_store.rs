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
use decenwser::state::JS;

pub fn js_store(
    program: &Program,
    js: String,
    web_name: String
) -> Result<()> {
    let (main_account, _bump) = Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let account: DecenwserAccount = program.account(decenwser)?;
    let (js_store, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::JsStore {
            main_account,
            decenwser,
            js_store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::JsStore {
            js
        })
        .send()?;
    let js_account: JS = program.account(js_store)?;
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