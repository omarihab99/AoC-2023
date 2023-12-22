use crate::workflow::{Part, PartRange, Rule};
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
    println!(
        "Part one: {}",
        accepted.iter().fold(0, |acc, part: &Part| acc
            + part.x
            + part.m
            + part.a
            + part.s)
    ); // part one solution
    println!("Part two: {}", part_two(&workflow_map)); // part two solution
}
// Trying every combination of x, m, a, s, from 1-4000 results in 4000^4 attempts, which is too much.
// Instead, we can trace each workflow as branches and calculate each value of x, m, a, s for each condition.
//                          in
//                      /       \
//                    /           \
//                  px            qqz
//                 /  \           /    \
//                A     rfg
//                ...   ...        ...
fn part_two(workflow_map: &HashMap<String, Vec<Rule>>) -> u64 {
    let range = PartRange::default();
    backtrack(&workflow_map, "in".to_string(), range, 0)
}
// part two
fn backtrack(
    workflow_map: &HashMap<String, Vec<Rule>>,
    curr_workflow: String,
    range: PartRange,
    rule_idx: usize,
) -> u64 {
    if curr_workflow.eq("A") {
        return range.permutations();
    }
    if curr_workflow.eq("R") {
        return 0;
    }
    let curr_rules = workflow_map.get(&curr_workflow).unwrap();
    let rule = curr_rules.get(rule_idx).unwrap();
    if let Some(condition) = &rule.condition {
        match condition.operator {
            '<' => {
                let (left, right) = range.split_range(condition.operand2 - 1, condition.operand1);
                let left_result = match rule.destination.as_str() {
                    // This is the case when condition will be taken for this branch.
                    "A" => left.permutations(),
                    "R" => 0,
                    _ => backtrack(workflow_map, rule.clone().destination, left, 0),
                };
                let right_result = backtrack(workflow_map, curr_workflow, right, rule_idx + 1); // This is the case when condition will not be taken in this branch.
                return left_result + right_result;
            }
            // Reverse the operations above.
            '>' => {
                let (left, right) = range.split_range(condition.operand2, condition.operand1);
                let left_result = backtrack(workflow_map, curr_workflow, left, rule_idx + 1);
                let right_result = match rule.destination.as_str() {
                    "A" => right.permutations(),
                    "R" => 0,
                    _ => backtrack(workflow_map, rule.clone().destination, right, 0),
                };
                return left_result + right_result;
            }
            _ => {
                unimplemented!("Unknown operator: {}", condition.operator);
            }
        };
    }
    backtrack(workflow_map, rule.clone().destination, range, 0)
}

// part one
fn process_part(workflow_map: &HashMap<String, Vec<Rule>>, part: Part) -> bool {
    let (x, m, a, s) = (part.x, part.m, part.a, part.s);
    let mut curr_workflow = "in".to_string(); // initial workflow
    while curr_workflow.ne("A") && curr_workflow.ne("R") {
        // loop until accepted or rejected
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
// part one
fn check_part(part_attr: u64, condition_attr: u64, operator: char) -> bool {
    match operator {
        '>' => {
            if part_attr > condition_attr {
                return true;
            }
        }
        '<' => {
            if part_attr < condition_attr {
                return true;
            }
        }
        _ => {
            unimplemented!("Unknown operator: {}", operator);
        }
    }
    false
}
