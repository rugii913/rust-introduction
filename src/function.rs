// --- function ---
pub fn function_ex() {
    println!("function_ex()");
    // - 함수 호출
    a_function();
    a_function_with_parameters(7, 6);
    let return_value = a_function_returning_a_value();
    println!("return_value = {return_value}");
}

// - 함수 작성
//   - fn 키워드 + 식별자(함수 이름)
fn a_function() {
    println!("a_function()");
}

//   - 매개변수(parameter)가 있는 함수
fn a_function_with_parameters(x: i32, y: i32) {
    let add = x + y;
    println!("a_function_with_parameters → x + y = {add}")
}

//   - cf. (명령)문 statement, 식 expression
//     - 문: 무언가 일을 하고 반환값은 없음 ex. let x = 3; → 결과값이 없기 때문에 let y = (let x = 3); 이런 작업이 불가능(가능한 언어도 있겠으나, Rust에서는 불가능)
//     - 식: 평가 후 최종 결과값이 있음
//     - function의 경우, 여러 문에 이어 마지막 식으로 끝남
//       - 마지막 식은 선택적: 있을 수도 있고, 없을 수도 있음

//   - parameter가 있는 함수: return type은 signature(함수 이름 + 파라미터) 뒤 -""> [data type]"으로 명시
fn a_function_returning_a_value() -> i32 {
    let x = {
        // Rust에서는 중괄호 블럭을 열어서 어떤 작업을 할 수 있고, 마지막 평가된 expression의 결과를 중괄호 블럭의 평가 결과로 돌려줌
        5 // 평가 후 결과값이 있으므로 expression임
    }; // let x = 5; 자체는 statement
    
    // x; // remove this semicolon to return this value → return 키워드를 사용하지 않고 반환되는 것에는 semicolon을 붙이면 안 됨 
    // return x; // 가능함
    x
}
