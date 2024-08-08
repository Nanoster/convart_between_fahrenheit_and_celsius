use std::io;

fn main() {

    select_info();
    let input = select_input();
    println!("selected number: {input}");
    // temperature_info();
    // temperature_input();
    // convert_temperature_f_to_c();
    // convert_temperature_c_to_f();
    // print_convert_temperature();
    
}

fn select_info(){
    print!("본 프로그램은 화씨와 섭씨를 변환해주는 프로그램입니다.
 
    1. 화씨(˚F)에서 섭씨(˚C)로 변환
    2. 섭씨(˚C)에서 화씨(˚F)로 변환
    3. 종료

    사용할 기능을 선택해 주세요 : ");
    
}

fn select_input() -> i32{
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("failed to read from stdin");

    let select = str.trim().parse::<i32>().unwrap();

    select
}