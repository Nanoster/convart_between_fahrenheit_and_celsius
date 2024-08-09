use std::io::{self, Write};

fn main() {
    // 온도 변환 기능 안내 및 선택지 제시 -> 온도 변환 실행 및 출력
    // 종료 선택지 입력이 올때까지 반복
    loop {
        // 선택지 입력 안내문 출력,  입력값 유효성 검사
        let select = select_input();

        // 선택지에 대한 행동
        match select {
            // 1 혹은 2일때는 온도 변환 수행
            1 | 2 => {
                let temperature = temperature_input();

                // 온도 변환 수행
                let convert_temperature = if select == 1 {
                    convert_temperature_f_to_c(temperature)
                } else {
                    convert_temperature_c_to_f(temperature)
                };

                // 변환온도 출력
                print_convert_temperature(convert_temperature);
            }

            // 3의 경우 루프 탈출
            3 => break,
            _ => unreachable!(), // 선택지 입력부분에서 이미 검증을 끝냈기 때문에 도달할 수 없음
        }
    }
}

// 선택지 입력 함수
// 안내문 출력 -> 입력(유효성 검사) -> 입력값 반환
// 올바른 입력이 올때까지 반복
fn select_input() -> u32 {
    loop {
        // 선택지 안내문 출력
        print!(
            "
        본 프로그램은 화씨와 섭씨를 변환해주는 프로그램입니다.
        
        1. 화씨(˚F)에서 섭씨(˚C)로 변환
        2. 섭씨(˚C)에서 화씨(˚F)로 변환
        3. 종료
            
        사용할 기능을 선택해 주세요 : "
        );
        io::stdout().flush().expect("failed to flush stdout"); // flush를 호출하여 출력이 즉시 표시되도록 함 - 커서 문제때문에 넣었음

        // 입력
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // 유효성 검사
        // 1~3 사이의 숫자면 반환하고 함수종료. 아니면 안내문 출력하고 반복
        match input.trim().parse::<u32>() {
            Ok(select) if (1..=3).contains(&select) => return select,
            _ => println!("1과 3사이의 숫자를 입력해주세요."),
        }
    }
}

// 온도 입력 함수
// 안내문 출력 -> 입력(유효성 검사) -> 입력값 반환
// 올바른 입력이 올때까지 반복
fn temperature_input() -> f64 {
    loop {
        // 온도 입력 안내문 출력
        print! {"온도를 입력해주세요 : "};
        io::stdout().flush().expect("failed to flush stdout"); // flush를 호출하여 출력이 즉시 표시되도록 함 - 커서 문제때문에 넣었음

        // 입력
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // 유효성 검사
        match input.trim().parse::<f64>() {
            Ok(temperature) => return temperature,
            Err(_) => println!("올바른 온도를 입력해주세요."),
        };
    }
}

// 화씨에서 섭씨로 변환하는 함수
fn convert_temperature_f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// 섭씨에서 화씨로 변환하는 함수
fn convert_temperature_c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// 변환된 온도를 출력해주는 함수
fn print_convert_temperature(convert_temperature: f64) {
    println!("변환된 온도 : {convert_temperature:.1}");
}
