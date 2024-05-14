// --- slice ---
pub fn slice_ex() {
    // - slice type: 어떤 collection에 있는 연속된 일부 요소들을 참조하는 방법, reference와 마찬가지로 소유권을 넘기지 않음
    //   - string literal는 사실 string slice
    //     - ex. string slice - &str type
    let s = String::from("헬로 월드");
    let length = s.len();

    let hello = &s[0..6]; // word로 소유권을 넘기지 않고, s의 연속된 일부 영역을 참조 가능 // cf. 마지막 index는 exclusive
    println!("hello = {}", hello);
    let hello: &str = &s[..6]; // string slice는 &str 타입
    println!("hello = {}", hello);

    let world: &str = &s[7..length];
    println!("world = {}", world);
    let world: &str = &s[7..];
    println!("world = {}", world);

    let all: &String = &s;
    println!("all = {}", all);
    let all: &str = &s;
    println!("all = {}", all);
    let all = &s[..];
    println!("all = {}", all);

    println!("first_word1(&s) = {}", first_word1(&s));

    //   - &String(String의 reference)과 &str(String의 slice)은 다름
    // first_word1(hello); // expected reference `&String`, found reference `&str` → &String 자리에 &str을 넘길 수 없음
    //     - 그런데 &str(string slice) 자리에는 String의 reference도 넘길 수 있음
    println!("first_word2(&s) = {}", first_word2(&s));
    println!("first_word2(hello) = {}", first_word2(hello));

    // - 다른 collection의 slice
    let a = [1, 2, 3, 4, 5];
    let _reference = &a; // &[i32; 5] type
    let slice = &a[1..3]; // &[i32] type
    println!("a = {:?}, slice = {:?}", a, slice);
    // cf. :? 없으면 컴파일 에러 - `[{integer}; 5]` doesn't implement `std::fmt::Display`, the trait `std::fmt::Display` is not implemented for `[{integer}; 5]`, in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //      추후 trait 볼 것
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // 문자열에 있는 모든 byte를 돌면서 i로 인덱스 접근(enumerate()로 인덱스 i 확보)
        if item == b' ' {
            return &s[..i]; // 공백 문자를 만나면 그 직전까지의 slice를 반환(마지막 index는 exclusive)
        }
    }

    &s[..] // 공백이 없다면 전체 슬라이스를 반환
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
