use anchor_lang::error_code;


#[error_code]
pub enum DemsError {
    #[msg("name too long")]
    NameTooLong,

    #[msg("user already voted")]
    AlreadyVoted,

    #[msg("Poll is close")]
    PollClose,
}
