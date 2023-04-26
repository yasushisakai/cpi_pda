use anchor_lang::prelude::*;

declare_id!("9aLnL3iN99rJNz9VqQrEzijLDN5dcHfnCoitR6zbWYFM");

#[program]
pub mod worker {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.value += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(init, payer=user, space=8+8)]
    data: Account<'info, Data>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64
}
