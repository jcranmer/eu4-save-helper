

pub enum ParadoxValue<'a> {
    String(pub String),
    Integer(pub i32),
    Fixed(pub Fixed),
    UnparsedComplex(&'a Parser)
}

impl TryFrom<ParadoxValue<'_>> for String {
}
