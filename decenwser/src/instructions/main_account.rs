use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn main_account(
    ctx: Context<MainAccountStruct>,
    web_name: String
) -> Result<()> {
    // Check if the web_name is within the allowed length.
    require!(web_name.len() <= 32, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    // Generate a program derived address and bump value from the web_name.
    let (_pda, bump) = Pubkey::find_program_address(&[
        &anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()
        ],
        ctx.program_id
    );
    main_account.bump_original = bump;
    main_account.web_name = web_name;
    main_account.authority = ctx.accounts.signer.key();
    // Set the html, css and js fields of the main account to empty arrays.
    main_account.html = [].to_vec();
    main_account.css = [].to_vec();
    main_account.js = [].to_vec();
    main_account.len = 107;
    Ok(())
}

#[derive(Accounts)]
#[instruction(web_name: String)]
pub struct MainAccountStruct<'info> {
    // Define the main account as an initialized account with the given seeds, bump, payer and space.
    #[account(init, seeds = [
        &anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()],
        bump,
        payer = signer,
        space = MainAccount::SIZE + 8
    )]
    pub main_account: Account<'info, MainAccount>,
    // Define a mutable reference to the signer account.
    #[account(mut)]
    pub signer: Signer<'info>,
    // Define a reference to the System program account.
    pub system_program: Program<'info, System>,
}
