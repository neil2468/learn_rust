use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("count = {}", input.lines().count());

    let mut circuit = Circuit::new();

    for line in input.lines() {
        println!("{}", line);
        let mut split: Vec<_> = line.split_whitespace().collect();
        let output: &str = split.pop().unwrap();
        split.pop();
        circuit.add(Input::from_text(&split), output);
    }

    let a = circuit.eval("a", 0, &mut HashMap::new());
    println!("wire a = {}", a);

    // Part two...
    circuit.add(Input::Literal(46065), "b");
    let a = circuit.eval("a", 0, &mut HashMap::new());
    println!("wire a = {}", a);
}

#[derive(Debug)]
enum Input {
    Literal(u16),
    Nop(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u32),
    RShift(String, u32),
}

impl Input {
    fn from_text(text: &Vec<&str>) -> Input {
        let result: Input;
        if text.len() == 1 && text[0].chars().next().unwrap().is_ascii_digit() {
            result = Input::Literal(text[0].parse::<u16>().unwrap());
        } else if text.len() == 1 {
            result = Input::Nop(text[0].to_string());
        } else if text[0] == "NOT" {
            result = Input::Not(text[1].to_string());
        } else if text[1] == "AND" {
            result = Input::And(text[0].to_string(), text[2].to_string());
        } else if text[1] == "OR" {
            result = Input::Or(text[0].to_string(), text[2].to_string());
        } else if text[1] == "LSHIFT" {
            result = Input::LShift(text[0].to_string(), text[2].parse::<u32>().unwrap());
        } else if text[1] == "RSHIFT" {
            result = Input::RShift(text[0].to_string(), text[2].parse::<u32>().unwrap());
        } else {
            panic!("Unable to parse text '{:?}", text);
        }
        result
    }
}

struct Circuit {
    elements: HashMap<String, Input>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            elements: HashMap::new(),
        }
    }

    fn add(&mut self, input: Input, output: &str) {
        self.elements.insert(String::from(output), input);
    }

    fn eval(&self, output: &str, depth: u32, cache: &mut HashMap<String, u16>) -> u16 {
        let new_depth = depth + 1;
        let element = self.elements.get(output);
        println!("eval {:?}", output);

        // Check cache
        let result = cache.get(output);
        if let Some(val) = result {
            println!("Cache hit");
            return *val;
        }

        // Evaluate (recursively)
        let result = match element {
            Some(Input::Literal(val)) => *val,
            Some(Input::Nop(op)) => self.eval(op, new_depth, cache),
            Some(Input::Not(op)) => !self.eval(op, new_depth, cache),
            Some(Input::And(op1, op2)) => {
                self.eval(op1, new_depth, cache) & self.eval(op2, new_depth, cache)
            }
            Some(Input::Or(op1, op2)) => {
                self.eval(op1, new_depth, cache) | self.eval(op2, new_depth, cache)
            }
            Some(Input::LShift(op1, op2)) => self.eval(op1, new_depth, cache) << op2,
            Some(Input::RShift(op1, op2)) => self.eval(op1, new_depth, cache) >> op2,
            _ => {
                // Handle literal operands
                if !output.chars().next().unwrap().is_ascii_digit() {
                    panic!("Unknown output '{}'", output);
                }
                output.parse::<u16>().unwrap()
            }
        };

        // Update cache
        cache.insert(output.to_string(), result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let mut circuit = Circuit::new();
        circuit.add(Input::Literal(123), "x");
        circuit.add(Input::Literal(456), "y");
        circuit.add(Input::And("x".to_string(), "y".to_string()), "d");
        circuit.add(Input::Or("x".to_string(), "y".to_string()), "e");
        circuit.add(Input::LShift("x".to_string(), 2), "f");
        circuit.add(Input::RShift("y".to_string(), 2), "g");
        circuit.add(Input::Not("x".to_string()), "h");
        circuit.add(Input::Not("y".to_string()), "i");

        circuit.add(Input::Nop("x".to_string()), "xx");

        assert_eq!(circuit.eval("d"), 72);
        assert_eq!(circuit.eval("e"), 507);
        assert_eq!(circuit.eval("f"), 492);
        assert_eq!(circuit.eval("g"), 114);
        assert_eq!(circuit.eval("h"), 65412);
        assert_eq!(circuit.eval("i"), 65079);
        assert_eq!(circuit.eval("x"), 123);
        assert_eq!(circuit.eval("y"), 456);
        assert_eq!(circuit.eval("xx"), 123);
    }
}
