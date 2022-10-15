pub use lc_249_group_shifted_strings::*;
fn main() {
    let foo = ShiftEqClass::new("".to_string());
    let fooo = ShiftEqClass::new("a".to_string());
    let bar = ShiftEqClass::new("hey".to_string());
    let foobar = ShiftEqClass::new("khb".to_string());
    println!("{:?}, {}", foo, foo.lt(&fooo));
    println!("{:?}, {}", fooo, foo.eq(&fooo));
    println!("{}", bar.eq(&foobar));
}
