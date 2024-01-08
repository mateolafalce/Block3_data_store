use crate::{state::accounts::*, utils::constants::MAX_CONTENT};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn modify_data_(ctx: Context<ModifyData>, new_content: String) -> Result<()> {
    let pda_as_arg: Pubkey = ctx.accounts.data_store.key();
    let authority: Pubkey = ctx.accounts.data_store.authority.key();
    let signer: Pubkey = ctx.accounts.signer.key();
    let data_store: &mut Account<DataStore> = &mut ctx.accounts.data_store;
    let (pda, _bump) = Pubkey::find_program_address(&[&signer.to_bytes()], ctx.program_id);
    // valid size
    require_gte!(MAX_CONTENT, new_content.len());
    require_keys_eq!(signer, authority);
    require_keys_eq!(pda, pda_as_arg);
    data_store.set_content(new_content);
    Ok(())
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct ModifyData<'info> {
    #[account(mut, seeds = [&signer.key().to_bytes()], bump = data_store.bump_original,realloc = DataStore::SIZE + content.len(), realloc::payer = signer, realloc::zero = false,)]
    pub data_store: Account<'info, DataStore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
