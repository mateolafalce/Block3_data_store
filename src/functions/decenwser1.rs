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
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::Decenwser {
            decenwser,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::Decenwser {})
        .send()?;
    let account: DecenwserAccount = program.account(decenwser)?;
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
    Ok(())
}