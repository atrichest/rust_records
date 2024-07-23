fn main() {
    let mut x = 5;
    let y = &mut x;

    let x = 5;
    //cannot borrow `x` as mutable, as it is not declared as mutable
    // let y = &mut x;
    let y = &x;
}
