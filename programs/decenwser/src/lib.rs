use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{create_data_store::create_data_store_, modify_data::modify_data_};

declare_id!("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN");

#[program]
pub mod data_store {
    use super::*;

    pub fn create_data_store(ctx: Context<CreateDataStore>, content: String) -> Result<()> {
        create_data_store_(ctx, content)
    }

    pub fn modify_data(ctx: Context<ModifyData>, new_content: String) -> Result<()> {
        modify_data_(ctx, new_content)
    }
}
