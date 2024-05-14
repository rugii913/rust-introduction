// --- struct ---
// - struct 작성
//   - struct 키워드, 이름 명시, 안에 들어가는 필드들 선언
struct User {
    name: String,
    email: String,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn struct_ex() {
    // - struct의 instance 만들기
    let user1 = User { // 이 user1이 어떤 scope에서 벗어나면 여기 name, email도 알아서 소유권 해제, active는 bool이라 copy로 관리되므로 해당 없음
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };

    // - struct instance의 field에 접근하기
    println!("user1.name = {}, user1.email = {}, user1.active = {}", user1.name, user1.email, user1.active);

    // - struct instance의 field 변경
    // user1.email = String:: from("gdhong@example.com"); // cannot mutate immutable variable `user1` → 다른 type과 마찬가지로 선언 시 mut이 붙어있어야 변경 가능
    let mut user2 = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };
    println!("user2.name = {}, user2.email = {}, user2.active = {}", user2.name, user2.email, user2.active);
    user2.email = String:: from("gdhong@example.com"); // cannot mutate immutable variable `user1`
    println!("user2.name = {}, user2.email = {}, user2.active = {}", user2.name, user2.email, user2.active);

    // - struct instance 만드는 함수 활용
    let user3 = build_user(String::from("김김김"), String::from("kimkimkim@example.com"));
    println!("user3.name = {}, user3.email = {}, user3.active = {}", user3.name, user3.email, user3.active);

    // - 이미 있는 struct instance로부터 새로운 instance 만들기
    let user4 = User {
        name: user3.name,
        email: user3.email,
        active: false,
    };
    //   - 이 경우 문제는 user3의 field를 사용할 수 없음, 다만 데이터를 clone()하여 회피 가능
    // println!("user3.email = {}", user3.email); // borrow of moved value: `user3.email`, move occurs because `user3.email` has type `String`, which does not implement the `Copy` trait
    let _user5 = User {
        name: user4.name.clone(),
        email: user4.email.clone(), // 이런 식으로 데이터를 clone하는 식으로는 가능할 것, 하지만 귀찮음
        active: false,
    };
    println!("user4.email = {}", user4.email);

    //   - 간편한 방법
    let user6 = User {
        active: false, // 지정하고 싶은 필드만 명시, 나머지는 user4에서 소유권 이전 - clone()이 아님
        ..user4 //  JS의 spread syntax와 비슷한 모양새
    };
    // println!("user4.email = {}", user4.email); // borrow of moved value: `user4.email`, move occurs because `user4.email` has type `String`, which does not implement the `Copy` trait → clone() 해온 건 아님
    println!("user6.email = {}", user6.email);

    // - tuple struct
    //   - tuple과 같지만, type 이름을 주어 엄격하게 관리할 수 있는 장점
    let color = Color(1, 2, 3);
    println!("color.0 = {}, color.1 = {}, color.2 = {}", color.0, color.1, color.2);
    let point = Point(1, 2, 3);
    println!("point.0 = {}, point.1 = {}, point.2 = {}", point.0, point.1, point.2);
}

fn build_user(name: String, email: String) -> User {
    User {
        // name: name,
        // email: email,
        // JS에서 쓰듯 field와 parameter 이름이 같다면 생략 가능
        name,
        email,
        active: true,
    }
}
