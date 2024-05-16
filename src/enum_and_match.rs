// --- enum and match ---
// - enum: 여러 정해진 경우의 수 중 택일 값을 다루게 하는 타입
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

#[derive(Debug)]
enum Message {
    StartGame,
    WinPoint { who: String }, // struct 형태
    ChangePlayerName(String), // tuple 형태
}

pub fn enumeration_ex() {
    let red: Color = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    println!("red = {:?}", red); // 디버그 출력 결과 red = Red

    //   - enum끼리 비교 가능: #[derive(PartialEq)]로 PartialEq 지시자 사용 → 비교 연산자 구현
    println!("red == green: {}", red == green);
    println!("red == red: {}", red == Color::Red);

    //   - enum 개별 값 안에 다른 값들 담기
    let m1 = Message::StartGame;
    let m2 = Message::WinPoint {
        who: String::from("홍길동"),
    };
    let m3 = Message::ChangePlayerName(String::from("둘리"));
    println!("m1 = {:?}", m1);
    println!("m2 = {:?}", m2);
    println!("m3 = {:?}", m3);

    //   - enum Option<T>: null pointer가 없으므로, 존재 부존재를 따질 때 Option이라는 enum을 사용
    let some_number = Some(2);
    let absent_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("absent_number = {:?}", absent_number);
    //   - some_number + 1; // 이런 식으로 처리할 수 없음 → 꺼내려면 match 구문 사용

    // - match 구문: pattern matching(패턴 매칭, 패턴 부합)
    //   - 구분 가능한 값에 따라 처리하는 다른 언어의 case/switch 등과 유사
    //   - 매칭 시 해당 값을 꺼낼 수 있음
    //   - 가능한 값 모두를 exaustive하게 명시해야하는 제약사항 있음
    println!("color_to_rgb(blue) = {:?}", color_to_rgb(blue));
    
    //   - match하면서 enum에서 값을 꺼내서 활용거나 match 안에서 다른 type으로 변환하여 enum 안의 값을 활용할 수 있음
    handle_message(&m2);
    handle_message(&m3);

    let rgb_red = color_to_rgb(red);
    println!("rgb_red.0 = {}", rgb_red.0);
    println!("rgb_red.1 = {}", rgb_red.1);
    println!("rgb_red.2 = {}", rgb_red.2);
    
    //   - enum Option<T> 안에 있는 값 사용하기
    println!("increment(some_number) = {:?}", increment(some_number));
    println!("increment(absent_number) = {:?}", increment(absent_number));

    //   - match 구문의 와일드카드 _ → 모든 경우의 수를 다 명시하기 어려운 상황 혹은 enum 안의 값을 의도적으로 사용하지 않을 때
    handle_message_with_wildcard1(&m3);
    handle_message_with_wildcard2(&m3);
}

fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0), // 각 경우는 comma(,)로 구분
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("게임 시작!"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        Message::ChangePlayerName(name) => println!("플레이어 이름 변경 => {}", name),
    };
}

fn handle_message_with_wildcard1(message: &Message) {
    match message {
        Message::StartGame => println!("게임 시작!"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        _ => println!("아직 구현하지 않은 메시지"),
    };
}

fn handle_message_with_wildcard2(message: &Message) {
    match message {
        Message::StartGame => println!("게임 시작!"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        Message::ChangePlayerName(_) => println!("플레이어 이름 변경 요청"), // enum 자체는 확정됐지만, 안에 있는 값을 다루지 않으려는 경우
    };
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
