// --- method ---
// - method
//   - 함수 fn과 비슷, 파라미터를 받고 return
//   - struct, enum, trait 문맥 안에 정의
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// - method 정의 방법
//   - impl 키워드 + 구조체 이름 + 중괄호 블럭(구조체 문맥) 내에서 함수 정의
impl Rectangle {
    //   - method의 첫번째 parameter는 self 혹은 &self 혹은 &mut self
    fn area(&self) -> u32 {
    // fn area(self: &Self) -> u32 { // parameter에 있는 &self는 self: &Self를 축약해서 쓸 수 있게 한 것
    // self 키워드는 어떤 문맥 안에서 자기 자신을 가리킴
    // 여기서 &Self는 &Rectangle과 같음
    // 소유권을 가져갈 수도 있고, 불변 혹은 가변으로 임대만할 수도 있음
        self.width * self.height
    }
}

// cf. implementation 블럭이 여러 곳으로 나뉘어져 있어도 상관 없음
// - (method가 아닌) 연관 함수 associated function - 어떤 struct 등에 특화된 함수에 필요한 경우 작성
impl Rectangle {
        fn square(size: u32) -> Rectangle { // associated function은 self를 받지 않음
            Rectangle {
                width: size,
                height: size,
            }
        }
}

pub fn struct_method_ex() {
    let rectangle = Rectangle {
        width: 20,
        height: 30,
    };
    println!("이 사각형의 면적은 {}", rectangle.area());
    dbg!(rectangle); // area에서 사용한 &self는 임대한 것이므로 소유권이 넘어가진 않았음
    // 만약 self: &Self가 아니라 self: Self로 받는다면, 소유권이 넘어가므로 윗줄에서 컴파일 에러 발생
   
    println!("정사각형 = {:?}", Rectangle::square(20)); // String::from()을 호출할 때와 비슷한 모양새
}
