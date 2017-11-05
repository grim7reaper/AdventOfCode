// {{{ Lints

#![deny(missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unstable_features,
        unused_import_braces,
        unused_qualifications
)]

// }}}
// {{{ Crates

use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate nom;

// }}}
// {{{ Instruction

/// Module that defines instructions to build a circuit.
mod instruction {
    use std::str;
    use nom;

    /// Input value for instruction's operations.
    #[derive(Clone, Copy, Debug)]
    pub enum Input<'a> {
        Wire(&'a str), // A wire name.
        Value(u16),    // A value.
    }

    /// An instruction to build a circuit.
    #[derive(Clone, Copy, Debug)]
    pub enum Instruction<'a> {
        Assign { value: Input<'a>,                   out: &'a str },
        Not    { value: Input<'a>,                   out: &'a str },
        And    { left:  Input<'a>, right: Input<'a>, out: &'a str },
        Or     { left:  Input<'a>, right: Input<'a>, out: &'a str },
        LShift { left:  Input<'a>, right: u8,        out: &'a str },
        RShift { left:  Input<'a>, right: u8,        out: &'a str },
    }

    impl<'a> From<&'a str> for Instruction<'a> {
        /// Builds an `Instruction` from a string.
        fn from(s: &str) -> Instruction {
            let res = parse_instruction(s.as_bytes());
            res.to_result().unwrap()
        }
    }

    // {{{ Parser

    /// Parse a wire's name.
    named!(name<&[u8], &str>, map_res!(nom::alpha, str::from_utf8));

    /// Parse an u16 value.
    named!(value<&[u8], u16>,
        do_parse!(
            value: map_res!(nom::digit, str::from_utf8) >>
            (value.parse::<u16>().unwrap())
        )
    );

    /// Parse a shift offset.
    named!(offset<&[u8], u8>,
        do_parse!(
            value: map_res!(nom::digit, str::from_utf8) >>
            (value.parse::<u8>().unwrap())
        )
    );

    /// Parse an input value.
    ///
    /// An input value can be a integer literal or a wire name.
    named!(input<&[u8], Input>,
        alt!(
            value => { |v| Input::Value(v)  }
          | name  => { |s| Input::Wire(s)   }
        )
    );

    /// Parse an assignement.
    named!(assign<&[u8], Instruction>,
        do_parse!(
            value: input >>
            tag!(" -> ") >>
            out: name    >>
            (Instruction::Assign { value, out })
       )
    );

    /// Parse the NOT operator.
    named!(not<&[u8], Instruction>,
        do_parse!(
            tag!("NOT ") >>
            value: input >>
            tag!(" -> ") >>
            out: name    >>
            (Instruction::Not { value, out })
        )
    );

    /// Parse the OR or AND operator.
    named!(and_or<&[u8], Instruction>,
        do_parse!(
            left: input                        >>
            char!(' ')                         >>
            op: alt!(tag!("AND") | tag!("OR")) >>
            char!(' ')                         >>
            right: input                       >>
            tag!(" -> ")                       >>
            out: name                          >>
            (if op == b"AND" {
                Instruction::And { left, right, out }
            } else {
                Instruction::Or  { left, right, out }
            })
       )
    );

    /// Parse the shift operators (LSHIFT or RSHIFT).
    named!(shift<&[u8], Instruction>,
        do_parse!(
            left: input                               >>
            char!(' ')                                >>
            op: alt!(tag!("LSHIFT") | tag!("RSHIFT")) >>
            char!(' ')                                >>
            right: offset                             >>
            tag!(" -> ")                              >>
            out: name                                 >>
            (if op == b"LSHIFT" {
                Instruction::LShift { left, right, out }
            } else {
                Instruction::RShift { left, right, out }
            })
       )
    );

    /// Parse an instruction.
    named!(parse_instruction<&[u8], Instruction>,
        alt_complete!(assign | not | and_or | shift)
    );

    // }}}
}

// }}}
// {{{ Circuit

/// A module to build and emulate a circuit.
mod circuit {
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use instruction::{Instruction, Input};

    // {{{ Operand

