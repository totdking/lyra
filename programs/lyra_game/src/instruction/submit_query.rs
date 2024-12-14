use anchor_lang::prelude::*;
use crate::error::LyraErrors;
use crate::state::*;

pub fn submit_query(ctx: Context<SubmitQuery>) -> Result<()> {
    //Scaling so as to store the values of the variables in one-tenth of a cent
    
    const SCALE: u64 = 1000;
    let game = &mut ctx.accounts.game;

    //Add a require that would ensure a player has enough funds to enter the contest

    let prize_ratio = (0.8 * SCALE as f64) as u64 ;
    let dev_ratio = (0.2 * SCALE as f64) as u64;

    // Increase query fee by 0.5% with each query
    game.query_fee = (game.query_fee as f64 * 1.005) as u64;

    // 80% of query fee goes to the prize pool, 20% to dev wallet
    let fee = game.query_fee;
    let scaled_fee = fee * SCALE;

    let prize_share = (scaled_fee * prize_ratio) / SCALE / SCALE;
    let dev_share = (scaled_fee * dev_ratio) / SCALE / SCALE;

    game.prize_pool += prize_share;
    game.developer_wallet += dev_share;

    // Increment the query count and check timer logic
    game.query_count += 1;
    if game.query_count >= 200 {
        let current_time = Clock::get()?.unix_timestamp;
        game.global_timer = current_time + 60 * 60; // Set timer to 60 minutes from now
    }
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitQuery<'info> {
    #[account(mut, seeds =[b"lyraGame", user.key().as_ref()], bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}