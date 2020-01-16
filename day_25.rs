use common::intcode2::{IntcodeComputer, Signal, Error, read_program};
use std::io;
use std::collections::HashSet;
use itertools::Itertools;

enum State {
    Items,
    Doors,
    Command,
    Other
}

fn main() -> Result<(), Error> {
    let code = read_program("day_25_input.txt")?;

    let mut comp = IntcodeComputer::new(&code);

    // comp.send_ascii("north\n");
    // comp.send_ascii("take giant electromagnet\n"); // can't move
    // comp.send_ascii("south\n");

    comp.send_ascii("south\n");
    comp.send_ascii("take astronaut ice cream\n"); // too light
    comp.send_ascii("north\n");

    comp.send_ascii("east\n");

    comp.send_ascii("take mouse\n");  // too light

    // get spool
    comp.send_ascii("north\n");
    comp.send_ascii("take spool of cat6\n");

    // get infinite loop
    // comp.send_ascii("west\n");
    // comp.send_ascii("north\n");
    // comp.send_ascii("take infinite loop\n"); // infinite loop!
    // comp.send_ascii("south\n");
    // comp.send_ascii("east\n");

    // get hypercube
    comp.send_ascii("north\n");
    comp.send_ascii("take hypercube\n");
    // get sand
    comp.send_ascii("east\n");
    comp.send_ascii("take sand\n");

    // get antenna
    comp.send_ascii("south\n");
    comp.send_ascii("take antenna\n");
    comp.send_ascii("north\n");

    // exit crew quarters
    comp.send_ascii("west\n");

    // exit engineering
    comp.send_ascii("south\n");

    // exit arcade
    comp.send_ascii("south\n");

    comp.send_ascii("south\n");
    comp.send_ascii("take mutex\n"); // too light
    comp.send_ascii("west\n");
    comp.send_ascii("take boulder\n"); // too light
    comp.send_ascii("south\n");
    //comp.send_ascii("take escape pod\n"); // LAUCHED INTO SPACE
    comp.send_ascii("south\n");
    //comp.send_ascii("take photons\n"); // eaten by a Grue!
    comp.send_ascii("south\n");
    //comp.send_ascii("take molten lava\n"); // You melt!
    comp.send_ascii("west\n");
    comp.send_ascii("south\n");
    //comp.send_ascii("south\n");

    //comp.send_ascii("inv\n");

    let mut linebuffer = vec![];
    let mut room = String::new();
    let mut inv = HashSet::new();
    let mut doors = HashSet::new();
    let mut items = HashSet::new();

    let mut all_items = HashSet::new();
    [
        "hypercube".to_string(),
        "sand".to_string(),
        "astronaut ice cream".to_string(),
        "spool of cat6".to_string(),
        "mouse".to_string(),
        "mutex".to_string(),
        "boulder".to_string(),
        "antenna".to_string(),
    ].into_iter().for_each(|i| { all_items.insert(i.clone()); });
    let mut perm_num = 1;
    let mut perm_iter = all_items.iter().permutations(1);

    let mut state = State::Other;

    loop {
        let v = comp.run()?;

        match v {
            Signal::Output(c) => {
                let c = std::char::from_u32(c as _).unwrap();
                print!("{}", c);
                if c == '\n' {
                    let s: String = linebuffer.iter().collect();
                    if s.starts_with("==") && s.ends_with("==") {
                        room = s[3..(s.len() - 3)].to_string();
                        doors = HashSet::new();
                        items = HashSet::new();
                    } else if s.starts_with("You take the ") {
                        let item = s[13..(s.len() - 1)].to_string();
                        items.remove(&item);
                        inv.insert(item);
                    } else if s.starts_with("You drop the ") {
                        let item = s[13..(s.len() - 1)].to_string();
                        inv.remove(&item);
                        items.insert(item);
                    } else if s == "Doors here lead:" {
                        state = State::Doors;
                    } else if s == "Items here:" {
                        state = State::Items;
                    } else if s.starts_with("- ") {
                        let item = s[2..].to_string();
                        match state {
                            State::Doors => {
                                doors.insert(item);
                            },
                            State::Items => {
                                items.insert(item);
                            },
                            _ => {

                            }
                        }
                    } else if s == "Command?" {
                        state = State::Command;
                    }
                    linebuffer = vec![];
                } else {
                    linebuffer.push(c);
                }
            },
            Signal::ExpectingInput => {
                if room == "Security Checkpoint" {
                    let items_to_try: HashSet<String> = match perm_iter.next() {
                        Some(v) => v.into_iter().cloned().collect(),
                        None => {
                            perm_num += 1;
                            if perm_num > all_items.len() {
                                panic!("got to end without finding a match!");
                            }
                            perm_iter = all_items.iter().permutations(perm_num);
                            perm_iter.next().unwrap().into_iter().cloned().collect()
                        }
                    };
                    print!(">>> Trying: ");
                    items_to_try.iter().for_each(|i| print!("{}, ", i));
                    println!("");
                    for item_to_drop in inv.difference(&items_to_try) {
                        comp.send_ascii(&format!("drop {}\n", item_to_drop));
                    }
                    for item_to_take in items_to_try.difference(&inv) {
                        comp.send_ascii(&format!("take {}\n", item_to_take));
                    }
                    comp.send_ascii("south\n");

                } else {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)
                        .ok().expect("Couldn't read input");
                    comp.send_ascii(&input);
                }
            },
            Signal::Exiting => {
                break;
            },
            v => {
                panic!("Unexpected signal: {:?}", v);
            }
        }
    }

    Ok(())
}
