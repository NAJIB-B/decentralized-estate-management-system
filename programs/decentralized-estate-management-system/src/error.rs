use anchor_lang::error_code;


#[error_code]
pub enum DemsError {
    #[msg("name too long")]
    NameTooLong,

    #[msg("name should not be empty")]
    InvalidName,

    #[msg("description is too long")]
    DescriptionTooLong,

    #[msg("description should not be empty")]
    InvalidDescription,


    #[msg("Amount is invalid")]
    InvalidAmount,

    #[msg("Amount exceeds vault balance")]
    ExceededBalance,

    #[msg("user already voted")]
    AlreadyVoted,

    #[msg("Poll is close")]
    PollClose,
}
