use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::system_instruction::transfer;// this is for the transfer function
use anchor_lang::solana_program::program::invoke_signed;//for the invoke signed function

use crate::error::LyraErrors;
use crate::state::*;

pub fn release_prize(ctx: Context<ReleasePrize>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let winner = &mut ctx.accounts.winner;
    // Transfer prize pool to the winner
    let prize_pool = game.prize_pool;
    game.prize_pool = 0; // Reset prize pool after distribution
                         // Transfer funds to winner (example using transfer method)

    let ix = transfer(
                                    &ctx.accounts.sender.key(), 
                                    &winner.key(), 
                                    prize_pool);

    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.sender.to_account_info(), winner.clone()],
        &[],
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct ReleasePrize<'info> {
    #[account(mut, seeds =[b"lyraGame", sender.key().as_ref()], bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}