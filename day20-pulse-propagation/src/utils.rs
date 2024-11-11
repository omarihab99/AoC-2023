use std::collections::HashMap;
pub(crate) enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}
pub(crate) struct Module {
    pub(crate) module_type: ModuleType,
    pub(crate) destinations: Vec<String>,
}
pub(crate) struct Pulse {
    pub(crate) source: String,
    pub(crate) destination: String,
    pub(crate) is_high: bool,
}
