use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Permission denied")]
    PermissionDenied,

    #[msg("Game is already completed")]
    GameAlreadyCompleted,

    #[msg("Invalid Position")]
    InvalidPosition,

    #[msg("Invalid Corner")]
    InvalidCorner,

    #[msg("No Enough Fund")]
    NoEnoughFund,

    #[msg("Game Match Already Finished")]
    GameMatchAlreadyFinished,

    #[msg("Wrong Player To Result")]
    WrongPlayerToResult,
}
