use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::system_instruction::transfer;// this is for the transfer function
use anchor_lang::solana_program::program::invoke_signed;//for the invoke signed function

use crate::error::LyraErrors;
use crate::state::*;

pub fn check_timer(ctx: Context<CheckTimer>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let current_time = Clock::get()?.unix_timestamp;

    // Check if the timer has run out
    if game.global_timer != 0 && current_time > game.global_timer {
        // Logic to identify the winner and distribute the prize
        // let winner_wallet = ..; // Identify the winner (custom logic needed)
        let prize_pool = game.prize_pool;

        // Transfer prize pool to the winner
        let ix = transfer(
            &ctx.accounts.user.key(), //sender public key
            &ctx.accounts.winner.key(), //winner public key
            prize_pool, //prize pool in lamports
        );

        invoke_signed(
            &ix,
            &[ctx.accounts.user.to_account_info()],
            &[],
        )?;

        // Reset the game state
        game.prize_pool = 0;
        game.query_count = 0;
        game.global_timer = 0;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CheckTimer<'info> {
    #[account(mut, seeds =[b"lyraGame", user.key().as_ref()], bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address = game.last_winner)]
    pub winner: AccountInfo<'info>
}
