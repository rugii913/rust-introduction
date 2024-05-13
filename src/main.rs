pub mod variable; // 참고: https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod scalar_data_type;
use crate::variable::variable_ex;
use crate::scalar_data_type::scalar_data_type_ex;

fn main() {
    println!("Hello, world!");

    variable_ex();
    scalar_data_type_ex();
}
