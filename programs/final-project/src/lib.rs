use anchor_lang::prelude::*;

declare_id!("CLCBzkzZr1CCRfwmNXmTSYiZxSR1M9HV9CrBEKWFZvgw");

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

#[program]
pub mod final_project {

    use events::CreatedCampaignEvent;

    use super::*;

    pub fn initialize(
        ctx: Context<Init>,
        bump: u8,
        treasury_bump: u8,
        operator: Pubkey,
        protocol_fee_percentage: u16,
        tip_percentage: u16,
    ) -> Result<()> {
        instructions::init(ctx, bump, treasury_bump, operator, protocol_fee_percentage, tip_percentage)
    }

    pub fn initialize_creator(ctx: Context<InitCreator>, bump: u8) -> Result<()> {instructions::init_creator(ctx, bump)}

    pub fn create_campaign(
        ctx: Context<InitCampaign>,
        bump: u8,
        name: String,
        symbol: String,
        uri: String,
        deposit_deadline: i64,
        trade_deadline: i64,
        donation_goal: u64,
    ) -> Result<()> {
        instructions::init_campaign(
            ctx,
            bump,
            name,
            symbol,
            uri,
            deposit_deadline,
            trade_deadline,
            donation_goal,
        )
    }

    pub fn donate(
        ctx: Context<DonateFund>,
        creator: Pubkey,
        campaign_index: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::donate_fund(
            ctx,
            creator,
            campaign_index,
            amount,
        )
    }

    pub fn claim_fund(ctx: Context<ClaimFundRaised>) -> Result<()> {
        instructions::claim_fund(ctx)
    }

    pub fn create_token(
        ctx: Context<CreatedCampaignToken>, slippage: u16
    ) -> Result<()> {
        instructions::create_campaign_token(ctx, slippage)
    }

    pub fn claim_token(
        ctx: Context<ClaimCampaignToken>,
    ) -> Result<()> {
        instructions::claim_campaign_token(ctx)
    }

    pub fn sell_token(
        ctx: Context<SellCampaignToken>,
        mint_sol_output: u64,
    ) -> Result<()> {
        instructions::sell_campaign_token(ctx, mint_sol_output)
    }
}
