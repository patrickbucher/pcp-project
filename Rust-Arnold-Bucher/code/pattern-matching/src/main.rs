fn main() {
    for n in [12, 50, 1000].iter() {
        match pythagorean_triplet(&n) {
            Some([a, b, c]) => println!("{}²+{}²={}²", a, b, c),
            None => println!("{} is not the sum of a Pythagorean Triplet", n),
        }
    }
}

/// Returns an Option of a Pythagorean Triplet summing up to n.
/// The triplet [a, b, c] returned fulfills these conditions:
/// 1) a²+b²=c² (Pythagorean Triplet)
/// 2) c>b>a
fn pythagorean_triplet(n: &u32) -> Option<[u32; 3]> {
    for a in 1..n / 3 {
        for b in a + 1..n - 2 * a {
            let c = n - a - b;
            if c * c == a * a + b * b {
                return Some([a, b, c]);
            }
        }
    }
    return None;
}
