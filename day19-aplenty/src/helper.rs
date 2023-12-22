use crate::workflow::{Condition, Part, Rule};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn read_input() -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut workflow_map = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut blank_line = false;
    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            blank_line = true;
            continue;
        }
        match blank_line {
            true => {
                let part = parse_parts(line.unwrap());
                parts.push(part);
            }
            false => {
                parse_workflow(line.unwrap(), &mut workflow_map);
            }
        }
    }
    (workflow_map, parts)
}
fn parse_parts(line: String) -> Part {
    let binding = line.replace(['{', '}'], "");
    let line = binding.split(",").collect::<Vec<&str>>();
    Part {
        x: line[0].split("=").collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap(),
        m: line[1].split("=").collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap(),
        a: line[2].split("=").collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap(),
        s: line[3].split("=").collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap(),
    }
}
fn parse_workflow(line: String, workflow_map: &mut HashMap<String, Vec<Rule>>) {
    let line = line.split("{").collect::<Vec<&str>>();
    let workflow = line[0].to_string();
    let workflow_rules = line[1].replace("}", "");
    let workflow_rules = workflow_rules.split(",").collect::<Vec<&str>>();
    let mut rules = vec![];
    for rule in workflow_rules {
        let rule = rule.split(":").collect::<Vec<&str>>();
        let rule = if rule.len() == 1 {
            let destination = rule[0];
            Rule {
                condition: None,
                destination: destination.to_string(),
            }
        } else {
            let destination = rule[1];
            let operand1 = rule[0].chars().nth(0).unwrap();
            let operator = rule[0].chars().nth(1).unwrap();
            let operand2 = rule[0].split(operator).collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap();
            let condition = Condition {
                operand1,
                operand2,
                operator,
            };
            Rule {
                condition: Some(condition),
                destination: destination.to_string(),
            }
        };
        rules.push(rule);
    }
    workflow_map.insert(workflow, rules);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_parts() {
        let (workflows, parts) = read_input();
        for part in parts {
            println!("{:?}", part);
        }
        for workflow in workflows {
            println!("Workflow: {:?}, rules: {:?}", workflow.0, workflow.1);
        }
    }
}
