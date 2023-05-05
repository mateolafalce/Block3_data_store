use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn js_store(
    ctx: Context<JsStore>,
    js: String,
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let js_store: &mut Account<JS> = &mut ctx.accounts.js_store;
    require!(8 + ctx.accounts.main_account.len < 9995, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    js_store.js = js;
    // Add the current total updates to the main account's JS array
    main_account.js.push(ctx.accounts.decenwser.total_updates);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    // Increment the total updates in the Decenwser account
    decenwser.total_updates += 1;
    // Increase the main account length by 8 bytes
    main_account.len += 8;
    Ok(())
}

#[derive(Accounts)]
pub struct JsStore<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        // Reallocate the main account with extra space for the new JS string and array element
        realloc = 8 + main_account.len as usize + 8,
        // Use the signer account to pay for the reallocation
        realloc::payer = signer,
        // Don't zero out the reallocated space
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut,seeds = [b"Decenwser"],bump = decenwser.bump_original)]
    pub decenwser: Account<'info, DecenwserAccount>,
    // Initialize a new JS account with space for the JS string and array element
    #[account(init, seeds = [&decenwser.total_updates.to_le_bytes()], bump, payer = signer, space = JS::SIZE + 8)]
    pub js_store: Account<'info, JS>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
