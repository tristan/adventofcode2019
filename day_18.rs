use std::io;
use std::fs;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

#[derive(Copy, Clone, PartialEq)]
enum Point {
    Wall,
    Empty,
    Start,
    Key(char),
    Door(char),
    Covered
}

fn read_input<R: io::Read>(mut reader: R) -> Result<Vec<Vec<Point>>, io::Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    Ok(lines.trim().split("\n").map(|line| {
        line.trim().chars()
            .map(|c| {
                match c {
                    '#' => Point::Wall,
                    '.' => Point::Empty,
                    '@' => Point::Start,
                    'a'..='z' => Point::Key(c),
                    'A'..='Z' => Point::Door(c),
                    _ => panic!("unhandled char: {}", c)
                }
            })
            .collect()
    }).collect())
}

fn find_start_pos(map: &Vec<Vec<Point>>) -> Option<(usize, usize)> {
    for (y, v) in map.iter().enumerate() {
        for (x, p) in v.iter().enumerate() {
            if *p == Point::Start {
                return Some((x, y));
            }
        }
    }
    None
}

fn find_key_positions(map: &Vec<Vec<Point>>) -> HashMap<char, (usize, usize)> {
    let mut keys = HashMap::new();
    for (y, v) in map.iter().enumerate() {
        for (x, p) in v.iter().enumerate() {
            match p {
                Point::Key(k) => {
                    keys.insert(*k, (x, y));
                },
                _ => ()
            }
        }
    }
    keys
}

struct Leaf {
    x: usize,
    y: usize,
    distance: usize,
    doors_passed: Vec<char>,
    keys_collected: Vec<char>
}

enum NewLeafResult {
    Leaf(Leaf),
    Key(Leaf, char),
    None
}

fn create_new_leaf(
    x: usize, y: usize,
    map: &Vec<Vec<Point>>,
    distance: usize,
    doors_passed: &Vec<char>,
    keys_collected: &Vec<char>
) -> NewLeafResult {
    match map[y][x] {
        Point::Wall | Point::Covered => {
            // no new leaf
            NewLeafResult::None
        },
        Point::Key(k) => {
            NewLeafResult::Key(Leaf {
                x, y,
                distance: distance + 1,
                doors_passed: doors_passed.clone(),
                keys_collected: {
                    let mut keys = keys_collected.clone();
                    if !keys.contains(&k) {
                        keys.push(k);
                    }
                    keys
                }
            }, k)
        },
        Point::Door(d) => {
            let k = d.to_lowercase().next().unwrap();
            NewLeafResult::Leaf(Leaf {
                x, y,
                distance: distance + 1,
                doors_passed: {
                    let mut doors = doors_passed.clone();
                    if !doors.contains(&k) {
                        doors.push(k);
                    }
                    doors
                },
                keys_collected: keys_collected.clone()
            })

        },
        Point::Empty | Point::Start => {
            NewLeafResult::Leaf(Leaf {
                x, y,
                distance: distance + 1,
                doors_passed: doors_passed.clone(),
                keys_collected: keys_collected.clone()
            })
        }
    }
}

fn tree_from_point(mut map: Vec<Vec<Point>>, point: (usize, usize)) -> HashMap<char, (usize, Vec<char>, Vec<char>)> {
    map[point.1][point.0] = Point::Covered;
    let mut leaves = vec![Leaf {
        x: point.0, y: point.1,
        distance: 0, doors_passed: Vec::new(), keys_collected: Vec::new()
    }];
    let mut nodes: HashMap<char, (usize, Vec<char>, Vec<char>)> = HashMap::new();
    while !leaves.is_empty() {
        let mut new_leaves = vec![];
        for leaf in &leaves {
            if leaf.x > 0 {
                match create_new_leaf(leaf.x - 1, leaf.y, &map, leaf.distance, &leaf.doors_passed, &leaf.keys_collected) {
                    NewLeafResult::Key(leaf, k) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        nodes.insert(k, (leaf.distance, leaf.doors_passed.clone(), leaf.keys_collected.clone()));
                        new_leaves.push(leaf);
                    },
                    NewLeafResult::Leaf(leaf) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        new_leaves.push(leaf);
                    }
                    NewLeafResult::None => ()
                };
            }
            if leaf.x < map[0].len() - 1 {
                match create_new_leaf(leaf.x + 1, leaf.y, &map, leaf.distance, &leaf.doors_passed, &leaf.keys_collected) {
                    NewLeafResult::Key(leaf, k) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        nodes.insert(k, (leaf.distance, leaf.doors_passed.clone(), leaf.keys_collected.clone()));
                        new_leaves.push(leaf);
                    },
                    NewLeafResult::Leaf(leaf) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        new_leaves.push(leaf);
                    }
                    NewLeafResult::None => ()
                };
            }
            if leaf.y > 0 {
                match create_new_leaf(leaf.x, leaf.y - 1, &map, leaf.distance, &leaf.doors_passed, &leaf.keys_collected) {
                    NewLeafResult::Key(leaf, k) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        nodes.insert(k, (leaf.distance, leaf.doors_passed.clone(), leaf.keys_collected.clone()));
                        new_leaves.push(leaf);
                    },
                    NewLeafResult::Leaf(leaf) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        new_leaves.push(leaf);
                    }
                    NewLeafResult::None => ()
                };
            }
            if leaf.y < map.len() - 1 {
                match create_new_leaf(leaf.x, leaf.y + 1, &map, leaf.distance, &leaf.doors_passed, &leaf.keys_collected) {
                    NewLeafResult::Key(leaf, k) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        nodes.insert(k, (leaf.distance, leaf.doors_passed.clone(), leaf.keys_collected.clone()));
                        new_leaves.push(leaf);
                    },
                    NewLeafResult::Leaf(leaf) => {
                        map[leaf.y][leaf.x] = Point::Covered;
                        new_leaves.push(leaf);
                    }
                    NewLeafResult::None => ()
                };
            }
        }
        leaves = new_leaves;
    }
    nodes
}

