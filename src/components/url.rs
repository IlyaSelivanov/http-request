pub enum Mode {
    Normal,
    EditUrl,
}

pub struct url {
    pub text: String,
    pub mode: Mode,
}
