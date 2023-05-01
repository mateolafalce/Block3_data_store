use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn html_store(
    ctx: Context<HtmlStore>,
    html: String,
) -> Result<()> {
    require!(
        ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(),
        ErrorCode::AuthorityError
    ); // Checks if the main account's authority key matches the signer's key
    let html_store: &mut Account<HTML> = &mut ctx.accounts.html_store;
    require!(
        8 + ctx.accounts.main_account.len < 9995,
        ErrorCode::TooLong
    ); // Checks if storing the HTML would exceed the maximum length
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    html_store.html = html; // Stores the HTML string in the HtmlStore account
    main_account.html.push(ctx.accounts.decenwser.total_updates);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    decenwser.total_updates += 1; // Increments the total number of updates in the Decenwser account
    main_account.len += 8; // Increments the main account's length by 8 bytes
    Ok(())
}


// This code is using the `Accounts` procedural macro to define a struct named `HtmlStore`.
// The struct has four public fields, all of which are accounts:
#[derive(Accounts)]
pub struct HtmlStore<'info> {
    pub main_account: Account<'info, MainAccount>,
    pub decenwser: Account<'info, DecenwserAccount>,
    pub html_store: Account<'info, HTML>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
