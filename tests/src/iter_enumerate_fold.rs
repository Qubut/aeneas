//@ [!lean] skip
#![feature(register_tool)]
#![register_tool(verify)]

#[verify::test]
pub fn enumerate_fold_sum() {
    let xs = [4usize, 5, 6];
    let total = xs.iter().enumerate().fold(0usize, |acc, (i, x)| acc + i * *x);
    assert!(total == 0 * 4 + 1 * 5 + 2 * 6);
}
