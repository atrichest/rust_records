//The backtrace generated by a call to panic! displayed when the environment variable RUST_BACKTRACE is set
//设置临时环境变量查看详细异常堆栈
//shell export RUST_BACKTRACE=1 当前以及子会话生效
//powershell $env:RUST_BACKTRACE=1 当前以及子会话生效
//cmd set RUST_BACKTRACE=1 当前会话生效
//
//永久环境变量cmd setx RUST_BACKTRACE=1 当前会话不生效，setx最大支持1024字符
//
//RUST_BACKTRACE=1 cargo run
fn main() {
    //calling panic!
    //[profile.release]
    //panic = 'abort'
    // panic!("crash and burn");

    //using a panic! Backtrace
    //Attempting to access an element beyond the end of a vector, which will cause a call to panic!
    let v = vec![1, 2, 3];
    v[99];
}
