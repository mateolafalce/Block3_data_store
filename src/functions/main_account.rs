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
    // Find the program address using the provided web name and the program ID
    let (main_account, _bump) = Pubkey::find_program_address(
        // Hash the web_name string using the Solana SDK's hash function and convert to bytes
        &[&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()],
        // Use the ID of the current program
        &program.id()
    );
    // Create and send a transaction to create a MainAccount using the provided data
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
    // Retrieve the MainAccount data from the blockchain
    let account: MainAccount = program.account(main_account)?;
    // Print out the transaction signature, PDA, and MainAccount data
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
    Ok(()) // Return Ok to indicate success
}
