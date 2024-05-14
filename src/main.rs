pub mod variable; // 참고: https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod scalar_data_type;
pub mod compound_data_type;
pub mod function;
pub mod control_flow;
pub mod ownership;
pub mod borrowing;
pub mod slice;
mod struct_and_instance;
mod struct_and_method;
use crate::variable::variable_ex;
use crate::scalar_data_type::scalar_data_type_ex;
use crate::compound_data_type::compound_data_type_ex;
use crate::function::function_ex;
use crate::control_flow::control_flow_ex;
use crate::ownership::ownership_ex;
use crate::borrowing::borrowing_ex;
use crate::slice::slice_ex;
// cf. namespace에 접근할 때 ::를 사용

fn main() {
    println!("Hello, world!");

    variable_ex();
    scalar_data_type_ex();
    compound_data_type_ex();
    function_ex();
    control_flow_ex();
    ownership_ex();
    borrowing_ex();
    slice_ex();
    struct_and_instance::struct_ex();
    struct_and_method::struct_method_ex();
}
