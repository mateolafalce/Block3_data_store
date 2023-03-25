use anchor_client::{
    anchor_lang,
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use decenwser::state::MainAccount;

pub fn main_account(
    program: &Program,
    web_name: String
) -> Result<()> {
    let (main_account, _bump) = Pubkey::find_program_address(&[&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::MainAccountStruct {
            main_account,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::MainAccount {
            web_name
        })
        .send()?;
    let account: MainAccount = program.account(main_account)?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", main_account);
    println!("------------------------------------------------------------");
    println!("Bump original: {}", account.bump_original);
    println!("------------------------------------------------------------");
    println!("Web name: {}", account.web_name);
    println!("------------------------------------------------------------");
    println!("Authority: {}", account.authority);
    println!("------------------------------------------------------------");
    println!("Html store: {:?}", account.html);
    println!("------------------------------------------------------------");
    println!("Css store: {:?}", account.css);
    println!("------------------------------------------------------------");
    println!("Js store: {:?}", account.js);
    println!("------------------------------------------------------------");
    println!("Space: {}", account.len);
    println!("------------------------------------------------------------");
    Ok(())
}