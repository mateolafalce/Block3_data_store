use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;

pub fn decenwser(
    ctx: Context<Decenwser>
) -> Result<()> {
    // Find the program address based on the program ID and a fixed string.
    let (_pda, bump) = Pubkey::find_program_address(&[b"Decenwser"], ctx.program_id);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    // Set some fields of the DecenwserAccount.
    decenwser.pages_online = 0;
    decenwser.total_updates = 0;
    decenwser.bump_original = bump;
    Ok(()) // Return success.
}

#[derive(Accounts)]
pub struct Decenwser<'info> {
    // The DecenwserAccount being initialized.
    #[account(init, seeds = [b"Decenwser"], bump, payer = signer, space = DecenwserAccount::SIZE + 8)]
    pub decenwser: Account<'info, DecenwserAccount>,
    #[account(mut)] // The signer of the transaction.
    pub signer: Signer<'info>,
    // The system program, which provides basic program functionality.
    pub system_program: Program<'info, System>,
}
