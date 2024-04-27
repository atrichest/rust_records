#[allow(unused)]
fn main() {
    borrow_checker();
    borrow_checker_error();
}

//An attempt to use a reference whose value has gone out of scope
//Annotations of the lifetimes of r and x, named 'a and 'b, respectively
fn borrow_checker_error(){
    let r;              //--------+-- 'a
    {                   //        |
        let x = 5;      //-+-- 'b |
        r = &x;         // |      |
    }                   //-+      |
    println!("r: {r}"); //        |
}                       //--------+

//A valid reference because the data has a longer lifetime than the reference
fn borrow_checker() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
