use crate::utils::constants::ANCHOR_BUFFER;
use anchor_lang::prelude::*;

#[account]
pub struct DataStore {
    pub bump_original: u8, // 1
    pub authority: Pubkey, // 32
    pub content: String,   // 4 + 32
}

impl DataStore {
    pub const SIZE: usize = 1 + 32 + 4 + 32 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }
}
