// --- compound data type: tuple, array ---
pub fn compound_data_type_ex() {

    // - tuple: 다른 data type 정해진 수만큼을 확보해둔 쌍
    let tuple1: (i32, bool, f64) = (32, true, 1.41);
    println!("tuple1 = {:?}", tuple1); // 참고: https://popofly.tistory.com/137
    
    //   - tuple destructuring
    let (x0, x1, x2) = tuple1;
    println!("x0 = {x0}, x1 = {x1}, x2 = {x2}");

    //   - tuple item 접근: .[index]로 접근 가능
    let x2 = tuple1.2;
    println!("x2 = {x2}");
    // let x3 = tuple1.3; // no field `3` on type `(i32, bool, f64)` → tuple 쌍 개수는 compile time에 체크 가능

    //   - 특별한 튜플 unit
    //     - 아무 값도 없는 유일한 tuple ()
    //     - 어떤 함수가 아무 반환값도 없을 때, 아무 반환값 없음을 의미하는 값으로 활용 - Java의 void 같은 것

    // - array:  동일한 data type의 정해진 개수(길이 고정)의 element를 관리 (cf. 길이 가변이며 동일 data type element 관리 필요 시에는 vector type 사용)
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    // let array1 = [1, 2, 3, 4, 5, "hello"]; // mismatched types - expected integer, found `&str` → 동일한 data type만 담을 수 있음
    // let array1: [i32; 6] = [1, 2, 3, 4, 5]; // mismatched types - expected an array with a fixed size of 6 elements, found one with 5 elements

    //   - array element 접근
    let element = array1[4];
    println!("element = {element}");
    
    //   - 같은 값으로 채운 array 만들기(문법적 편의 기능)
    let threes = [3; 100];
    let last_of_threes = threes[99];
    println!("last_of_threes = {last_of_threes}");
    // last_of_threes = 2; // cannot mutate immutable variable `last_of_threes` → mutable이 아님
}
