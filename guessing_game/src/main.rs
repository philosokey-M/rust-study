use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 범위 표현식: start..=end -> start부터 end까지 범위 | ex) 1..=100 -> 1부터 100까지 범위
    
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // rust는 기본적으로 변수가 불변 -> 가변으로 쓰고 싶으면, mut 키워드 사용

    io::stdin()
        .read_line(&mut guess) // read_line() : 사용자 입력을 읽어오는 함수 -> 입력값 저장 및 Result 타입의 값(Ok | Err) 반환
        .expect("Failed to read line"); // read_line() 함수 호출 결과 Err 인 경우 처리 -> expect() : 에러 메시지 출력 및 프로그램 종료

    let guess: u32 = guess.trim()
                    .parse()
                    .expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
/* <:: 표시>
 * -> 경로 구분자
 * 
 * 연관 함수 : 어떤 타입에 구현된 함수 -> 타입::함수명() 형태로 호출
 * ex) String::new()
 * new() : 비어있는 새 문자열을 생성
 */