use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{Token, TokenAccount},
};
use pumpdotfunn_sdk::cpi::{sell, accounts::Sell};

use crate::{
  constants::CAMPAIGN_SEED,
  errors::CustomError,
  state::Campaign,
  utils::withdraw_sol,
  events::SoldCampaignTokenEvent,
};

#[derive(Accounts)]
pub struct SellCampaignToken<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    mut,
    seeds = [CAMPAIGN_SEED, creator.key().as_ref(), campaign_account.index.to_le_bytes().as_ref()],
    bump = campaign_account.bump,
    has_one = creator,
    has_one = mint,
  )]
  pub campaign_account: Account<'info, Campaign>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = campaign_account,
  )]
  pub associated_campaign: Box<Account<'info, TokenAccount>>,

  /// CHECK: mint
  #[account()]
  pub mint: UncheckedAccount<'info>,

  /// CHECK:: pump.fun fee recipient
  #[account()]
  pub pump_fun_fee_recipient: UncheckedAccount<'info>,

  /// CHECK: pump.fun bonding curve
  #[account()]
  pub pump_fun_bonding_curve: UncheckedAccount<'info>,

  /// CHECK: pump.fun associated bonding curve
  #[account()]
  pub pump_fun_associated_bonding_curve: UncheckedAccount<'info>,

  /// CHECK: pump.fun global
  #[account()]
  pub pump_fun_global: UncheckedAccount<'info>,

  /// CHECK: pump.fun event authority
  #[account()]
  pub pump_fun_event_authority: UncheckedAccount<'info>,

  /// CHECK: pump.fun program
  #[account()]
  pub pump_fun_program: AccountInfo<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn sell_campaign_token(ctx: Context<SellCampaignToken>, mint_sol_output: u64) -> Result<()> {
  let campaign_account = &mut ctx.accounts.campaign_account;
  let campaign_account_info = campaign_account.to_account_info();
  let pump_fun_program_id = ctx.accounts.pump_fun_program.to_account_info();
  let associated_campaign = &ctx.accounts.associated_campaign;

  let now = Clock::get()?.unix_timestamp;
  if campaign_account.trade_deadline > 0 {
    require!(
      now > campaign_account.trade_deadline,
      CustomError::CampaignTradeDeadlineNotPassed
    );
  }

  require!(
    !campaign_account.is_sell_all,
    CustomError::CampaignAlreadySoldAllToken
  );

  let campaing_token_balance = associated_campaign.amount;
  campaign_account.sell_all_token()?;
  
  let bump_seed = campaign_account.bump;
  let creator_bytes = campaign_account.creator.as_ref();
  let index_bytes = campaign_account.index.to_le_bytes();
  let signer_seeds: &[&[&[u8]]] = &[&[CAMPAIGN_SEED, creator_bytes, &index_bytes, &[bump_seed]]];

  //pump.fun sell token cpi
  let cpi_context = CpiContext::new(
    pump_fun_program_id,
    Sell {
      global: ctx.accounts.pump_fun_global.to_account_info(),
      fee_recipient: ctx.accounts.pump_fun_fee_recipient.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        bonding_curve: ctx.accounts.pump_fun_bonding_curve.to_account_info(),
        associated_bonding_curve: ctx
          .accounts
          .pump_fun_associated_bonding_curve
          .to_account_info(),
      associated_user: ctx.accounts.associated_campaign.to_account_info(),
      user: campaign_account_info,
      system_program: ctx.accounts.system_program.to_account_info(),
      associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(),
      event_authority: ctx.accounts.pump_fun_event_authority.to_account_info(),
      program: ctx.accounts.pump_fun_program.to_account_info(),
    },
  ).with_signer(signer_seeds);

  sell(cpi_context, campaing_token_balance, mint_sol_output)?;

  let rent = &ctx.accounts.rent;
  let campaign_balance = campaign_account.get_lamports();
  let minimum_rent_exemption = rent.minimum_balance(Campaign::LEN);
  let available_balance = campaign_balance - minimum_rent_exemption;

  withdraw_sol(
    &campaign_account.to_account_info(),
    &ctx.accounts.creator,
    available_balance,
  )?;

  emit!(SoldCampaignTokenEvent {
    creator: campaign_account.creator,
    campaign_index: campaign_account.index,
    mint: ctx.accounts.mint.key(),
    timestamp: now,
  });

  Ok(())
}