use std::ops::AddAssign;
use regex::Regex;
use common::utils::lcm;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
struct Body {
    pos: Vec3,
    vel: Vec3
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct System {
    a: Body,
    b: Body,
    c: Body,
    d: Body,
    step: isize
}

impl System {

    fn load(s: &str) -> System {
        let re: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        let bodies = s.trim().split("\n").map(|s| {
            re.captures(s).map(|caps| {
                // unwrap is safe because we trust regex
                let x = caps[1].parse::<isize>().unwrap();
                let y = caps[2].parse::<isize>().unwrap();
                let z = caps[3].parse::<isize>().unwrap();
                Body { pos: Vec3 { x, y, z }, vel: Vec3::default() }
            }).unwrap()
        }).collect::<Vec<_>>();
        System {
            a: bodies[0],
            b: bodies[1],
            c: bodies[2],
            d: bodies[3],
            step: 0
        }
    }

    fn step(&mut self) {

        // a -> b
        if self.a.pos.x > self.b.pos.x {
            self.a.vel.x -= 1;
            self.b.vel.x += 1;
        } else if self.a.pos.x < self.b.pos.x {
            self.a.vel.x += 1;
            self.b.vel.x -= 1;
        }
        if self.a.pos.y > self.b.pos.y {
            self.a.vel.y -= 1;
            self.b.vel.y += 1;
        } else if self.a.pos.y < self.b.pos.y {
            self.a.vel.y += 1;
            self.b.vel.y -= 1;
        }

        if self.a.pos.z > self.b.pos.z {
            self.a.vel.z -= 1;
            self.b.vel.z += 1;
        } else if self.a.pos.z < self.b.pos.z {
            self.a.vel.z += 1;
            self.b.vel.z -= 1;
        }

        // a -> c
        if self.a.pos.x > self.c.pos.x {
            self.a.vel.x -= 1;
            self.c.vel.x += 1;
        } else if self.a.pos.x < self.c.pos.x {
            self.a.vel.x += 1;
            self.c.vel.x -= 1;
        }
        if self.a.pos.y > self.c.pos.y {
            self.a.vel.y -= 1;
            self.c.vel.y += 1;
        } else if self.a.pos.y < self.c.pos.y {
            self.a.vel.y += 1;
            self.c.vel.y -= 1;
        }

        if self.a.pos.z > self.c.pos.z {
            self.a.vel.z -= 1;
            self.c.vel.z += 1;
        } else if self.a.pos.z < self.c.pos.z {
            self.a.vel.z += 1;
            self.c.vel.z -= 1;
        }

        // a -> d
        if self.a.pos.x > self.d.pos.x {
            self.a.vel.x -= 1;
            self.d.vel.x += 1;
        } else if self.a.pos.x < self.d.pos.x {
            self.a.vel.x += 1;
            self.d.vel.x -= 1;
        }
        if self.a.pos.y > self.d.pos.y {
            self.a.vel.y -= 1;
            self.d.vel.y += 1;
        } else if self.a.pos.y < self.d.pos.y {
            self.a.vel.y += 1;
            self.d.vel.y -= 1;
        }

        if self.a.pos.z > self.d.pos.z {
            self.a.vel.z -= 1;
            self.d.vel.z += 1;
        } else if self.a.pos.z < self.d.pos.z {
            self.a.vel.z += 1;
            self.d.vel.z -= 1;
        }

        // b -> c
        if self.b.pos.x > self.c.pos.x {
            self.b.vel.x -= 1;
            self.c.vel.x += 1;
        } else if self.b.pos.x < self.c.pos.x {
            self.b.vel.x += 1;
            self.c.vel.x -= 1;
        }
        if self.b.pos.y > self.c.pos.y {
            self.b.vel.y -= 1;
            self.c.vel.y += 1;
        } else if self.b.pos.y < self.c.pos.y {
            self.b.vel.y += 1;
            self.c.vel.y -= 1;
        }

        if self.b.pos.z > self.c.pos.z {
            self.b.vel.z -= 1;
            self.c.vel.z += 1;
        } else if self.b.pos.z < self.c.pos.z {
            self.b.vel.z += 1;
            self.c.vel.z -= 1;
        }

        // b -> d
        if self.b.pos.x > self.d.pos.x {
            self.b.vel.x -= 1;
            self.d.vel.x += 1;
        } else if self.b.pos.x < self.d.pos.x {
            self.b.vel.x += 1;
            self.d.vel.x -= 1;
        }
        if self.b.pos.y > self.d.pos.y {
            self.b.vel.y -= 1;
            self.d.vel.y += 1;
        } else if self.b.pos.y < self.d.pos.y {
            self.b.vel.y += 1;
            self.d.vel.y -= 1;
        }

        if self.b.pos.z > self.d.pos.z {
            self.b.vel.z -= 1;
            self.d.vel.z += 1;
        } else if self.b.pos.z < self.d.pos.z {
            self.b.vel.z += 1;
            self.d.vel.z -= 1;
        }

        // c -> d
        if self.c.pos.x > self.d.pos.x {
            self.c.vel.x -= 1;
            self.d.vel.x += 1;
        } else if self.c.pos.x < self.d.pos.x {
            self.c.vel.x += 1;
            self.d.vel.x -= 1;
        }
        if self.c.pos.y > self.d.pos.y {
            self.c.vel.y -= 1;
            self.d.vel.y += 1;
        } else if self.c.pos.y < self.d.pos.y {
            self.c.vel.y += 1;
            self.d.vel.y -= 1;
        }

        if self.c.pos.z > self.d.pos.z {
            self.c.vel.z -= 1;
            self.d.vel.z += 1;
        } else if self.c.pos.z < self.d.pos.z {
            self.c.vel.z += 1;
            self.d.vel.z -= 1;
        }

        self.a.pos += self.a.vel;
        self.b.pos += self.b.vel;
        self.c.pos += self.c.vel;
        self.d.pos += self.d.vel;

        self.step += 1;
    }

