// --- control flow: if, loop, while, for ---
pub fn control_flow_ex() {
    if_expression_ex();
    println!("loop_ex() = {:?}", loop_ex());
    while_ex();
    for_ex();
}

fn if_expression_ex() {
        // - if expression: expression이므로 평가 결과 사용 가능
    //   - if, else
    let x = 4;
    let is_x_even = if x % 2 == 0 {
        println!("x는 짝수입니다.");
        x % 2 == 0
    } else {
        // cf. if를 expression으로 사용하여 평가 결과를 이용한다면 else가 반드시 있어야 함, 그렇지 않으면 오류 - else가 없는 if expression은 unit ()으로 평가됨
        //      ←  `if` may be missing an `else` clause, `if` expressions without `else` evaluate to `()`, consider adding an `else` block that evaluates to the expected type
        println!("x는 홀수입니다.");
        x % 2 != 0 // return 값이므로 ;를 붙이지 않음(return 키워드를 사용하지 않을 경우)
    }; // 할당은 statement이므로 ;이 붙음 // 만약 if의 평가 결과를 이용하지 않고 statement처럼 사용한다면 ;을 붙이지 않아도 됨
    println!("is_x_even = {is_x_even}");

    let y: () = if x % 2 == 0 {}; // 활용할 일은 없을 것 같지만 위에서 언급했듯 else가 없는 if expression은 unit ()으로 평가됨
    let is_y_unit = y == ();
    println!("y == ()? {is_y_unit}");

    //   - else if
    let a = 4;
    if a % 3 == 0 {
        println!("x는 3으로 나누어 떨어집니다.")
    } else if a % 3 == 1 {
        println!("x는 3으로 나눈 나머지는 1입니다.")
    } else {
        println!("x는 3으로 나눈 나머지는 2입니다.")
    }; // 평가 결과를 이용하지 않으므로 ;를 붙이지 않아도 됨, 붙여도 안 붙여도 warning이나 error는 없음
}

fn loop_ex() -> i32 {
    // - loop: while보다 더 원시적인 방법의 반복문
    //   - 기본적으로 종료되지 않고 계속 반복
    //   - 어떤 조건에 의해 break;로 종료되도록 작성해야함
    let mut counter = 0;
    loop {
        print!("반복!");
        println!();
        counter += 1;
        if  counter == 3 {
            break;
        }
    }
    
    //   - ! break도 중괄호 블록을 끝내며 값을 가지고 나갈 수 있음
    counter = 0;
    let return_value = loop {
        print!("반복!");
        println!();
        counter += 1;
        if  counter == 3 {
            break counter;
        }
    };
    
    return_value
}

fn while_ex() {
    // - while: loop에서 종료 조건을 명시한 형태
    let mut counter = 0;
    while counter < 3 {
        print!("반복!");
        println!();
        counter += 1;
    }
    
    //   - while로 array item 반복
    let array = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while idx < array.len() {
        println!("array[{}] = {}", idx, array[idx]);
        idx += 1;
    }
}

fn for_ex() {
    // - for: while보다 경계값에 대해 더 명확하게 처리 가능
    //   - for에서 in을 이용하여 array item 반복
    let array = [1, 2, 3, 4, 5];
    for x in array {
        println!("x = {}", x);
    }

    //   - for로 숫자 범위 다루기
    for i in 0..5 {
        println!("i = {}", i);
    }

    let length = array.len();
    for i in (0..length).rev() { // rev()로 숫자 범위를 거꾸로 돌릴 수도 있음
        println!("i = {}", i);
    }
}
