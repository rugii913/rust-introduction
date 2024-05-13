pub mod variable; // 참고: https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod scalar_data_type;
pub mod compound_data_type;
pub mod function;
use crate::variable::variable_ex;
use crate::scalar_data_type::scalar_data_type_ex;
use crate::compound_data_type::compound_data_type_ex;
use crate::function::function_ex;

fn main() {
    println!("Hello, world!");

    variable_ex();
    scalar_data_type_ex();
    compound_data_type_ex();
    function_ex();
}
