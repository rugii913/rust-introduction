// --- scalar data type: integer, float, bool, char---
pub fn scalar_data_type_ex() {
    // - 정수 integer type
    //   - 타입
    //     - 부호 있는 i / 부호 없는 u
    //     - 비트 수 8, 16, 32, 64, 128, 아키텍처
    //   - integer literal 표현 (cf. 구분 기호 _ 사용 가능)
    //     - 10진수       ex. 19_384
    //     - 16진수       ex. 0xff
    //     - 8진수         ex. 0o77
    //     - 2진수         ex. 0b1111_1111
    //     - 바이트(u8) ex. b'A' (cf. u8과 char는 다르지만 이런 literal 사용이 가능함)

    // - 부동 소수점 floating-point type: 부호 있는 수만 있음
    //   - f32, f64 → 기본이 f64, f32를 사용하려면 명시 필요
    //   - cf. IEEE-754 표준

    // - 숫자 기본 연산
    let add = 3 + 8;
    let sub = 26.5 - 2.5;
    let mul = 7 * 20;
    let quotient = 12.0 / 3.14; // float type 간 나누기
    // let quotient = 12 / 3.14; // cannot divide `{integer}` by `{float}` the trait `Div<{float}>` is not implemented for `{integer}`
    let truncated = 7 / 5; // integer type 간 나누기
    let remainder = 46 % 5;
    println!("add = {add}, sub = {sub}, mul = {mul}, quotient = {quotient}, truncated = {truncated}, remainder = {remainder}");

    // - 불린 boolean type
    //   - boolean literal은 true, false 두 가지
    let t = true;
    let f: bool = false;
    println!("t = {t}, f = {f}");

    // - 문자 character type
    let a = 'A';
    // let a: char = b'A'; // mismatched types - expected `char`, found `u8`
    let a2 = b'A'; // u8과 char는 다름
    let ga: char = '가';
    let unicorn = '🦄';
    println!("a = {a}, a2 = {a2}, ga = {ga}, unicorn = {unicorn}")
}
