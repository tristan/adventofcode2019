use std::io;
use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Position {
    Inside,
    Outside
}

#[derive(Debug)]
struct Map {
    entrance: Point,
    exit: Point,
    portals: HashMap<Point, (Position, Point)>,
    paths: HashSet<Point>
}

fn read_input<R: io::Read>(mut reader: R) -> Result<Map, io::Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;

    let mut portal_char_points: HashMap<Point, char> = HashMap::new();
    let mut portals_named: HashMap<String, (Point, Point, Position)> = HashMap::new();
    let mut portals: HashMap<Point, (Position, Point)> = HashMap::new();
    let mut outside: HashSet<Point> = HashSet::new();
    let mut inside: HashSet<Point> = HashSet::new();

    let lines = lines.split("\n").filter(|line| !line.is_empty()).collect::<Vec<_>>();
    let y_len = lines.len() as isize;
    lines.iter().enumerate().for_each(|(y, line)| {
        let y = y as isize;
        let x_len = line.len() as isize;
        line.chars().enumerate().for_each(|(x, ca)| {
            let x = x as isize;
            let this = Point::new(x, y);
            match ca {
                '#' => {},
                '.' => {
                    inside.insert(this);
                },
                ' ' => {
                    outside.insert(this);
                },
                'A'..='Z' => {
                    if x > 0 && y > 0 {
                        let left = Point::new(x - 1, y);
                        let up = Point::new(x, y - 1);
                        if let Some(cb) = portal_char_points.get(&left) {
                            let portal_name = format!("{}{}", cb, ca);
                            let (portal_entrance, portal_exit, portal_position) = if x - 1 > 0 {
                                let next = Point::new(x - 2, y);
                                if outside.contains(&next) {
                                    // portal must be accessible from the right side
                                    (this, Point::new(x + 1, y), Position::Inside)
                                } else if inside.contains(&next) {
                                    // portal is accessible from the left side
                                    (left, next, if x + 1 == x_len {
                                        Position::Outside
                                    } else {
                                        Position::Inside
                                    })
                                } else {
                                    panic!("invalid map: unable to determine entrance of horizontal portal: {} at {}", portal_name, this);
                                }
                            } else {
                                // we're at the edge, so it has to be this
                                (this, Point::new(x + 1, y), Position::Outside)
                            };
                            if let Some((other_entrance, other_exit, other_position)) = portals_named.remove(&portal_name) {
                                portals.insert(portal_entrance, (portal_position, other_exit));
                                portals.insert(other_entrance, (other_position, portal_exit));
                            } else {
                                portals_named.insert(portal_name, (portal_entrance, portal_exit, portal_position));
                            }
                        } else if let Some(cb) = portal_char_points.get(&up) {
                            let portal_name = format!("{}{}", cb, ca);
                            let (portal_entrance, portal_exit, portal_position) = if y - 1 > 0 {
                                let next = Point::new(x, y - 2);
                                if outside.contains(&next) {
                                    // portal must be accessible from the bottom side
                                    (this, Point::new(x, y + 1), Position::Inside)
                                } else if inside.contains(&next) {
                                    // portal is accessible from the top side
                                    (up, next, if y + 1 == y_len {
                                        Position::Outside
                                    } else {
                                        Position::Inside
                                    })
                                } else {
                                    panic!("invalid map: unable to determine entrance of vertical portal {} at {}", portal_name, this);
                                }
                            } else {
                                (this, Point::new(x, y + 1), Position::Outside)
                            };
                            if let Some((other_entrance, other_exit, other_position)) = portals_named.remove(&portal_name) {
                                portals.insert(portal_entrance, (portal_position, other_exit));
                                portals.insert(other_entrance, (other_position, portal_exit));
                            } else {
                                portals_named.insert(portal_name, (portal_entrance, portal_exit, portal_position));
                            }
                        } else {
                            portal_char_points.insert(this, ca);
                        }
                    } else {
                        portal_char_points.insert(this, ca);
                    }
                },
                _ => panic!("unhandled char: {}", ca)
            }
        });
    });
    let entrance = portals_named.get("AA").expect("Unable to find entrance").1;
    let exit = portals_named.get("ZZ").expect("Unable to find exit").1;

    Ok(Map {
        entrance,
        exit,
        portals,
        paths: inside
    })
}

fn part1(map: &Map) -> usize {
    let mut scores: HashMap<Point, usize> = HashMap::new();

    // warp out of entrance
    let deltas = [Point::new(-1, 0), Point::new(0, -1), Point::new(1, 0), Point::new(0, 1)];
    let mut walkers = vec![(map.entrance, 0)];

    while !walkers.is_empty() {
        walkers = walkers.into_iter().map(|(walker, distance)| {
            deltas.iter().filter_map(|&d| {
                let p = walker + d;
                let p = if map.paths.contains(&p) {
                    p
                } else if let Some(&(_, exit)) = map.portals.get(&p) {
                    exit
                } else {
                    return None;
                };
                if let Some(_) = scores.get(&p) {
                    None
                } else {
                    scores.insert(p, distance + 1);
                    Some((p, distance + 1))
                }
            }).collect::<Vec<_>>()
        }).flatten().collect();
    }

    *scores.get(&map.exit).expect("Missing score for exit")
}

fn part2(map: &Map) -> usize {

    let mut level_scores: HashMap<usize, HashMap<Point, usize>> = HashMap::new();

    // warp out of entrance
    let deltas = [Point::new(-1, 0), Point::new(0, -1), Point::new(1, 0), Point::new(0, 1)];
    let mut walkers = vec![(map.entrance, 0, 0)];
    level_scores.entry(0).or_default().insert(map.entrance, 0);

    while !walkers.is_empty() {
        let mut new_walkers = vec![];
        for (walker, distance, level) in walkers {
            if level == 0 && walker == map.exit {
                return distance;
            }
            new_walkers.extend(deltas.iter().filter_map(|&d| {
                let p = walker + d;
                if map.paths.contains(&p) {
                    let scores = level_scores.entry(level).or_default();
                    if let Some(_) = scores.get(&p) {
                        None
                    } else {
                        scores.insert(p, distance + 1);
                        Some((p, distance + 1, level))
                    }
                } else if let Some((position, exit)) = map.portals.get(&p) {
                    let (scores, level) = match position {
                        Position::Outside => {
                            if level == 0 {
                                return None;
                            }
                            (level_scores.entry(level - 1).or_default(), level - 1)
                        },
                        Position::Inside => {
                            (level_scores.entry(level + 1).or_default(), level + 1)
                        }
                    };
                    if let Some(_) = scores.get(exit) {
                        None
                    } else {
                        scores.insert(*exit, distance + 1);
                        Some((*exit, distance + 1, level))
                    }
                } else {
                    None
                }
            }).collect::<Vec<_>>());
        }
        walkers = new_walkers;
    }

    panic!("not found!");
}

fn main() -> Result<(), io::Error> {
    let map = read_input(fs::File::open("day_20_input.txt")?)?;
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
        let input = "                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";

        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part1(&map), 58);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), io::Error> {
        let input = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     ";


        let map = read_input(io::Cursor::new(input))?;
        assert_eq!(part2(&map), 396);
        Ok(())
    }
}
