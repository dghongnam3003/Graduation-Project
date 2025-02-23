use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR, U8_SIZE};

#[account]
pub struct Treasury {
    pub bump: u8,
}

impl Treasury {
    // 8 + 1 = 9
    pub const LEN: usize = DISCRIMINATOR + U8_SIZE;

    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;

        Ok(())
    }
}
