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
// {{{ Factory

mod factory {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    use instruction::Instruction;

    // A bot's output.
    //
    // A bot can connected to another bot or to an output bin.
    #[derive(Clone, Copy, Debug)]
    pub enum Output {
        Bot(u32),
        Bin(u32)
    }

    // Represent an action.
    //
    // It can be used to keep track of which bot compares which values.
    #[derive(Debug)]
    pub struct Action {
        pub bot:  u32,
        pub low:  i32,
        pub high: i32,
    }

    // {{{ Bot

    // A factory's bot.
    //
    // A bot is connected to two outputs.
    // At first, it doesn't hold any data, but as its state change it can hold
    // up to two value.
    // When that happen, the values are dispatched through its output
    // connections.
    #[derive(Clone, Copy, Debug)]
    struct Bot {
        id:    u32,
        low:   Output,
        high:  Output,
        v1:    Option<i32>,
        v2:    Option<i32>,
    }

    impl Bot {
        // Create a bot with no value, connected to two outputs.
        pub fn new(id: u32, low: Output, high: Output) -> Self {
            Bot { id: id, low: low, high: high, v1: None, v2: None }
        }

        // Take a value.
        //
        // This will panic if the bot is already full.
        pub fn take(&mut self, value: i32) {
            assert!(!self.is_full());
            if self.v1.is_none() {
                self.v1 = Some(value);
            } else {
                self.v2 = Some(value);
            }
        }

        // Check if the bot is full (i.e: already has two values).
        pub fn is_full(&self) -> bool {
            self.v2.is_some()
        }
    }

    // }}}
    // {{{ Factory

    // A factory.
    //
    // A factory contains a set of bots and output bins that are identified by
    // a unique ID.
    #[derive(Debug)]
    pub struct Factory {
        bots: HashMap<u32, Bot>,
        bins: HashMap<u32, Option<i32>>,
        start_bot: Option<u32>,
    }

    impl Factory {
        // Setup a factory from a list of instructions.
        //
        // The instructions are used to:
        // - create the bots and the output bins
        // - create the connections
        // - give an initial value to some bots
        pub fn new(instructions : &[Instruction]) -> Self {
            let mut fact = Factory {
                bots: HashMap::new(),
                bins: HashMap::new(),
                start_bot: None,
            };
            // 1. Create and connect the bots and output bins.
            for instr in instructions {
                if let Instruction::Connect { bot, lo, hi } = *instr {
                    // Create bot.
                    assert_eq!(fact.bots.contains_key(&bot), false);
                    fact.bots.insert(bot, Bot::new(bot, lo, hi));
                    // Create output bins, if any.
                    for out in &[lo, hi] {
                        if let Output::Bin(id) = *out {
                            assert_eq!(fact.bins.contains_key(&id), false);
                            fact.bins.insert(id, None);
                        }
                    }
                }
            }
            // 2. Give initial value to some bots.
            for instr in instructions {
                if let Instruction::GiveValue{ bot: bot_id, value } = *instr {
                    // At this point, bot must exists!
                    let bot = fact.bots.get_mut(&bot_id).unwrap();
                    bot.take(value);
                    // If bot has two values this is the starting point.
                    if bot.is_full() {
                        // We should have a single starting point.
                        assert!(fact.start_bot.is_none());
                        fact.start_bot = Some(bot_id);
                    }
                }
            }
            fact
        }

        // Run the factory until all bots are in a stable state.
        //
        // A log of the actions performed is returned.
        pub fn run(&mut self) -> Vec<Action> {
            // We must have a starting bot to run the simulation!
            let start_bot = self.start_bot.unwrap();
            let mut queue = VecDeque::new();
            let mut log   = Vec::new();

            queue.push_back(start_bot);
            while let Some(bot_id) = queue.pop_front() {
                // At this point, bot must exists!
                let mut bot = self.bots.remove(&bot_id).unwrap();
                if bot.is_full() {
                    // Add the output bots, if any, to the queue.
                    for out in &[bot.low, bot.high] {
                        if let Output::Bot(out_id) = *out {
                            queue.push_back(out_id);
                        }
                    }
                    // Dispatch the values and log the action.
                    log.push(self.dispatch(&mut bot));
                    self.bots.insert(bot.id, bot);
                }
            };
            log
        }

