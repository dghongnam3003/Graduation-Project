use anchor_lang::prelude::*;

use crate::{constants::CONFIG_SEED, state::Config, errors::CustomError};

#[derive(Accounts)]
pub struct UpdateOperator<'info> {
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

pub fn update_operator(ctx: Context<UpdateOperator>, new_operator: Pubkey) -> Result<()> {
  let config = &mut ctx.accounts.config;

  require!(new_operator != config.operator, CustomError::OperatorIsSame);

  config.operator = new_operator;

  Ok(())
}