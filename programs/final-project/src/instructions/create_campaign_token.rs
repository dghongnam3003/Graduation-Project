use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};
use pumpdotfunn_sdk::cpi::{accounts::Create, create};

use crate::{
  constants::{CAMPAIGN_SEED, CONFIG_SEED, PERCENTAGE_DENOMINATOR, TREASURY_SEED},
  errors::CustomError,
  state::{ Campaign, Config, Treasury },
  utils::{calc_out_token_amount, withdraw_sol},
  events::CreatedCampaignTokenEvent,
};

#[derive(Accounts)]
pub struct CreateCampaignToken<'info> {
  #[account(mut)]
  pub operator: Signer<'info>,

  #[account(
    seeds=[CONFIG_SEED],
    bump = config.bump,
    has_one = operator,
  )]
  pub config: Account<'info, Config>,

  #[account(
    mut,
    seeds = [TREASURY_SEED],
    bump = treasury.bump,
  )]
  pub treasury: Account<'info, Treasury>,

  /// CHECK: creator
  #[account()]
  pub creator: UncheckedAccount<'info>,

  #[account(
    mut,
    seeds = [CAMPAIGN_SEED, creator.key.as_ref(), campaign_account.index.to_le_bytes().as_ref()],
    bump = campaign_account.bump,
    has_one = creator,
  )]
  pub campaign_account: Account<'info, Campaign>,

  /// CHECK: Mint
  #[account(mut)]
  pub mint: Signer<'info>,

  /// CHECK: pump.fun mint authority,
  #[account()]
  pub pump_fun_mint_authority: UncheckedAccount<'info>,

  /// CHECK: pump.fun bonding curve,
  #[account(mut)]
  pub pump_fun_bonding_curve: UncheckedAccount<'info>,

  /// CHECK: pump.fun associated bonding curve,
  #[account(mut)]
  pub pump_fun_associated_bonding_curve: UncheckedAccount<'info>,

  /// CHECK: pump.fun global,
  #[account()]
  pub pump_fun_global: UncheckedAccount<'info>,

  /// CHECK: pump.fun event authority,
  #[account()]
  pub pump_fun_event_authority: UncheckedAccount<'info>,

  /// CHECK: pump.fun program,
  #[account()]
  pub pump_fun_program: AccountInfo<'info>,

  /// CHECK: pump.fun token Metadata,
  #[account(mut)]
  pub metadata: UncheckedAccount<'info>,

  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,

  /// CHECK: metaplex metadata program,
  #[account()]
  pub metaplex_metadata_program: UncheckedAccount<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn create_campaign_token(ctx: Context<CreateCampaignToken>, slippage: u16) -> Result<()> {
  let config = &ctx.accounts.config;
  let campaign_account = &mut ctx.accounts.campaign_account;
  let rent = &ctx.accounts.rent;

  require!(
    campaign_account.mint == Pubkey::default(),
    CustomError::TokenAlreadyCreated
  );

  let campaign_balance = campaign_account.get_lamports();
  let minimum_rent_exemption = rent.minimum_balance(Campaign::LEN);
  let deposited_balance = campaign_balance.checked_sub(minimum_rent_exemption).unwrap();

  require!(
    deposited_balance >= campaign_account.donation_goal,
    CustomError::CampaignDonationGoalNotReached
  );

  require!(
    !campaign_account.is_claimed_fund,
    CustomError::CampaignAlreadyClaimed
  );

  let create_token_cpi_context = CpiContext::new(
    ctx.accounts.pump_fun_program.to_account_info(),
    Create {
      mint: ctx.accounts.mint.to_account_info(),
      mint_authority: ctx.accounts.pump_fun_mint_authority.to_account_info(),
      bonding_curve: ctx.accounts.pump_fun_bonding_curve.to_account_info(),
      associated_bonding_curve: ctx.accounts.pump_fun_associated_bonding_curve.to_account_info(),
      global: ctx.accounts.pump_fun_global.to_account_info(),
      mpl_token_metadata: ctx.accounts.metaplex_metadata_program.to_account_info(),
      metadata: ctx.accounts.metadata.to_account_info(),
      user: ctx.accounts.operator.to_account_info(),
      system_program: ctx.accounts.system_program.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(),
      associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
      rent: ctx.accounts.rent.to_account_info(),
      event_authority: ctx.accounts.pump_fun_event_authority.to_account_info(),
      program: ctx.accounts.pump_fun_program.to_account_info(),
    },
  );

  create(
    create_token_cpi_context,
    campaign_account.name.clone(),
    campaign_account.symbol.clone(),
    campaign_account.uri.clone(),
    ctx.accounts.creator.key(),
  )?;

  campaign_account.set_mint(ctx.accounts.mint.key())?;

  let now = Clock::get().unwrap().unix_timestamp;

  // Emit event ngay sau khi tạo token thành công
  emit!(CreatedCampaignTokenEvent {
    creator: ctx.accounts.creator.key(),
    campaign_index: campaign_account.index,
    mint: ctx.accounts.mint.key(),
    bought_amount: 0, // Không buy token nữa, set về 0
    timestamp: now,
  });

  let fee = deposited_balance
      .checked_mul(config.protocol_fee_percentage as u64)
      .unwrap()
      .checked_div(PERCENTAGE_DENOMINATOR as u64)
      .unwrap();

  let max_sol_cost = deposited_balance.checked_sub(fee).unwrap();
  let token_amount = calc_out_token_amount(
    max_sol_cost,
    slippage
  );

 campaign_account.buy_token(token_amount)?;

  withdraw_sol(
    &campaign_account.to_account_info(),
    &ctx.accounts.treasury.to_account_info(),
    fee,
  )?;

  withdraw_sol(
    &campaign_account.to_account_info(),
    &ctx.accounts.operator.to_account_info(),
    deposited_balance.checked_sub(fee).unwrap(),
  )?;

  Ok(())
}