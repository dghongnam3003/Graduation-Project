use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR, PUBKEY_SIZE, U64_SIZE, U8_SIZE};

#[account]
pub struct Creator {
    pub bump: u8,
    pub creator: Pubkey,
    pub last_campaign_index: u64,
}

impl Creator {
    pub const LEN: usize = DISCRIMINATOR + U8_SIZE + PUBKEY_SIZE + U64_SIZE;

    pub fn init(&mut self, bump: u8, creator: Pubkey) -> Result<()> {
        self.bump = bump;
        self.creator = creator;
        self.last_campaign_index = 0;

        Ok(())
    }

    pub fn increment_last_campaign_index(&mut self) -> Result<()> {
        self.last_campaign_index = self.last_campaign_index.checked_add(1).unwrap();

        Ok(())
    }
}
