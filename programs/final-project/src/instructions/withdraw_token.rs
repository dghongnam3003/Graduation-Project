use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::{CONFIG_SEED, TREASURY_SEED}, state::{Config, Treasury}, errors::CustomError, utils::withdraw_token_from_treasury};

#[derive(Accounts)]
pub struct WithdrawCampaignToken<'info> {
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

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = treasury,
  )]
  pub associated_treasury: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = admin,
  )]
  pub associated_admin: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub mint: Account<'info, Mint>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn withdraw_campaign_token(ctx: Context<WithdrawCampaignToken>, amount: u64) -> Result<()> {
  let treasury = &mut ctx.accounts.treasury;
  let associated_treasury = &ctx.accounts.associated_treasury;

  let withdraw_amount = if amount > associated_treasury.amount {
    associated_treasury.amount
  } else {
    amount
  };

  withdraw_token_from_treasury(
    &ctx.accounts.associated_treasury,
    &ctx.accounts.associated_admin,
    withdraw_amount,
    &ctx.accounts.token_program,
    &treasury.to_account_info(),
    treasury.bump,
  )?;

  Ok(())
}
