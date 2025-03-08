use anchor_lang::prelude::*;

use crate::constants::{
  BOOL_SIZE, DISCRIMINATOR, I64_SIZE, PUBKEY_SIZE, STRING_MAX_100_CHAR_SIZE,
  STRING_MAX_10_CHAR_SIZE, STRING_MAX_32_CHAR_SIZE, U64_SIZE, U8_SIZE
};

#[account]
pub struct Campaign {
  pub bump: u8,
  pub creator: Pubkey,
  pub mint: Pubkey,
  pub index: u64,
  pub name: String,
  pub symbol: String,
  pub uri: String,
  pub deposit_deadline: i64,
  pub trade_deadline: i64,
  pub donation_goal: u64,
  pub total_token_bought: u64,
  pub claimable_amount: u64,
  pub total_claimed: u64,
  pub is_claimed_fund: bool,
  pub is_sell_all: bool,
}

impl Campaign {
  // size for campaign account
  // 8 + 1 + 32 + 32 + 8 + (4 + 32) + (4 + 10) + (4 + 100) + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 = 285
  pub const LEN: usize = DISCRIMINATOR
    + U8_SIZE
    + PUBKEY_SIZE * 2
    + STRING_MAX_32_CHAR_SIZE
    + STRING_MAX_10_CHAR_SIZE
    + STRING_MAX_100_CHAR_SIZE
    + I64_SIZE * 2
    + U64_SIZE * 5
    + BOOL_SIZE * 2;

  #[allow(clippy::too_many_arguments)]
  pub fn init(
    &mut self,
    bump: u8,
    creator: Pubkey,
    index: u64,
    name: String,
    symbol: String,
    uri: String,
    deposit_deadline: i64,
    trade_deadline: i64,
    donation_goal: u64,
  ) -> Result<()> {
    self.bump = bump;
    self.creator = creator;
    self.mint = Pubkey::default();
    self.index = index;
    self.name = name;
    self.symbol = symbol;
    self.uri = uri;
    self.deposit_deadline = deposit_deadline;
    self.trade_deadline = trade_deadline;
    self.donation_goal = donation_goal;
    self.total_token_bought = 0;
    self.claimable_amount = 0;
    self.total_claimed = 0;
    self.is_claimed_fund = false;
    self.is_sell_all = false;

    Ok(())
  }

  pub fn set_mint(&mut self, mint: Pubkey) -> Result<()> {
    self.mint = mint;

    Ok(())
  }

  pub fn claim_fund(&mut self) -> Result<()> {
    self.is_claimed_fund = true;

    Ok(())
  }

  pub fn is_sell_all(&mut self) -> Result<()> {
    self.is_sell_all = true;

    Ok(())
  }

  pub fn sell_all_token(&mut self) -> Result<()> {
    self.is_sell_all = true;

    Ok(())
  }

  pub fn buy_token(&mut self, amount: u64) -> Result<()> {
    self.total_token_bought = self.total_token_bought.checked_add(amount).unwrap();

    Ok(())
  }

  pub fn increase_claimable_amount(&mut self, amount: u64) -> Result<()> {
    self.claimable_amount = self.claimable_amount.checked_add(amount).unwrap();

    Ok(())
  }

  pub fn increase_total_claimed(&mut self, amount: u64) -> Result<()> {
    self.claimable_amount = self.claimable_amount.checked_sub(amount).unwrap();
    self.total_claimed = self.total_claimed.checked_add(amount).unwrap();

    Ok(())
  }
}