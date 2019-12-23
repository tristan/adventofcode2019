use std::mem;

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Iterative_version_in_C
pub fn gcd(mut u: isize, mut v: isize) -> isize {
    if u == 0 { return v.abs() }; // result should always be positive
    if v == 0 { return u.abs() };

    // ignore negatives, because it doesn't matter
    u = u.abs();
    v = v.abs();

    // store common factors of 2
    let shift = (u | v).trailing_zeros();

    // remove all factors of 2 in u
    u >>= u.trailing_zeros();

    loop {
        // remove all factors of 2 in v
        v >>= v.trailing_zeros();
        if u > v {
            mem::swap(&mut u, &mut v);
        }
        v -= u;

        if v == 0 { break; }
    };

    // restore common factors of 2
    u << shift
}

pub fn lcm(u: isize, v: isize) -> isize {
    (u * (v / gcd(u, v))).abs()
}
