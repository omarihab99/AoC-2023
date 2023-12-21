#[derive(Debug,Clone,Copy)]
pub(crate) struct Part {
    pub(crate) x: u32,
    pub(crate) m: u32,
    pub(crate) a: u32,
    pub(crate) s: u32
}
#[derive(Debug,Clone,Copy)]
pub(crate) struct Condition {
    pub(crate) operand1: char,
    pub(crate) operand2: u32,
    pub(crate) operator: char
}
#[derive(Debug,Clone)]
pub(crate) struct Rule {
    pub(crate) condition: Option<Condition>,
    pub(crate) destination: String,
}