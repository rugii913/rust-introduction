# rust-introduction

## Rust 설치 방법
- \(Windows\)rustup.rs에서 rustup-init.exe 파일 다운로드 후 설치
  - https://rust-kr.org/pages/install/
  - https://learn.microsoft.com/ko-kr/windows/dev-environment/rust/setup
    - C++ 빌드 도구가 필요하므로 Visual Studio를 설치하며 Desktop development with C++을 함께 설치
    - rustup-init.exe 설치 파일 실행 시 Install the C++ build tools before proceeding. 문구가 출력된다면, 반드시 Microsoft C++ Build Tools가 필요
    - \(참고\) https://velog.io/@pikamon/Rust-1
- rustc --version → Rust 컴파일러 버전 확인 명령어
- cargo --version → cargo(Rust 패키지 관리자) 버전 확인 명령어
- cf. \(Rust 및 관련 툴 삭제\) rustup self uninstall

## cargo 이용, Rust 프로젝트 생성, 빌드, 실행 방법
- cargo new \[프로젝트명\] → \[프로젝트명\] 프로젝트 생성
  - Cargo.toml 파일: 프로젝트 설정 파일(ex. npm 이용 시 package.json, Gradle 이용 시 settings.gradle + build.gradle 같은 역할)
  - Cargo.lock 파일: 프로젝트 dependencies 정확한 버전을 추적 - 사용자가 관리하는 파일이 아닌 cargo가 관리하는 파일
- cargo build → 프로젝트 빌드: \[프로젝트 경로\]/src에 있는 소스 코드를 \[프로젝트 경로\]/target/debug 경로에 컴파일
  - cf. cargo check → 컴파일 성공 여부 확인, binary는 생성하지 않음, build보다 빠름
  - cf. cargo build --release → 릴리즈 모드 빌드, target/debug가 아닌 target/release에 binary 생성 (참고: https://velog.io/@keum0821/Rust-Cargo-알아보기)
- cargo run → 프로젝트 실행: 빌드 및 \[프로젝트 경로\]/target에 있는 binary 실행
  - cargo build 후 소스 코드를 변경하지 않았다면 다시 컴파일 하진 않음
  - VSCode 사용 시 rust-analyzer 확장이 설치된 상태라면 rs 파일 위 Run 버튼을 눌러 실행 가능

## VSCode Rust 관련 확장
- rust-analyzer: Rust language support for Visual Studio Code
- CodeLLDB: A native debugger powered by LLDB. Debug C++, Rust and other compiled languages.
