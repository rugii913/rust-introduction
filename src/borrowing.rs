// --- borrowing ---
pub fn borrowing_ex() {
    // - 참조 Reference
    //   - 특정 데이터가 위치한, 접근할 수 있는 주소, 소유권을 넘기지 않고 데이터 접근 가능
    //   - immutable reference는 여러 개 활용 가능하지만, mutable reference가 나오는 순간 해당 메모리에 대한 reference는 하나만 존재 가능
    //     - immutable reference만 사용한다면 자유롭게 사용 가능
    //     - mutable reference 유일함 판단은 이에 대한 reference가 사용 중인지 사용이 끝났는지를 기준으로 함
    let s = String::from("헬로");

    //   - &을 이용해 소유권을 단지 임대할 뿐임을 명시
    let length = calculate_length(&s);
    println!("{}의 길이는 {} byte입니다.", s, length); // 번거롭게 tuple과 shadowing을 사용할 필요가 없어짐

    //   - 참조는 기본적으로 immutable
    // append_word1(&s);
    //   - mutable한 참조는 명시해줘야 하며, 넘기는 인자도 mutable이어야 함
    let mut mutable_s =String::from("헬로");
    append_word2(&mut mutable_s);

    // - 변경 가능한 참조 mutable reference 제약 조건
    //   - 어떤 메모리에 대한 mutable reference가 있으면 다른 reference는 쓸 수 없게 됨 - 컴파일 에러
    //     - mutable이든 immutable이든 더 만들 수 없음
    //     - 다른 immutable refence가 mutable reference보다 먼저 선언됐어도 컴파일 에러 - 아예 다른 scope로 분리돼있지 않은 이상 - mutable이 끼는 순간 배타적이 됨
    mutable_reference_constraints_ex1();
    mutable_reference_constraints_ex2();

    // - 데이터 경쟁조건 data race
    //   - 둘 이상의 포인터가 같은 데이터를 참조
    //   - 한 개 이상의 포인터가 데이터를 쓰려고 접근 → 하나에만 접근하려고 해도 컴파일 오류
    //   - 해당 데이터 접근을 동기화할 방법이 없음
    //   - Rust는 컴파일러에서 데이터 경쟁 조건을 방지함
    //     - 다만 데이터 경쟁 조건을 방지하기 위한 것이기에 범위가 겹치지만 않는다면 사용 가능함
    data_race_ex1();
    data_race_ex2();
    data_race_ex3();
    data_race_ex4();
}

fn calculate_length(s: &String) -> usize { // parameter의 type에 소유권을 임대받음을 명시
    s.len()
}

// fn append_word1(s: &String) {
    // s.push_str("월드"); // cannot borrow `*s` as mutable, as it is behind a `&` reference, `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }

fn append_word2(s: &mut String) {
    s.push_str("월드");
}

fn mutable_reference_constraints_ex1() {
    let mut s = String::from("헬로");

    // * mutable reference는 추가 참조를 만들 수 없음
    let _r1 = &mut s;
    let _r2 = &mut s; // cannot borrow `s` as mutable more than once at a time, second mutable borrow occurs here → 선언까지는 되는데 사용할 수 없음
    // println!("r1 = {}, r2 = {}", &_r1, &_r2); // 왜 사용하려고 하면 할당 부분에서 컴파일 오류가 생기는지? → 컴파일러에서 데이터 경쟁조건을 막기 때문
    // println!("r1 = {}", &_r1); // 하나의 포인터만 사용하려고 해도 두번째 할당 부분에서 컴파일 오류
}

fn mutable_reference_constraints_ex2() {
    let mut s = String::from("헬로");

    // * mutable reference는 추가 참조를 만들 수 없음 - immutable reference라도 마찬가지
    let _r1 = &mut s;
    let _r2 = &s; // cannot borrow `s` as mutable more than once at a time, second mutable borrow occurs here → immutable reference라도 마찬가지
    // println!("r1 = {}, r2 = {}", &_r1, &_r2);
    // println!("r1 = {}", &_r1); // 하나의 포인터만 사용하려고 해도 두번째 할당 부분에서 컴파일 오류
}

fn data_race_ex1() {
    //   - 다른 블록을 사용할 수 있음
    let mut s = String::from("헬로");

    let _r1 = &mut s;
    {
        let r2 = &mut s; // 아래 println!("r1 = {}", r1);을 주석 해제할 경우 - cannot borrow `s` as mutable more than once at a time second mutable borrow occurs here → 여기서도 둘 다 쓸 순 없음
        // println!("_r1 = {}", _r1);
        println!("r2 = {}", &r2);
    }
    // println!("_r1 = {}", _r1); // 여기서 주석을 해제해도 위 r2에서 문제가 생김, 위 r2가 있는 scope에서는 여러 참조가 있는 꼴이 되기 때문
}

fn data_race_ex2() {
    //   - 바깥에서도 참조를 사용하고 싶다면 어쨌든 어떤 scope에서도 mutable reference로 가리키는 것에 대한 reference가 여러 번 나타나지 않도록 해야 함
    let mut s = String::from("헬로");
    {
        let r2 = &mut s; // 아래 println!("r1 = {}", r1);을 주석 해제할 경우 - cannot borrow `s` as mutable more than once at a time second mutable borrow occurs here → 여기서도 둘 다 쓸 순 없음
        println!("r2 = {}", &r2);
    }
    let r1 = &mut s;
    println!("r1 = {}", &r1);
    // * 어떤 scope에서도 mutable reference로 가리키는 것에 대한 reference가 여러 번 나타나지 않으므로 가능함
}

fn data_race_ex3() {
    // * 다른 immutable refence가 mutable reference보다 먼저 선언됐어도 컴파일 에러
    let mut s = String::from("헬로");

    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", &r1, &r2);

    let _r3 = &mut s; // 역시 여기까지는 작성했을 때는 컴파일 에러 발생하지 않으나, 아래처럼 접근하려고 하면 이 줄에서 컴파일 에러 발생
    // println!("r1 = {}", &r1);
}

fn data_race_ex4() {
    let mut s = String::from("헬로");

    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", &r1, &r2);

    // * data_race_ex3()과 비교해보면, 범위가 겹치지만 않는다면 사용 가능 - 참조의 범위는 블럭 뿐만 아니라, 실제 사용하는 것까지 체크하기 때문
    let r3 = &mut s; 
    println!("r3 = {}", &r3);

    // * mutable reference끼리도 마찬가지
    let r4 = &mut s;
    println!("r4 = {}", &r4);
}
