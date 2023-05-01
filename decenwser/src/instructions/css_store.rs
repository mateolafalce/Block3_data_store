use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn css_store(
    ctx: Context<CssStore>,
    css: String,
) -> Result<()> {
    // Ensure that the authority key of `main_account` matches the signer key, otherwise return an error.
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let css_store: &mut Account<CSS> = &mut ctx.accounts.css_store;
    // Ensure that the length of `main_account` does not exceed the maximum allowed length.
    require!(8 + ctx.accounts.main_account.len < 9995, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    css_store.css = css;
    // Push the current `total_updates` value of `decenwser` to the `css` field of `main_account`.
    main_account.css.push(ctx.accounts.decenwser.total_updates);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    decenwser.total_updates += 1;
    main_account.len += 8;
    Ok(())
}

#[derive(Accounts)]
pub struct CssStore<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize + 8, // Resize account to fit data and padding
        realloc::payer = signer, // Charge resize to `signer` account
        realloc::zero = false, // Don't zero out unused space in the account
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut,seeds = [b"Decenwser"],bump = decenwser.bump_original)]
    pub decenwser: Account<'info, DecenwserAccount>,
    #[account(init, seeds = [&decenwser.total_updates.to_le_bytes()], bump, payer = signer, space = CSS::SIZE + 8)]
    pub css_store: Account<'info, CSS>,
    // Define the `signer` field, which is a signer that provides permission to modify the account.
    #[account(mut)]
    pub signer: Signer<'info>,
    // Define the `system_program` field, which is a reference to the Solana system program
    pub system_program: Program<'info, System>,
}
