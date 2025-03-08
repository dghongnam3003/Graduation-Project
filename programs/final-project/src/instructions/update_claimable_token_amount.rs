use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::{
  constants::{CONFIG_SEED, CAMPAIGN_SEED},
  errors::CustomError,
  state::{Campaign, Config},
  events::ClaimableTokenAmountUpdatedEvent,
};

#[derive(Accounts)]
pub struct UpdateClaimableTokenAmount<'info> {
  #[account(mut)]
  pub operator: Signer<'info>,

  #[account(
    seeds = [CONFIG_SEED],
    bump = config.bump,
    has_one = operator,
  )]
  pub config: Account<'info, Config>,

  /// CHECK: Creator account
  #[account()]
  pub creator: UncheckedAccount<'info>,

  #[account(
    mut,
    seeds = [CAMPAIGN_SEED, creator.key().as_ref(), campaign_account.index.to_le_bytes().as_ref()],
    bump = campaign_account.bump,
    has_one = creator,
    has_one = mint,
  )]
  pub campaign_account: Account<'info, Campaign>,

  /// CHECK: mint
  #[account()]
  pub mint: UncheckedAccount<'info>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn update_claimable_token_amount(
  ctx: Context<UpdateClaimableTokenAmount>,
  new_amount: u64,
) -> Result<()> {
  let campaign_account = &mut ctx.accounts.campaign_account;

  require!(
    campaign_account.total_claimed.checked_add(new_amount).unwrap() <= campaign_account.total_token_bought,
    CustomError::ExceedsTotalTokenBought
  );

  campaign_account.increase_claimable_amount(new_amount)?;

  let now = Clock::get()?.unix_timestamp;

  emit!(ClaimableTokenAmountUpdatedEvent {
    creator: ctx.accounts.creator.key(),
    campaign_index: campaign_account.index,
    mint: ctx.accounts.mint.key(),
    new_amount,
    timestamp: now,
  });

  Ok(())
}
