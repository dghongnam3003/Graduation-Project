use anchor_lang::prelude::*;

use crate::{constants::{CONFIG_SEED, PERCENTAGE_DENOMINATOR}, state::Config, errors::CustomError};

#[derive(Accounts)]
pub struct UpdateFee<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,

  #[account(
    mut,
    seeds = [CONFIG_SEED],
    bump = config.bump,
    has_one = admin,
  )]
  pub config: Account<'info, Config>,
}

pub fn update_fee(ctx: Context<UpdateFee>, new_protocol_fee_percentage: u16, new_tip_percentage: u16) -> Result<()> {
  let config = &mut ctx.accounts.config;

  require!(new_protocol_fee_percentage <= PERCENTAGE_DENOMINATOR, CustomError::InvalidProtocolFeePercentage);
  require!(new_tip_percentage <= PERCENTAGE_DENOMINATOR, CustomError::InvalidTipPercentage);

  config.protocol_fee_percentage = new_protocol_fee_percentage;
  config.tip_percentage = new_tip_percentage;

  Ok(())
}