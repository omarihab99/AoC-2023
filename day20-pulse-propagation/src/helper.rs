use std::collections::HashMap;

use crate::utils::{Module, ModuleType};
pub(crate) fn read_input() -> HashMap<String, Module> {
    let input = include_str!("input.txt");
    let mut modules = HashMap::new();
    let mut conjunction_inputs: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts[0].starts_with("&") {
            let name = parts[0][1..].to_string();
            conjunction_inputs.insert(name, Vec::new());
        }
    }
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let destinations: Vec<String> =
            parts[1].split(", ").map(|x| x.trim().to_string()).collect();
        let (name, module_type) = if parts[0] == "broadcaster" {
            ("broadcaster".to_string(), ModuleType::Broadcaster)
        } else {
            let type_char = parts[0].chars().next().unwrap();
            let name = parts[0][1..].to_string();
            match type_char {
                '&' => (name, ModuleType::Conjunction(HashMap::new())),
                '%' => (name, ModuleType::FlipFlop(false)),
                _ => panic!("Unsupported module type."),
            }
        };
        for des in &destinations {
            if conjunction_inputs.contains_key(des) {
                conjunction_inputs.get_mut(des).unwrap().push(name.clone());
            }
        }
        modules.insert(
            name,
            Module {
                module_type,
                destinations,
            },
        );
    }
    for (name, module) in modules.iter_mut() {
        if let ModuleType::Conjunction(ref mut memory) = module.module_type {
            if let Some(inputs) = conjunction_inputs.get(name) {
                for input in inputs {
                    memory.insert(input.clone(), false);
                }
            }
        }
    }
    modules
}
