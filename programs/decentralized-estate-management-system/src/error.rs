use anchor_lang::error_code;


#[error_code]
pub enum DemsError {
    #[msg("name too long")]
    NameTooLong,
}
