use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

pub struct Device {
    wires: HashMap<String, bool>, // wire label -> current value
    gates: Vec<Gate>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Gate {
    input1: String, // wire label
    input2: String, // wire label
    output: String, // wire label
    gate_type: GateType,
}

impl Gate {
    fn print(&self) {
        print!(
            "{} {:?} {} -> {}",
            self.input1, self.gate_type, self.input2, self.output
        );
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl GateType {
    fn process(&self, a: bool, b: bool) -> bool {
        match self {
            GateType::And => a && b,
            GateType::Or => a || b,
            GateType::Xor => a ^ b,
        }
    }
}

impl FromStr for GateType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AND" => Ok(GateType::And),
            "OR" => Ok(GateType::Or),
            "XOR" => Ok(GateType::Xor),
            _ => Err(()),
        }
    }
}

pub fn parse_day24(lines: &[String]) -> Device {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        if let Some((label, value_str)) = line.split_once(": ") {
            let value = if let Ok(val) = value_str.parse::<u8>() {
                val == 1
            } else {
                panic!("expected a number")
            };
            wires.insert(label.to_string(), value);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 5);
            let gate_type = GateType::from_str(parts[1]).expect("unknown gate type");
            gates.push(Gate {
                input1: parts[0].to_string(),
                gate_type,
                input2: parts[2].to_string(),
                output: parts[4].to_string(),
            });
        }
    }
    Device { wires, gates }
}

pub fn result_day24_stage1(device: &mut Device) -> u64 {
    let mut queue: VecDeque<Gate> = device.gates.iter().cloned().collect();
    while let Some(gate) = queue.pop_front() {
        if device.wires.contains_key(&gate.input1) && device.wires.contains_key(&gate.input2) {
            let value1 = *device.wires.get(&gate.input1).unwrap();
            let value2 = *device.wires.get(&gate.input2).unwrap();
            device
                .wires
                .insert(gate.output, gate.gate_type.process(value1, value2));
        } else {
            // The gate cannot currently be processed, so push it back onto the queue
            queue.push_back(gate);
        }
    }
    wires_to_decimal(device, 'z')
}

fn wires_to_decimal(device: &Device, prefix: char) -> u64 {
    let mut output_values: Vec<(String, bool)> = device
        .wires
        .iter()
        .filter_map(|(name, &value)| {
            if name.starts_with(prefix) {
                Some((name.clone(), value))
            } else {
                None
            }
        })
        .collect();
    // Sort by name
    output_values.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    output_values.iter().enumerate().fold(
        0,
        |acc, (i, &(_, val))| {
            if val { acc + (1 << i) } else { acc }
        },
    )
}