    /// An operand for an operation.
    ///
    /// Can be either a constant value or a wire.
    #[derive(Clone, Debug)]
    pub enum Operand {
        Wire(String),  // A wire.
        Literal(u16),  // An integer literal.
    }

    impl<'a> From<Input<'a>> for Operand {
        /// Create an Operand from an instructions::Input.
        fn from(input: Input) -> Operand {
            match input {
                Input::Value(v) => Operand::Literal(v),
                Input::Wire(w)  => Operand::Wire(w.to_owned()),
            }
        }
    }

    // }}}
    // {{{ Operation

    /// An operation.
    ///
    /// Can be one of the five logic gates.
    #[derive(Clone, Debug)]
    pub enum Operation {
        Assign { wire:  String                  },
        Not    { op:    Operand,                },
        And    { left:  Operand, right: Operand },
        Or     { left:  Operand, right: Operand },
        LShift { left:  Operand, right: u8,     },
        RShift { left:  Operand, right: u8,     },
    }

    // }}}
    // {{{ Signal

    /// A signal on a wire.
    ///
    /// The signal's value can be:
    /// - known or computed
    /// - defined by an operation (whose evaluation will produce the value).
    #[derive(Clone, Debug)]
    pub enum Signal {
        Value(u16),    // A known or already computed value.
        Op(Operation), // An operation to compute the value.
    }

    impl<'a> From<Instruction<'a>> for Signal {
        /// Create a Signal from an instructions::Instruction.
        fn from(instruction: Instruction) -> Signal {
            match instruction {
                Instruction::Assign { value, .. } => {
                    match value {
                        Input::Value(v) =>
                            Signal::Value(v),
                        Input::Wire(w)  =>
                            Signal::Op(Operation::Assign {
                                wire: w.to_owned()
                            }),
                    }
                },
                Instruction::Not { value , .. } =>
                    Signal::Op(Operation::Not {
                        op: Operand::from(value)
                    }),
                Instruction::And { left, right, .. } =>
                    Signal::Op(Operation::And {
                        left:  Operand::from(left),
                        right: Operand::from(right),
                    }),
                Instruction::Or { left, right, .. } =>
                    Signal::Op(Operation::Or {
                        left:  Operand::from(left),
                        right: Operand::from(right),
                    }),
                Instruction::LShift { left, right, .. } =>
                    Signal::Op(Operation::LShift {
                        left:  Operand::from(left),
                        right: right,
                    }),
                Instruction::RShift { left, right, .. } =>
                    Signal::Op(Operation::RShift {
                        left:  Operand::from(left),
                        right: right,
                    }),
            }
        }
    }

    // }}}
    // {{{ Circuit

    /// A Circuit
    ///
    /// A circuit is composed of a set of wires that are connected to each
    /// others.
    #[derive(Debug)]
    pub struct Circuit {
        wires: HashMap<String, Signal>,
        // Graph represented by an adjacency list.
        graph: HashMap<String, Vec<String>>
    }

    impl Circuit {
        /// Initialize a circuit.
        pub fn new() -> Self {
            Circuit {
                wires: HashMap::new(),
                graph: HashMap::new()
            }
        }

        /// Build a circuit from a description.
        pub fn build(&mut self, description : &str) {
            let instructions = description.lines()
                                          .map(|line| Instruction::from(line))
                                          .collect::<Vec<_>>();

            for instruction in instructions.into_iter() {
                match instruction {
                    Instruction::Assign { value, out } |
                    Instruction::Not    { value, out } => {
                        self.add_wire(out, Signal::from(instruction));
                        self.update_relations(value, out);
                    },
                    Instruction::And { left, right, out } |
                    Instruction::Or  { left, right, out } => {
                        self.add_wire(out, Signal::from(instruction));
                        self.update_relations(right, out);
                        self.update_relations(left, out);
                    },
                    Instruction::LShift { left, out, .. } |
                    Instruction::RShift { left, out, .. } => {
                        self.add_wire(out, Signal::from(instruction));
                        self.update_relations(left, out);
                    },
                }
            }
        }

