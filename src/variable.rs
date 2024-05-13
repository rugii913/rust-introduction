// --- 변수, 불변성, 상수, 변수 가리기 ---
// - 상수 선언 const: 함수 내에서 선언될 수도 있음
// const pi = 3.141592; // Syntax Error: missing type for `const` or `static` → const는 type을 명시해줘야 함
// const pi: f32 = 3.141592; // Constant `pi` should have UPPER_SNAKE_CASE name, e.g. `PI` → const는 UPPER_SNAKE_CASE를 사용, error는 아니고 warning(컴파일 가능)
const PI: f32 = 3.141592; // 
// const mut PI2: f32 = 3.14; // Syntax Error: const globals cannot be mutable → const는 mut이 될 수 없음
// let pi = 3.141592 // Syntax Error: expected an item → let은 함수 바깥에서 선언할 수 없음 (참고: https://wonlf.tistory.com/entry/Rust-3-Rust%EC%97%90%EC%84%9C-%EB%B3%80%EC%88%98%EC%9D%98-%ED%8A%B9%EC%A7%95)

pub fn variable_ex() {
    // - 변수 선언 let
    let x = 3;
    println!("x의 값은 {x}입니다."); // { }로 문자열 내에서 변수를 가져올 수 있음
    // x = 4; // cannot mutate immutable variable `x` → 변수 선언 시 기본적으로 immutable

    // - mutable 변수 선언 let mut
    let mut y = 1;
    println!("y의 값은 {y}입니다.");
    y = 2;
    println!("y의 값은 {y}입니다.");

    println!("PI의 값은 {PI}입니다.");

    // - 변수 가리기 shadowing
    // x = 4;는 컴파일 에러가 발생하지만
    let x = x + 1; // 다시 let 키워드를 붙여서 x를 선언할 경우, shadowing 발생 - 같은 scope 내에서도 같은 식별자로 새로운 변수 선언 가능
    println!("x의 값은 {x}입니다.");
    {
        // - 내부 scope 중괄호 블럭: 블럭을 열어서 새로운 내부 scope를 만들 수 있고, 내부 scope에서 외부 scope의 변수 shadowing 가능
        println!("x의 값은 {x}입니다.");
        let x = x * 2;
        println!("안 쪽 범위에서 x의 값은 {x}입니다.");
    }
    println!("x의 값은 {x}입니다."); // scope가 끝나면 원래 scope의 변수를 사용
}