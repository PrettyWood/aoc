use std::collections::HashMap;

pub fn solve_part2(input: &str) -> u16 {
    let mut circuit = parse(input);
    let v = circuit.get_value("a");

    // Insert a value into b and reset cache
    circuit
        .wires
        .insert("b".to_string(), Signal::Eq(Value::Number(v)));
    circuit.cache = HashMap::default();

    circuit.get_value("a")
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Signal>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn get_value(&mut self, wire: &str) -> u16 {
        if let Some(n) = self.cache.get(wire) {
            return *n;
        }

        let signal = self
            .wires
            .get(wire)
            .expect("wire id should be in circuit")
            .clone();
        let v = signal.get_value(self);
        self.cache.insert(wire.to_string(), v);
        v
    }
}

#[derive(Debug, Clone)]
enum Value {
    Wire(String),
    Number(u16),
}

impl Value {
    fn get_value(&self, circuit: &mut Circuit) -> u16 {
        match self {
            Value::Number(n) => *n,
            Value::Wire(id) => circuit.get_value(id),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        if let Ok(n) = s.parse::<u16>() {
            Value::Number(n)
        } else {
            Value::Wire(s.to_owned())
        }
    }
}

#[derive(Debug, Clone)]
enum Signal {
    Eq(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, u16),
    RShift(Value, u16),
    Not(Value),
}

impl Signal {
    fn get_value(&self, circuit: &mut Circuit) -> u16 {
        match self {
            Self::Eq(v) => v.get_value(circuit),
            Self::And(v1, v2) => v1.get_value(circuit) & v2.get_value(circuit),
            Self::Or(v1, v2) => v1.get_value(circuit) | v2.get_value(circuit),
            Self::LShift(v, shift) => v.get_value(circuit) << shift,
            Self::RShift(v, shift) => v.get_value(circuit) >> shift,
            Self::Not(v) => !v.get_value(circuit),
        }
    }
}

fn parse(input: &str) -> Circuit {
    let wires = input
        .lines()
        .map(|line| {
            let (left, wire) = line.split_once(" -> ").expect("should have -> ");
            let signal = parse_signal(left);
            (wire.to_string(), signal)
        })
        .collect();
    Circuit {
        wires,
        cache: HashMap::default(),
    }
}

fn parse_signal(input: &str) -> Signal {
    let tokens: Vec<_> = input.split_whitespace().collect();
    match &tokens[..] {
        [v] => Signal::Eq((*v).into()),
        ["NOT", v] => Signal::Not((*v).into()),
        [v1, "AND", v2] => Signal::And((*v1).into(), (*v2).into()),
        [v1, "OR", v2] => Signal::Or((*v1).into(), (*v2).into()),
        [v, "LSHIFT", n] => Signal::LShift(
            (*v).into(),
            (*n).parse::<u16>().expect("should be a valid shift"),
        ),
        [v, "RSHIFT", n] => Signal::RShift(
            (*v).into(),
            (*n).parse::<u16>().expect("should be a valid shift"),
        ),
        _ => panic!("tokens not valid"),
    }
}
