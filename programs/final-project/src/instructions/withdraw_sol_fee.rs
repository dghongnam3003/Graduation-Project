use anchor_lang::prelude::*;

use crate::{constants::{CONFIG_SEED, TREASURY_SEED}, state::{Config, Treasury}, errors::CustomError, utils::withdraw_sol};

#[derive(Accounts)]
pub struct WithdrawSolFee<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,

  #[account(
    seeds = [CONFIG_SEED],
    bump = config.bump,
    has_one = admin,
  )]
  pub config: Account<'info, Config>,

  #[account(
    mut,
    seeds = [TREASURY_SEED],
    bump = treasury.bump,
  )]
  pub treasury: Account<'info, Treasury>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn withdraw_sol_fee(ctx: Context<WithdrawSolFee>, amount: u64) -> Result<()> {
  let rent = &ctx.accounts.rent;
  let treasury_balance = ctx.accounts.treasury.get_lamports();
  let minimum_rent_exemption = rent.minimum_balance(Treasury::LEN);
  let available_balance = treasury_balance - minimum_rent_exemption;

  let withdraw_amount = if amount > available_balance {
    available_balance
  } else {
    amount
  };

  withdraw_sol(
    &ctx.accounts.treasury.to_account_info(),
    &ctx.accounts.admin,
    withdraw_amount,
  )?;

  Ok(())
}