    fn energy(&self) -> isize {
        [self.a, self.b, self.c, self.d].iter().map(|Body { pos, vel }| {
            let pot = pos.x.abs() + pos.y.abs() + pos.z.abs();
            let kin = vel.x.abs() + vel.y.abs() + vel.z.abs();
            pot * kin
        }).sum()
    }

    fn find_repeating_phase(&mut self) -> isize {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        while x == 0 || y == 0 || z == 0 {
            self.step();
            if x == 0 && a.pos.x == self.a.pos.x && a.vel.x == self.a.vel.x
                && b.pos.x == self.b.pos.x && b.vel.x == self.b.vel.x
                && c.pos.x == self.c.pos.x && c.vel.x == self.c.vel.x
                && d.pos.x == self.d.pos.x && d.vel.x == self.d.vel.x {
                    x = self.step;
                }
            if y == 0 && a.pos.y == self.a.pos.y && a.vel.y == self.a.vel.y
                && b.pos.y == self.b.pos.y && b.vel.y == self.b.vel.y
                && c.pos.y == self.c.pos.y && c.vel.y == self.c.vel.y
                && d.pos.y == self.d.pos.y && d.vel.y == self.d.vel.y {
                    y = self.step;
                }
            if z == 0 && a.pos.z == self.a.pos.z && a.vel.z == self.a.vel.z
                && b.pos.z == self.b.pos.z && b.vel.z == self.b.vel.z
                && c.pos.z == self.c.pos.z && c.vel.z == self.c.vel.z
                && d.pos.z == self.d.pos.z && d.vel.z == self.d.vel.z {
                    z = self.step;
                }
        }
        lcm(lcm(x, y), z)
    }
}


fn main() {
    let input = "<x=3, y=-6, z=6>
<x=10, y=7, z=-9>
<x=-3, y=-7, z=9>
<x=-8, y=0, z=4>";

    let mut system = System::load(&input);
    (0..1000).for_each(|_| system.step());
    println!("Part1: {}", system.energy());

    let mut system = System::load(&input);
    let r = system.find_repeating_phase();
    println!("Part2: {}", r);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system2_one() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut system = System::load(&input);
        (0..10).for_each(|_| system.step());
        assert_eq!(system.energy(), 179);
    }

    #[test]
    fn test_one() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        let mut system = System::load(&input);
        system.step();
        assert_eq!(system, System {
            step: 1,
            a: Body { pos: Vec3 { x: 2, y: -1, z: 1 }, vel: Vec3 { x: 3, y: -1, z: -1 } },
            b: Body { pos: Vec3 { x: 3, y: -7, z: -4 }, vel: Vec3 { x: 1, y: 3, z: 3 } },
            c: Body { pos: Vec3 { x: 1, y: -7, z: 5 }, vel: Vec3 { x: -3, y: 1, z: -3 } },
            d:  Body { pos: Vec3 { x: 2, y: 2, z: 0 }, vel: Vec3 { x: -1, y: -3, z: 1 } }
        });
        system.step();
        assert_eq!(system, System {
            step: 2,
            a: Body { pos: Vec3 { x: 5, y: -3, z: -1 }, vel: Vec3 { x: 3, y: -2, z: -2 } },
            b: Body { pos: Vec3 { x: 1, y: -2, z: 2 }, vel: Vec3 { x: -2, y: 5, z: 6 } },
            c: Body { pos: Vec3 { x: 1, y: -4, z: -1 }, vel: Vec3 { x: 0, y: 3, z: -6 } },
            d: Body { pos: Vec3 { x: 1, y: -4, z: 2 }, vel: Vec3 { x: -1, y: -6, z: 2 } }
        });
        (0..8).for_each(|_| system.step());
        assert_eq!(system, System {
            step: 10,
            a: Body { pos: Vec3 { x: 2, y: 1, z: -3 }, vel: Vec3 { x: -3, y: -2, z: 1 } },
            b: Body { pos: Vec3 { x: 1, y: -8, z: 0 }, vel: Vec3 { x: -1, y: 1, z: 3 } },
            c: Body { pos: Vec3 { x: 3, y: -6, z: 1 }, vel: Vec3 { x: 3, y: 2, z: -3 } },
            d: Body { pos: Vec3 { x: 2, y: 0, z: 4 }, vel: Vec3 { x: 1, y: -1, z: -1 } }
        });
        assert_eq!(system.energy(), 179);

        let mut system = System::load(&input);
        let steps = system.find_repeating_phase();
        assert_eq!(steps, 2772);
    }

    #[test]
    fn test_two() {
        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let mut system = System::load(&input);
        let steps = system.find_repeating_phase();
        assert_eq!(steps, 4686774924);
    }

}
