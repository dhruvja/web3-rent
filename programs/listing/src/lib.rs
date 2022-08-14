use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const LISTING_BUMP: &'static [u8] = b"listing";

#[program]
pub mod listing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _listing_id: String, rent: u64) -> Result<()> {
        let listing_parameters = &mut ctx.accounts.listing_account;
        
        listing_parameters.new(rent, ctx.accounts.authority.key());

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(listing_id: String)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, seeds = [LISTING_BUMP, listing_id.as_ref()], bump, space = 1 + 32 + 8 + 8)]
    pub listing_account: Account<'info, ListingParameters>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Status {
    Occupied, 
    Vacant,
}

#[account]
pub struct ListingParameters {
    pub status: Status, // 1
    pub owner: Pubkey, // 32
    pub rent: u64, // 8
}

impl ListingParameters {
    fn new (&mut self, rent: u64, owner: Pubkey) {
        self.status = Status::Vacant;
        self.owner = owner; 
        self.rent = rent;
    }
}
