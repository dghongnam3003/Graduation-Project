use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::{CAMPAIGN_SEED, CONFIG_SEED, PERCENTAGE_DENOMINATOR, TREASURY_SEED},
    errors::CustomError,
    state::{Campaign, Config, Treasury},
    utils::withdraw_token_from_campaign,
    events::ClaimedTokenEvent,
};

#[derive(Accounts)]
pub struct ClaimCampaignToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
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
        seeds = [CAMPAIGN_SEED, creator.key.as_ref(), campaign_account.index.to_le_bytes().as_ref()],
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
    pub associated_campaign: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = treasury,
    )]
    pub associated_treasury: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub associated_creator: Account<'info, TokenAccount>,
    
    /// CHECK: mint
    #[account()]
    pub mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

}

pub fn claim_campaign_token(ctx: Context<ClaimCampaignToken>) -> Result<()> {

    let config = &ctx.accounts.config;
    let campaign_account = &mut ctx.accounts.campaign_account;

    let claim_amount = campaign_account.claimable_amount;
    require!(
        campaign_account
            .total_claimed
            .checked_add(claim_amount)
            .unwrap()
            <= campaign_account.total_token_bought,
        CustomError::ExceedsTotalTokenBought
    );

    campaign_account.increase_total_claimed(claim_amount);

    let tip_amount = claim_amount
        .checked_mul(config.tip_percentage as u64)
        .unwrap()
        .checked_div(PERCENTAGE_DENOMINATOR as u64)
        .unwrap();

    withdraw_token_from_campaign(
        &ctx.accounts.associated_campaign,
        &ctx.accounts.associated_creator,
        claim_amount.checked_sub(tip_amount).unwrap(),
        &ctx.accounts.token_program,
        &campaign_account.to_account_info(),
        campaign_account.bump,
        campaign_account.creator,
        campaign_account.index,
    )?;

    withdraw_token_from_campaign(
        &ctx.accounts.associated_campaign,
        &ctx.accounts.associated_treasury,
        tip_amount,
        &ctx.accounts.token_program,
        &campaign_account.to_account_info(),
        campaign_account.bump,
        campaign_account.creator,
        campaign_account.index,
    )?;

    let now = Clock::get()?.unix_timestamp;

    emit!(ClaimedTokenEvent {
        creator: ctx.accounts.creator.key(),
        campaign_index: campaign_account.index,
        claimed_amount: claim_amount,
        mint: ctx.accounts.mint.key(),
        timestamp: now,
    });
    
    Ok(())
}