        // Dispatch the values of `bot` throught its output connections.
        fn dispatch(&mut self, bot: &mut Bot) -> Action {
            assert!(bot.is_full());
            let (v1, v2) = (bot.v1.take().unwrap(), bot.v2.take().unwrap());
            let (lo, hi) = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            self.forward_value(&mut bot.low,  lo);
            self.forward_value(&mut bot.high, hi);
            Action { bot: bot.id, low: lo, high: hi }
        }

        pub fn get_bin_value(&self, id: u32) -> Option<i32> {
            self.bins[&id]
        }

        // Forward the value `val` into `out`.
        fn forward_value(&mut self, out: &mut Output, val: i32) {
            match *out {
                Output::Bin(id) => {
                    let bin = self.bins.get_mut(&id).unwrap();
                    assert!(bin.is_none());
                    *bin = Some(val);
                },
                Output::Bot(id) => {
                    let bot = self.bots.get_mut(&id).unwrap();
                    bot.take(val);
                },
            };
        }
    }

    // }}}
}

// }}}
// {{{ Instruction

mod instruction {
    use std::str;
    use nom;

    use factory::Output;

    #[derive(Clone, Copy, Debug)]
    pub enum Instruction {
        GiveValue { bot: u32, value: i32 },
        Connect   { bot: u32, lo: Output, hi: Output }
    }

    impl str::FromStr for Instruction {
        type Err = nom::ErrorKind;

        /// Builds an `Instruction` from a string.
        fn from_str(s: &str) -> Result<Instruction, nom::ErrorKind> {
            parse_instruction(s.as_bytes()).to_result()
        }
    }

    // {{{ Parser

    /// Parse the description of the output.
    named!(output<&[u8], Output>,
        do_parse!(
            dst: alt!(tag!("bot ") | tag!("output ")) >>
            id:  map_res!(nom::digit, str::from_utf8)  >>
            (if dst == b"bot " {
                Output::Bot(id.parse::<u32>().unwrap())
            } else {
                Output::Bin(id.parse::<u32>().unwrap())
            })
        )
    );

    /// Parse the instruction to give a value to a bot.
    named!(give_value<&[u8], (u32, i32)>,
        do_parse!(
           tag!("value ")                             >>
           val: map_res!(nom::digit, str::from_utf8)  >>
           tag!(" goes to bot ")                      >>
           bot: map_res!(nom::digit, str::from_utf8)  >>
           (bot.parse::<u32>().unwrap(), val.parse::<i32>().unwrap())
       )
    );

    /// Parse the instruction that connect a bot to its output.
    named!(connect<&[u8], (u32, Output, Output)>,
        do_parse!(
           tag!("bot ") >>
           bot: map_res!(nom::digit, str::from_utf8)  >>
           tag!(" gives low to ") >>
           low: output >>
           tag!(" and high to ") >>
           high: output >>
           (bot.parse::<u32>().unwrap(), low, high)
       )
    );

    /// Parse an instruction.
    named!(parse_instruction<&[u8], Instruction>,
        alt!(
            give_value => {
                |(b, v)| Instruction::GiveValue { bot: b, value: v }
            }
          | connect => {
                |(b, l, h)| Instruction::Connect { bot: b, lo: l, hi: h }
            }
        )
    );

    // }}}
}

// }}}

use factory::Factory;
use instruction::Instruction;

fn output_product(fact: &Factory, ids: &[u32]) -> i32 {
    ids.iter().fold(1, |product, id| product * fact.get_bin_value(*id).unwrap())
}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap())
                                    .collect::<Vec<_>>();

    let mut fact = Factory::new(&instructions);
    let log = fact.run();
    if let Some(action) = log.iter().find(|a| a.low == 17 && a.high == 61) {
        println!("Bot {} compared the value 17 and 61.", action.bot);
    }
    println!("The product of the output bins 0, 1 and 2 is {}.",
             output_product(&fact, &[0, 1, 2]));
}

// {{{ Tests

#[test]
fn examples_part1() {
    let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap())
                                    .collect::<Vec<_>>();

    let mut fact = Factory::new(&instructions);
    fact.run();
    assert_eq!(fact.get_bin_value(0), Some(5));
    assert_eq!(fact.get_bin_value(1), Some(2));
    assert_eq!(fact.get_bin_value(2), Some(3));
}

#[test]
fn examples_part2() {
    let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap())
                                    .collect::<Vec<_>>();

    let mut fact = Factory::new(&instructions);
    fact.run();
    assert_eq!(output_product(&fact, &[0, 1, 2]), 30);
}

// }}}
