use anchor_lang::prelude::*;

use crate::constants::{
  DISCRIMINATOR, PUBKEY_SIZE, U8_SIZE, U16_SIZE, U64_SIZE
};

#[account]
pub struct Config {
  pub bump: u8,
  pub admin: Pubkey,
  pub operator: Pubkey,
  pub initial_deposit_amount: u64,
  pub protocol_fee_percentage: u16,
  pub tip_percentage: u16, // Tip percentage for operations
}

impl Config {
  // size for config account
  // 8 + 32 + 32 + 8 + 2 + 2 + 1 = 85
  pub const LEN: usize = DISCRIMINATOR
    + U8_SIZE
    + PUBKEY_SIZE * 2
    + U64_SIZE
    + U16_SIZE * 2;

    pub fn init(
      &mut self,
      bump: u8,
      admin: Pubkey,
      operator: Pubkey,
      protocol_fee_percentage: u16,
      tip_percentage: u16,
  ) -> Result<()> {
      self.bump = bump;
      self.admin = admin;
      self.operator = operator;
      self.initial_deposit_amount = 1_000_000_000; // 1 SOL
      self.protocol_fee_percentage = protocol_fee_percentage;
      self.tip_percentage = tip_percentage;

      Ok(())
  }

  pub fn set_admin(&mut self, admin: Pubkey) -> Result<()> {
      self.admin = admin;

      Ok(())
  }

  pub fn set_operator(&mut self, operator: Pubkey) -> Result<()> {
      self.operator = operator;

      Ok(())
  }

  pub fn set_initial_deposit_amount(&mut self, initial_deposit_amount: u64) -> Result<()> {
      self.initial_deposit_amount = initial_deposit_amount;

      Ok(())
  }

  pub fn set_fee(&mut self, protocol_fee_percentage: u16, tip_percentage: u16) -> Result<()> {
      self.protocol_fee_percentage = protocol_fee_percentage;
      self.tip_percentage = tip_percentage;

      Ok(())
  }
}