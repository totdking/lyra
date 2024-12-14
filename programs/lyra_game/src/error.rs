use anchor_lang::prelude::*;

#[error_code]
pub enum LyraErrors{
    #[msg("You cannot start a game that has been started already")]
    GameStart,
    #[msg ("There is no user in this room")]
    EmptyRoom,
    #[msg ("user does not have enough funds to enter this room at this time")]
    PlayerFunded,
    #[msg ("No Liquidity has been added to start the challenge")]
    NoLiquidityInPool,
    #[msg ("The challenge has already been started")]
    AlreadyStarted,
    #[msg ("The challenge has already been ended")]
    AlreadyEnded,
    #[msg ("Winner has been selected already")]
    WinnerSelected,
    #[msg ("Prize has been claimed")]
    PrizeClaimed,
}