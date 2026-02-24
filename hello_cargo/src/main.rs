fn main() {
    println!("Hello, world!");
}

/* [cargo 명령어]
 *  1. cargo new <project-name>
 *    -> 프로젝트 생성
 *    -> --options 확인 : cargo new --help
 * 
 *  2. cargo build
 *    -> 프로젝트 빌드
 *    -> --options
 *      1) --release  (배포용 빌드) => 빌드 속도 Down / 성능 최적화 O
 *      2) --debug  (기본값) => 빌드 속도 Up / 성능 최적화 X
 *    -> 빌드된 파일 실행 : ex) ./target/debug/hello_cargo
 * 
 *  3. cargo run
 *    -> 프로젝트 빌드 및 실행
 *    -> 주로 개발 중 사용 명령어
 * 
 *  4. cargo check
 *    -> 프로젝트 빌드 없이 컴파일 오류 체크
 *    -> 주로 개발 중 사용 명령어
 * 
 *    
 */