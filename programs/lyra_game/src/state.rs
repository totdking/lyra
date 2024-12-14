use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub prize_pool: u64,
    pub developer_wallet: u64,
    pub query_fee: u64,
    pub query_count: u64,
    pub global_timer: i64,
    pub bump: u8,
    pub last_winner: Pubkey,
    pub game_started: bool,
}