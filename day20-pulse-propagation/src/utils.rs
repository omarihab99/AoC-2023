use std::collections::HashMap;
#[derive(Debug,Hash, PartialEq, Eq)]
pub(crate) enum Module {
    FLIPFLOP,
    CONJUNCTION,
    BROADCASTER,
}
#[derive(PartialEq)]
pub(crate) enum Pulse {
    LOW,
    HIGH,
}
pub(crate) struct FlipFlop {
    pub(crate) status: bool,
}
impl FlipFlop {
    pub(crate) fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::HIGH => None,
            Pulse::LOW => match self.status {
                false => {
                    self.status = true;
                    Some(Pulse::HIGH)
                }
                true => {
                    self.status = false;
                    Some(Pulse::LOW)
                }
            },
        }
    }
}

pub(crate) struct Conjunction {
    pub(crate) inputs: HashMap<Module, Pulse>,
}
impl Conjunction {
    pub(crate) fn process(&mut self, pulse: Pulse, module: Module) -> Pulse {
        self.inputs.insert(module, pulse);
        if self.inputs.values().all(|x| *x == Pulse::HIGH) {
            Pulse::LOW
        } else {
            Pulse::HIGH
        }
    }
}

pub(crate) struct Broadcaster {}
impl Broadcaster {
    pub(crate) fn process(&mut self, pulse: Pulse) -> Pulse {
        pulse
    }
}
