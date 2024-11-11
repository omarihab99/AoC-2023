use std::collections::{HashMap, VecDeque};

use helper::read_input;
use utils::{Module, ModuleType, Pulse};

mod helper;
mod utils;
fn main() {
    let mut modules = read_input();

    part_one(&mut modules, &mut None);
}

fn part_one(modules: &mut HashMap<String, Module>, rx_pulse: &mut Option<bool>) {
    let mut total_low = 0;
    let mut total_high = 0;
    for _ in 0..1000 {
        let (low, high) = push_button(modules, rx_pulse);
        total_low += low;
        total_high += high;
    }
    println!("Result: {}", total_low * total_high);
}
fn push_button(
    modules: &mut HashMap<String, Module>,
    rx_pulse: &mut Option<bool>,
) -> (usize, usize) {
    let mut low_count = 0;
    let mut high_count = 0;
    let mut pulse_queue = VecDeque::new();
    pulse_queue.push_back(Pulse {
        source: "button".to_string(),
        destination: "broadcaster".to_string(),
        is_high: false,
    });
    while let Some(pulse) = pulse_queue.pop_front() {
        if pulse.is_high {
            high_count += 1;
        } else {
            low_count += 1;
        }
        if let Some(module) = modules.get_mut(&pulse.destination) {
            let new_pulses = match &mut module.module_type {
                ModuleType::Broadcaster => module
                    .destinations
                    .iter()
                    .map(|d| Pulse {
                        source: pulse.destination.clone(),
                        destination: d.clone(),
                        is_high: pulse.is_high,
                    })
                    .collect(),
                ModuleType::FlipFlop(state) => {
                    if !pulse.is_high {
                        *state = !*state;
                        module
                            .destinations
                            .iter()
                            .map(|d| Pulse {
                                source: pulse.destination.clone(),
                                destination: d.clone(),
                                is_high: *state,
                            })
                            .collect()
                    } else {
                        vec![]
                    }
                }
                ModuleType::Conjunction(memory) => {
                    memory.insert(pulse.source.clone(), pulse.is_high);
                    let send_high = !memory.values().all(|&x| x);
                    module
                        .destinations
                        .iter()
                        .map(|d| Pulse {
                            source: pulse.destination.clone(),
                            destination: d.clone(),
                            is_high: send_high,
                        })
                        .collect()
                }
            };
            pulse_queue.extend(new_pulses);
        } else if pulse.destination == "rx" && rx_pulse.is_some() {
            *rx_pulse = Some(pulse.is_high);
        }
    }
    (low_count, high_count)
}
