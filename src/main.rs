pub mod variable; // 참고: https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
use crate::variable::variable_ex;

fn main() {
    println!("Hello, world!");

    variable_ex();
}
