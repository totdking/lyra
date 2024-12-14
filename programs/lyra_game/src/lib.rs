use anchor_lang::prelude::*;

use crate::instruction::*;
pub mod instruction;

pub mod error;
pub mod state;

declare_id!("HvMDyWvHheS5HYNJhQyEESibd1EBwXpVroSucT7UFbgc");

pub mod lyra_game {

    use super::*;
    // Initialize the game (set up prize pool and timer)
    pub fn initialize(ctx: Context<InitializeGame>, start_fee: u64) -> Result<()> {
        initialize_game(ctx, start_fee)
    }
    
    
    // Handle the query submission and update prize pool
    pub fn submit(ctx: Context<SubmitQuery>) -> Result<()> {
        submit_query(ctx)
    }

    //to end the contest if they are not up to 200 queries and the time is elapsed
    pub fn check(ctx: Context<CheckTimer>) -> Result<()> {
        check_timer(ctx)
    }

    // In case of a winner, distribute the prize pool to the winner
    pub fn release(ctx: Context<ReleasePrize>) -> Result<()> {
       release_prize(ctx)
    }
}

