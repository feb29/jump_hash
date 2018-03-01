extern crate jump_hash;
extern crate seahash;
use jump_hash::slot;

#[test]
fn rebalance() {
    for n in 3..16 {
        let m1 = simulate(" mod", n, n + 1, bymod);
        let m2 = simulate("slot", n, n + 1, slot);
        assert!(m1 > m2);
    }
}

fn bymod(key: u64, len: usize) -> u32 {
    (key % (len as u64)) as u32
}

fn simulate<F>(name: &'static str, n: usize, m: usize, func: F) -> usize
where
    F: Fn(u64, usize) -> u32,
{
    assert!(n < m);
    let mut moves = 0;
    for i in 0..100000 {
        if func(i, n) != func(i, m) {
            moves += 1;
        }
    }
    println!("{:>5} {:>2}->{:<3} {:>6}", name, n, m, moves);
    moves
}
