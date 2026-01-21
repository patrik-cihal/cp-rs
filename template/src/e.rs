use super::*;

fn solve_test_case() {
    let n = inp!(usz);
    let v = inp![usz; n];

    outp!("{}\n", v.iter().max().unwrap())
}

pub fn solve() {
    let t = inp!(usz);
    for _ in 0..t {
        solve_test_case();
    }
    writer().flush().unwrap();
}

