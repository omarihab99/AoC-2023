use crate::workflow::{Part, Rule};
use std::collections::HashMap;

mod helper;
mod workflow;

fn main() {
    let (workflow_map, parts) = helper::read_input();
    let mut accepted: Vec<Part> = Vec::new();
    for part in parts {
        if process_part(&workflow_map, part) {
            accepted.push(part);
        }
    }
    println!("Part one: {}", accepted.iter().fold(0, |acc, part: &Part| acc + part.x + part.m + part.a + part.s));
}

fn process_part(workflow_map: &HashMap<String, Vec<Rule>>, part: Part) -> bool {
    let (x, m, a, s) = (part.x, part.m, part.a, part.s);
    let mut curr_workflow = "in".to_string(); // initial workflow
    while curr_workflow.ne("A") && curr_workflow.ne("R") { // loop until accepted or rejected
        let curr_rules = workflow_map.get(&curr_workflow).unwrap();
        for rule in curr_rules.iter() {
            match rule.condition {
                Some(condition) => {
                    let check = match condition.operand1 {
                        'x' => check_part(x, condition.operand2, condition.operator),
                        'm' => check_part(m, condition.operand2, condition.operator),
                        'a' => check_part(a, condition.operand2, condition.operator),
                        's' => check_part(s, condition.operand2, condition.operator),
                        _ => {
                            unimplemented!("Unknown operand: {}", condition.operand1);
                        }
                    };
                    if check {
                        curr_workflow = rule.clone().destination; // go to next workflow.
                        break;
                    }
                }
                None => {
                    curr_workflow = rule.clone().destination; // go to next workflow if this is the last rule.
                }
            }
        }

    }
    return if curr_workflow == "A" { true } else { false }; // return true if accepted.
}

fn check_part(part_attr: u32, condition_attr: u32, operator: char) -> bool {
    match operator {
        '>' => {
            if part_attr > condition_attr {
                return true
            }
        }
        '<' => {
            if part_attr < condition_attr {
                return true
            }
        }
        _ => {
            unimplemented!("Unknown operator: {}", operator);
        }
    }
    false
}