// With a bit of help from Reddit, I read that the circuit represents
// a Ripple Carry Adder. My solution was to draw an adder for 1-bit
// on a piece of paper and come up with some heuristics of what type
// of wires can and can't go into a gate. The goal was to find all
// the bad wires by finding wires that break the heuristics.
// E.g. I figured out that the OR gate can't ever have an xNN, yNN,
// or zNN connection, unless it's the last bit.
pub fn result_day24_stage2(device: &Device) -> String {
    let input_map = build_input_map(device);
    for n in 0..45 {
        find_nth_bit_adder(n, &input_map);
        println!();
    }

    let mut bad_wires: HashSet<String> = HashSet::new();
    let bad = or_gates_no_xyz(&device.gates);
    println!("CHECK: OR gates can't have xyz wires in or out: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_gate_no_xyz_output(&device.gates);
    println!("CHECK: AND gates can't have xyz outputs: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_xor_gates_both_xyz_or_none(&device.gates);
    println!(
        "CHECK: AND/XOR gate inputs are both or neither xyz: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = and_output_is_or_input(&device.gates, &input_map);
    println!("CHECK: AND outputs are followed by a single OR: {:?}", bad);
    bad_wires.extend(bad);

    let bad = or_output_goes_in_one_and_one_xor(&device.gates, &input_map);
    println!(
        "CHECK: OR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_output_non_z_goes_in_one_and_one_xor(&device.gates, &input_map);
    println!(
        "CHECK: Non-z XOR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_with_non_xy_in_has_z_out(&device.gates);
    println!("CHECK: XOR with non-xy inputs has z output: {:?}", bad);
    bad_wires.extend(bad);

    let mut sorted: Vec<String> = bad_wires.into_iter().collect();
    sorted.sort();
    sorted.join(",")
}

fn build_input_map(device: &Device) -> HashMap<String, Vec<Gate>> {
    let mut map = HashMap::new();
    for wire in device.wires.keys() {
        let mut gates_in = Vec::new();
        for gate in device.gates.iter() {
            if &gate.input1 == wire || &gate.input2 == wire {
                gates_in.push(gate.clone());
            }
        }
        map.insert(wire.to_string(), gates_in);
    }
    map
}

fn find_nth_bit_adder(n: usize, input_map: &HashMap<String, Vec<Gate>>) {
    let x_name = format!("x{:02}", n);
    let y_name = format!("y{:02}", n);

    let mut gates: HashSet<Gate> = HashSet::new();
    for gate in input_map.get(&x_name).unwrap() {
        gates.insert(gate.clone());
    }
    for gate in input_map.get(&y_name).unwrap() {
        gates.insert(gate.clone());
    }
    let mut next_gates: HashSet<Gate> = HashSet::new();
    for gate in &gates {
        if !gate.output.starts_with("z") {
            for next_gate in input_map.get(&gate.output).unwrap() {
                next_gates.insert(next_gate.clone());
            }
        }
    }
    gates.extend(next_gates);

    for gate in gates {
        gate.print();
        println!(" is part of bit {}", n);
    }
}

fn is_xyz(wire: &str) -> bool {
    wire.starts_with("x") || wire.starts_with("y") || wire.starts_with("z")
}

fn or_gates_no_xyz(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.gate_type != GateType::Or {
            continue;
        }
        if is_xyz(&gate.input1) {
            bad_wires.push(gate.input1.clone());
        }
        if is_xyz(&gate.input2) {
            bad_wires.push(gate.input2.clone());
        }
        if is_xyz(&gate.output) && gate.output != "z45" {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn and_gate_no_xyz_output(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.gate_type != GateType::And {
            continue;
        }
        if is_xyz(&gate.output) {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn and_xor_gates_both_xyz_or_none(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if ![GateType::And, GateType::Xor].contains(&gate.gate_type) {
            continue;
        }
        if (is_xyz(&gate.input1) && !is_xyz(&gate.input2))
            || (!is_xyz(&gate.input1) && is_xyz(&gate.input2))
        {
            bad_wires.push(gate.input1.clone());
            bad_wires.push(gate.input2.clone());
        }
    }
    bad_wires
}

fn and_output_is_or_input(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        let inputs = (&gate.input1, &gate.input2);
        if gate.gate_type != GateType::And
        // the output of the 0th bit doesn't fulfil this condition
        || inputs == (&String::from("x00"), &String::from("y00"))
        || inputs == (&String::from("y00"), &String::from("x00"))
        {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 1 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if next_gates[0].gate_type != GateType::Or {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn or_output_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.gate_type != GateType::Or || gate.output == "z45" {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].gate_type == GateType::And && next_gates[1].gate_type == GateType::Xor)
            || (next_gates[0].gate_type == GateType::Xor
                && next_gates[1].gate_type == GateType::And))
        {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn xor_output_non_z_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.gate_type != GateType::Xor || gate.output.starts_with("z") {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].gate_type == GateType::And && next_gates[1].gate_type == GateType::Xor)
            || (next_gates[0].gate_type == GateType::Xor
                && next_gates[1].gate_type == GateType::And))
        {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn xor_with_non_xy_in_has_z_out(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.gate_type != GateType::Xor || is_xyz(&gate.input1) || is_xyz(&gate.input2) {
            continue;
        }
        if !gate.output.starts_with("z") {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

#[cfg(test)]
mod day24 {
    use super::*;

    fn get_small_example() -> Vec<String> {
        vec![
            String::from("x00: 1"),
            String::from("x01: 1"),
            String::from("x02: 1"),
            String::from("y00: 0"),
            String::from("y01: 1"),
            String::from("y02: 0"),
            String::from(""),
            String::from("x00 AND y00 -> z00"),
            String::from("x01 XOR y01 -> z01"),
            String::from("x02 OR y02 -> z02"),
        ]
    }

    fn get_large_example() -> Vec<String> {
        vec![
            String::from("x00: 1"),
            String::from("x01: 0"),
            String::from("x02: 1"),
            String::from("x03: 1"),
            String::from("x04: 0"),
            String::from("y00: 1"),
            String::from("y01: 1"),
            String::from("y02: 1"),
            String::from("y03: 1"),
            String::from("y04: 1"),
            String::from(""),
            String::from("ntg XOR fgs -> mjb"),
            String::from("y02 OR x01 -> tnw"),
            String::from("kwq OR kpj -> z05"),
            String::from("x00 OR x03 -> fst"),
            String::from("tgd XOR rvg -> z01"),
            String::from("vdt OR tnw -> bfw"),
            String::from("bfw AND frj -> z10"),
            String::from("ffh OR nrd -> bqk"),
            String::from("y00 AND y03 -> djm"),
            String::from("y03 OR y00 -> psh"),
            String::from("bqk OR frj -> z08"),
            String::from("tnw OR fst -> frj"),
            String::from("gnj AND tgd -> z11"),
            String::from("bfw XOR mjb -> z00"),
            String::from("x03 OR x00 -> vdt"),
            String::from("gnj AND wpb -> z02"),
            String::from("x04 AND y00 -> kjc"),
            String::from("djm OR pbm -> qhw"),
            String::from("nrd AND vdt -> hwm"),
            String::from("kjc AND fst -> rvg"),
            String::from("y04 OR y02 -> fgs"),
            String::from("y01 AND x02 -> pbm"),
            String::from("ntg OR kjc -> kwq"),
            String::from("psh XOR fgs -> tgd"),
            String::from("qhw XOR tgd -> z09"),
            String::from("pbm OR djm -> kpj"),
            String::from("x03 XOR y03 -> ffh"),
            String::from("x00 XOR y04 -> ntg"),
            String::from("bfw OR bqk -> z06"),
            String::from("nrd XOR fgs -> wpb"),
            String::from("frj XOR qhw -> z04"),
            String::from("bqk OR frj -> z07"),
            String::from("y03 OR x01 -> nrd"),
            String::from("hwm AND bqk -> z03"),
            String::from("tgd XOR rvg -> z12"),
            String::from("tnw OR pbm -> gnj"),
        ]
    }

    #[test]
    fn small_example_parse() {
        let device = parse_day24(&get_small_example());
        let mut expected_wires = HashMap::new();
        expected_wires.insert(String::from("x00"), true);
        expected_wires.insert(String::from("x01"), true);
        expected_wires.insert(String::from("x02"), true);
        expected_wires.insert(String::from("y00"), false);
        expected_wires.insert(String::from("y01"), true);
        expected_wires.insert(String::from("y02"), false);
        let expected_gates = vec![
            Gate {
                input1: String::from("x00"),
                input2: String::from("y00"),
                output: String::from("z00"),
                gate_type: GateType::And,
            },
            Gate {
                input1: String::from("x01"),
                input2: String::from("y01"),
                output: String::from("z01"),
                gate_type: GateType::Xor,
            },
            Gate {
                input1: String::from("x02"),
                input2: String::from("y02"),
                output: String::from("z02"),
                gate_type: GateType::Or,
            },
        ];
        assert_eq!(device.wires, expected_wires);
        assert_eq!(device.gates, expected_gates);
    }

    #[test]
    fn stage1_small() {
        let mut device = parse_day24(&get_small_example());
        let result = result_day24_stage1(&mut device);
        assert_eq!(result, 4);
    }

    #[test]
    fn stage1_large() {
        let mut device = parse_day24(&get_large_example());
        let result = result_day24_stage1(&mut device);
        assert_eq!(result, 2024);
    }
}
