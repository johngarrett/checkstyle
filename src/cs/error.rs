// this associates an error to a line number, manipluation will be done later
pub enum CSError {
    TrallingWhiteSpace(u16),
    Indentation(u16),
    MissingAuthor(u16),
    MissingParam(u16),
    MissingReturn(u16),
}