fn find_shortest_distance_between_keys(map: Vec<Vec<Point>>, start_pos: (usize, usize)) -> (Vec<char>, usize) {

    // for row in &map {
    //     for c in row {
    //         print!("{}", match c {
    //             Point::Covered => '.',
    //             Point::Door(c) => *c,
    //             Point::Empty => '.',
    //             Point::Key(c) => *c,
    //             Point::Start => '@',
    //             Point::Wall => '#'
    //         });
    //     }
    //     println!("");
    // }
    // println!("");

    let start_node = tree_from_point(map.clone(), start_pos);
    let reachable_keys = start_node.keys().map(|k| *k).collect::<Vec<_>>();

    let keys = find_key_positions(&map);
    let mut nodes: HashMap<char, HashMap<char, (usize, Vec<char>, Vec<char>)>> = HashMap::new();
    keys.iter().for_each(|(k, pos)| {
        if reachable_keys.contains(k) {
            let children = tree_from_point(map.clone(), *pos);
            nodes.insert(*k, children);
        }
    });

    let mut queue: BinaryHeap<Reverse<(usize, char, Vec<char>)>> = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse((0, '@', Vec::new())));
    nodes.insert('@', start_node);

    while let Some(Reverse((dist_u, u, keys))) = queue.pop() {
        //println!(">>> u: {} -> {}: {}", u, dist_u, keys.iter().map(|c| *c).collect::<String>());
        if keys.len() == nodes.len() - 1 {
            return (keys, dist_u);
        }
        // make sure we don't process a node for the same set of keys twice
        let mut sorted = keys.clone();
        sorted.sort();
        if seen.insert((u, sorted)) {
            // find keys that can be taken
            let node_u = nodes.get(&u).unwrap();
            node_u.iter().for_each(|(v, (length_u_v, doors_u_v, keys_u_v))| {
                // doors.filter = for part2, removes doors that we can't get the keys for
                if !keys.contains(v) && doors_u_v.iter().filter(|d| reachable_keys.contains(d)).all(|d| keys.contains(d)) {
                    // println!("v: {} -> {}: {}, {}", v, length_u_v,
                    //          keys_u_v.iter().map(|c| *c).collect::<String>(),
                    //          doors_u_v.iter().map(|c| *c).collect::<String>());
                    let mut keys_s_u = keys.clone();
                    keys_u_v.iter().for_each(|k| {
                        if !keys_s_u.contains(k) {
                            keys_s_u.push(*k);
                        }
                    });
                    queue.push(Reverse((dist_u + length_u_v, *v, keys_s_u)));
                }
            });
        }
    }

    (Vec::new(), 0)
}

fn part1(map: &Vec<Vec<Point>>) -> usize {
    let start_pos = find_start_pos(map).unwrap();
    let (_, dist) = find_shortest_distance_between_keys(map.clone(), start_pos);
    dist
}


fn part2(map: &Vec<Vec<Point>>) -> usize {
    let start_pos = find_start_pos(map).unwrap();
    let mut map = map.clone();
    let start_positions = [(-1isize, -1isize), (-1, 1), (1, -1), (1, 1)].iter().map(|(dx, dy)| {
        let y = (start_pos.1 as isize + dy) as usize;
        let x = (start_pos.0 as isize + dx) as usize;
        map[y][x] = Point::Start;
        (x, y)
    }).collect::<Vec<(usize, usize)>>();
    [(0isize, 0isize), (1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|(dx, dy)| {
        let y = (start_pos.1 as isize + dy) as usize;
        let x = (start_pos.0 as isize + dx) as usize;
        map[y][x] = Point::Wall;
    });

    start_positions.iter().map(|&pos| {
        let (_keys, dist) = find_shortest_distance_between_keys(map.clone(), pos);
        //dbg!(dist, _keys);
        dist
    }).sum()
}

fn main() -> Result<(), io::Error> {
    let map = read_input(fs::File::open("day_18_input.txt")?)?;
    let s = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(&map), s.elapsed());
    let s = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(&map), s.elapsed());
    Ok(())
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_one() -> Result<(), io::Error> {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part1(&map), 132);
        //assert_eq!(keys, ['b', 'a', 'c', 'd', 'f', 'e', 'g']);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), io::Error> {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part1(&map), 136);
        //assert_eq!(keys, ['a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm']);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), io::Error> {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part1(&map), 81);
        //assert_eq!(keys, ['a', 'c', 'f', 'i', 'd', 'g', 'b', 'e', 'h']);

        Ok(())
    }

    #[test]
    fn test_four() -> Result<(), io::Error> {
        let input = "#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";

        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part2(&map), 32);
        Ok(())
    }

    #[test]
    fn test_five() -> Result<(), io::Error> {
        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

        // @TODO: fix this test!
        // the solution to the real problem works because none of the
        // doors in each segment really depend on the keys from the others
        // but in this test case it does!
        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part2(&map), 72);
        Ok(())
    }

}