        /// Emulate the circuit until all the values are propagated.
        pub fn emulate(&mut self) {
            let empty = vec![];

            // Initialize the queue with the wires which have a known value.
            let mut queue = self.wires.iter().filter_map(|(wire, value)|
                match value {
                    &Signal::Value(_) => Some(wire.to_owned()),
                    &Signal::Op(_)    => None,
                }
            ).collect::<VecDeque<String>>();

            // Propagate the values.
            while !queue.is_empty() {
                let src = queue.pop_front().unwrap();

                for tgt in self.graph.get(&src).unwrap_or(&empty) {
                    let signal = self.wires.remove(tgt).unwrap();
                    let mut new_signal = signal.clone();

                    // The wire's signal is unknown: try to compute it!
                    if let Signal::Op(op) = signal {
                        if let Some(value) = self.evaluate_operation(op) {
                            // We have a value: we can update the wire's signal
                            // and process its successor.
                            new_signal = Signal::Value(value);
                            queue.push_back(tgt.to_owned());
                        }
                    }

                    // XXX Can't call `add_wire` because of the borrow of `for`.
                    self.wires.insert(tgt.to_owned(), new_signal);
                }
            }
        }

        /// Returns the signal on the specified wire.
        ///
        /// Returns None if the wire doesn't exists or if the wire signal wasn't
        /// computed.
        pub fn get_signal(&self, wire: &str) -> Option<u16> {
            self.wires.get(wire).and_then(|signal|
                match *signal {
                    Signal::Value(v) => Some(v),
                    _                => None
                }
            )
        }

        // {{{ Internals

        // Add a wire into the circuit.
        fn add_wire(&mut self, wire: &str, signal: Signal) {
            if let Some(_) = self.wires.insert(wire.to_owned(), signal) {
                panic!("wire {} already exists", wire);
            }
        }

        // Update the adjacency list that represent the relation between the wires.
        fn update_relations(&mut self, value: Input, target: &str) {
            if let Input::Wire(wire) = value {
                let source = self.graph.entry(wire.to_owned())
                                       .or_insert(Vec::new());
                (*source).push(target.to_owned());
            }
        }

        fn get_operand_value(&self, operand: Operand) -> Option<u16> {
            match operand {
                Operand::Wire(wire)     => self.get_signal(&wire),
                Operand::Literal(value) => Some(value),
            }
        }

        // Try to evaluate the given operation.
        //
        // If one of the operand is not known, None is returned.
        fn evaluate_operation(&self, op: Operation) -> Option<u16> {
            match op {
                Operation::Assign { wire } =>
                    self.get_signal(&wire),
                Operation::Not { op } =>
                    self.get_operand_value(op).map(|value| !value),
                Operation::And { left, right } =>
                    self.get_operand_value(left).and_then(|l|
                    self.get_operand_value(right).map(|r|
                        l & r
                    )),
                Operation::Or { left, right } =>
                    self.get_operand_value(left).and_then(|l|
                    self.get_operand_value(right).map(|r|
                        l | r
                    )),
                Operation::LShift { left: op, right: shift } =>
                    self.get_operand_value(op).map(|value| value << shift),
                Operation::RShift { left: op, right: shift } =>
                    self.get_operand_value(op).map(|value| value >> shift),
            }
        }

        // }}}
    }

    // }}}
}

// }}}

use circuit::Circuit;

fn main() {
    let mut circuit     = Circuit::new();
    let mut file        = File::open("input.txt").unwrap();
    let mut description = String::new();

    file.read_to_string(&mut description).unwrap();
    circuit.build(&description);
    circuit.emulate();

    println!("After running the circuit, the signal on wire `a` is {}.",
             circuit.get_signal("a").unwrap());
}

// {{{ Tests

#[test]
fn examples_part1() {
    let description = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
    let mut circuit = Circuit::new();

    circuit.build(description);
    circuit.emulate();

    assert_eq!(circuit.get_signal("d"), Some(72));
    assert_eq!(circuit.get_signal("e"), Some(507));
    assert_eq!(circuit.get_signal("f"), Some(492));
    assert_eq!(circuit.get_signal("g"), Some(114));
    assert_eq!(circuit.get_signal("h"), Some(65412));
    assert_eq!(circuit.get_signal("i"), Some(65079));
    assert_eq!(circuit.get_signal("x"), Some(123));
    assert_eq!(circuit.get_signal("y"), Some(456));
}

// }}}
