#[derive(Debug)]
pub enum SeekOrigin {
    Begin,
    Current,
    End
}

#[derive(Debug)]
pub enum SeekError {
    BeforeBegin,
    AfterEnd
}
