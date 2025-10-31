use anchor_lang::prelude::*;

declare_id!("9GpL1ZXeQM9L9BJJnZySCST95W3Pc5SXoiPp689cT8q9");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
