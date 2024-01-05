use crate::{state::accounts::*, utils::constants::MAX_CONTENT};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn create_data_store_(ctx: Context<CreateDataStore>, content: String) -> Result<()> {
    let signer: Pubkey = ctx.accounts.signer.key();
    let data_store: &mut Account<DataStore> = &mut ctx.accounts.data_store;
    let (_pda, bump) = Pubkey::find_program_address(&[&signer.to_bytes()], ctx.program_id);
    require_gte!(MAX_CONTENT, content.len());
    // update state
    data_store.set_bump_original(bump);
    data_store.set_authority(signer);
    data_store.set_content(content);
    Ok(())
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct CreateDataStore<'info> {
    #[account(init, seeds = [&signer.key().to_bytes()],
        bump,
        payer = signer,
        space = DataStore::SIZE
    )]
    pub data_store: Account<'info, DataStore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
