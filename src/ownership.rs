// --- ownership ---
// - 소유권 ownerhsip: Rust 프로그램이 메모리를 어떻게 관리할지 결정하는 규칙들
//   - (메모리 할당과 해제 비교) GC를 이용하는 언어들 Java, Go, Python, JavaScript / 프로그래머가 수동으로 메모리 관리 C, C++ / 컴파일 시점에 메모리 관리 규칙 검사 Rust
//   - 소유권 규칙에 따라 컴파일 시점에 메모리 할당/해제 관리
//   - 규칙에 어긋나면 컴파일 오류, 규칙을 잘 지키면 컴파일러가 처리해주는 셈
//   - 변수의 scope가 끝나면 메모리 해제 가능 → 블록 안에서만 변수가 유효, 블록을 벗어나면 무효

// - stack, heap 메모리
//   - cf. Rust에서는 초반에 필수적으로 알아야 함, 다른 언어에서는 거의 생각할 필요가 없는 부분
//   - string literal(문자열 리터럴)과 String - 비슷해보이지만 미묘하게 다르다
//     - 기본 데이터 타입은 stack에서 쉽게 관리 가능(사이즈가 정해져있으므로)
//     - string literal도 immutable이므로 프로그램에 고정으로 확보 가능
//     - 그러면 미리 크기를 알 수 없는 사용자 입력 문자열(DB에서 가져온 문자열, 네트워크로 받은 문자열)은?
//     - String type으로 관리 → mutable, 컴파일 시점에 미리 크기를 알 수 없음 → heap 메모리를 사용해야함 

pub fn ownership_ex() {
    string_literal_and_string_ex();
    ownership_rules_ex();
}

fn string_literal_and_string_ex() {
    //     - let s = "hello"; // rust-analyzer가 띄워주는 추론 타입을 보면 String 타입이 아니라 &str임을 볼 수 있음
    //     - let s: String = "hello"; // mismatched types, expected `String`, found `&str` → String과 &str이 다름을 확인할 수 있음
    let s1: String = String::from("hello"); // string literal을 heap으로 보낸 것
    println!("s1 = {}", s1);

    //     - 앞서 말했듯, String은 mutable, string literal을 이용해서 변경하려면 push_str()을 이용
    let mut s2: String = String::from("헬로");
    s2.push_str(", 월드!");
    println!("s2 = {}", s2);
    //     - 위처럼 String type은 실행 시점에 메모리 할당 요청, 메모리 할당한 문자열을 다 사용했다면 메모리 반환 방법 필요
}

fn ownership_rules_ex() {
    // - ownership 규칙
    //   - Rust에서 모든 값은 소유자(owner)가 있음
    //   - 한 시점에 딱 하나의 소유자만 있음
    //   - 소유자의 범위가 끝나면 값도 제거됨
    //   - ex. 단순 데이터는 stack에서 복사
    //     - scalar data type이 주로 속함: 모든 정수 타입, bool 타입, char 타입, 이들로 이뤄진 tuple (cf. tuple에 String 같은 게 들어간다면 소유권 개념 필요)
    let x = 3;
    let y = x;
    println!("x = {x}, y = {y}");

    //   - ex. heap 메모리에 위치하는 데이터 예시: String, heap 메모리 공유, 소유권, clone()
    {
        let s1 = String::from("헬로"); // Rust에서 String은 UTF-8 인코딩 사용 - 한글 3 byte 사용 - 헬로는  6byte - s1은 현재 크기 6 byte인 heap을 가리킴
        let s2 = s1; // 위의 heap 메모리를 공유, s1의 소유권 박탈, s2가 소유권 가짐
        // println!("s1 = {}", s1); // borrow of moved value: `s1`, value borrowed here after move → heap에 위치한 데이터는 복사가 아니라 소유권 개념이 들어가므로 불가능한 작업
        println!("s2 = {}", s2); // 문제 없음
        //     - 이미 소유권이 넘어갔기 때문에 블록이 끝날 때에도 메모리 해제 작업은 한 번만 발생함
    } 
    {
        let s1 = String::from("헬로");
        let s2 = s1.clone(); // clone()을 사용해서 heap 메모리 데이터 복사
        println!("s1 = {}", s1); // 문제 없음 // s1의 데이터를 heap에서 공유한 게 아니라 복사했기 때문
        println!("s2 = {}", s2); // 문제 없음
    }

    //   - ex. 함수 호출 시 소유권 이동
    let hello = String::from("헬로");
    let returned_hello = print_string_length(hello);
    // println!("hello = {}", hello); // borrow of moved value: `hello`, value borrowed here after move → 함수로 소유권이 이동했기 때문에 사용할 수 없음
    //     - 여기서도 마찬가지로 기본 데이터였다면 소유권 문제는 발생하지 않음 - stack에서 값을 복사하므로
    //     - 한편 return value의 소유권은 called function 쪽에 있음
    println!("반환된 문자열은 \"{}\"입니다.", returned_hello); // 소유권에는 변동이 있었지만, hello와 returned_hello는 heap 메모리의 같은 데이터를 가리킴
    
    //   - ex. (불편한 방법) tuple 및 shadowing을 이용해 소유권을 다시 넘겨 받기 → 더 나은 방법은 임대
    let string_ex = String::from("헬로");
    let (len, string_ex) = get_string_length(string_ex); // string_ex를 shadowing
    println!("문자열 string_ex \"{}\"의 길이는 {} byte", string_ex, len);
}

fn print_string_length(string_parameter: String) -> String {
    println!("문자열의 길이는 {} byte입니다.", string_parameter.len());
    string_parameter
}

fn get_string_length(string_parameter: String) -> (usize, String) {
    println!("string_parameter = {}", string_parameter);
    (string_parameter.len(), string_parameter)
}
