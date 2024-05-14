pub mod variable; // 참고: https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod scalar_data_type;
pub mod compound_data_type;
pub mod function;
pub mod control_flow;
pub mod ownership;
use crate::variable::variable_ex;
use crate::scalar_data_type::scalar_data_type_ex;
use crate::compound_data_type::compound_data_type_ex;
use crate::function::function_ex;
use crate::control_flow::control_flow_ex;
use crate::ownership::ownership_ex;
// cf. namespace에 접근할 때 ::를 사용

fn main() {
    println!("Hello, world!");

    variable_ex();
    scalar_data_type_ex();
    compound_data_type_ex();
    function_ex();
    control_flow_ex();
    ownership_ex();
}
