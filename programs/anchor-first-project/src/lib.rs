use anchor_lang::prelude::*;

declare_id!("Bxt2pzDoJ5zKbbLeC1u2qDa66YrLX8zLiRdgL6iNp6R");

#[program]
pub mod anchor_first_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
