use anchor_lang::prelude::*;
use crate::error::LyraErrors;
use crate::state::*;

pub fn initialize_game(ctx: Context<InitializeGame>, start_fee: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    require!(game.game_started == false, LyraErrors::GameStart);
    
    game.prize_pool = 0;
    game.developer_wallet = 0;
    game.query_fee = start_fee;
    game.query_count = 0;
    game.global_timer = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, 
              payer = user, 
              space = 8 + 8 + 8 + 8 + 8 + 8, 
              seeds =[b"lyraGame", user.key().as_ref()], 
              bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}