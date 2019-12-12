use std::fs;
use std::io::{self, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    InvalidLine(String),
    InputHasMultipleDirectOrbits(String),
    MissingObject(String)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}


struct Node {
    name: String,
    parent: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn count_orbits(&self) -> usize {
        match &self.parent {
            Some(p) => {
                1 + p.borrow().count_orbits()
            },
            None => {
                0
            }
        }
    }

    fn path_to(&self) -> HashSet<String> {
        match &self.parent {
            Some(p) => {
                let mut a = HashSet::new();
                a.insert(p.borrow().name.clone());
                let b = p.borrow().path_to();
                b.into_iter().for_each(|x| { a.insert(x); });
                a
            },
            None => {
                HashSet::new()
            }
        }
    }
}

type Tree = HashMap<String, Rc<RefCell<Node>>>;

fn read_input<R: io::Read>(mut reader: R) -> Result<Tree, Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    let pairs = lines.trim().split("\n").map(|line| {
        let mut parts = line.split(")");
        let a = parts.next().ok_or_else(|| Error::InvalidLine(line.to_string()))?;
        let b = parts.next().ok_or_else(|| Error::InvalidLine(line.to_string()))?;
        Ok((a.to_string(), b.to_string()))
    }).collect::<Result<Vec<(String, String)>, Error>>()?;
    let mut tree = Tree::new();
    for (a, b) in pairs.into_iter() {
        let node_a = match tree.get(&a) {
            Some(n) => n.clone(),
            None => {
                let n = Rc::new(RefCell::new(Node { name: a.to_string(), parent: None }));
                tree.insert(a, n.clone());
                n
            }
        };
        match tree.get(&b) {
            Some(n) => {
                if n.borrow().parent.is_none() {
                    n.borrow_mut().parent = Some(node_a.clone());
                } else {
                    return Err(Error::InputHasMultipleDirectOrbits(b.to_string()));
                }
            },
            None => {
                let n = Rc::new(RefCell::new(Node { name: b.to_string(), parent: Some(node_a.clone()) }));
                tree.insert(b, n.clone());
            }
        };
    }
    Ok(tree)
}

fn calculate_checksum(tree: &Tree) -> usize {
    tree.iter().map(|(_k, n)| {
        let orbits = n.borrow().count_orbits();
        orbits
    }).sum()
}

fn calculate_transfers(tree: &Tree) -> Result<usize, Error> {
    let you = tree.get("YOU")
        .ok_or_else(|| Error::MissingObject("YOU".to_string()))?;
    let san = tree.get("SAN")
        .ok_or_else(|| Error::MissingObject("SAN".to_string()))?;
    let you_path = you.borrow().path_to();
    let san_path = san.borrow().path_to();
    let result = you_path.symmetric_difference(&san_path).count();

    Ok(result)
}

fn main() -> Result<(), Error> {
    let file = fs::File::open("day_06_input.txt")?;
    let reader = BufReader::new(file);
    let tree = read_input(reader)?;

    println!("Part1: {}", calculate_checksum(&tree));
    println!("Part2: {}", calculate_transfers(&tree)?);
    Ok(())
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() -> Result<(), Error> {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let reader = io::Cursor::new(input.as_bytes());
        let tree = read_input(reader)?;

        assert_eq!(calculate_checksum(&tree), 42);

        Ok(())
    }


    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let reader = io::Cursor::new(input.as_bytes());
        let tree = read_input(reader)?;

        assert_eq!(calculate_transfers(&tree)?, 4);

        Ok(())
    }
}
