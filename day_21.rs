use common::intcode::{IntcodeComputer, Error, read_program};

const NOT: &str = "NOT";
const AND: &str = "AND";
const OR: &str = "OR";
const WALK: &str = "WALK";
const RUN: &str = "RUN";
const A: &str = "A";
#[allow(dead_code)]
const B: &str = "B";
const C: &str = "C";
const D: &str = "D";
#[allow(dead_code)]
const E: &str = "E";
#[allow(dead_code)]
const F: &str = "F";
#[allow(dead_code)]
const G: &str = "G";
#[allow(dead_code)]
const H: &str = "H";
#[allow(dead_code)]
const I: &str = "I";
const J: &str = "J";
const T: &str = "T";

macro_rules! springcode {
    ( $( $op:ident $x:ident $y:ident );*; !$exec:ident ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push(format!("{} {} {}\n", $op, $x, $y));
        )*
        temp_vec.push(format!("{}\n", $exec));
        temp_vec.join("")
    }}
}

fn main() -> Result<(), Error> {
    let program = read_program("day_21_input.txt")?;
    let mut comp = IntcodeComputer::new(&program);

    let springcode = springcode!(
        // jump if A is a hole
        // ###...###
        //   ^ABCD
        NOT A J; // J = true
        // jump if C is a hole and D is not
        // #####..#.########
        //    ^ABCD
        NOT C T; // T = true if C is a hole
        AND D T; // set T = true if D is not a hole and C is a hole
        OR  T J; // combine with previous
        !WALK
    );

    comp.send_ascii(&springcode)?;
    comp.run()?;

    comp.print_output_ascii();
    println!("");

    comp.reset();

    let springcode = springcode!(
        // JUMP if there is a hole and the landing is safe
        NOT A J; // J is true if A is a hole
        NOT B T; // T is true if B is a hole
        OR T J;  // J is true if A or B is a hole
        NOT C T; // T is true if C is a hole
        OR T J;  // J is true if A or B or C is a hole
        AND D J; // J is true if (A or B or C is a hole) and D is safe

        // if E is a hole, and H is a hole, abort jump
        NOT E T; // T is true if E is a hole
        AND H T; // T is true if E is a hole and H is safe
        OR E T; // T is true if E is safe or (E is a hole and H is safe)
        AND T J;

        !RUN
    );

    comp.send_ascii(&springcode)?;
    comp.run()?;

    comp.print_output_ascii();
    println!("");

    Ok(())
}
