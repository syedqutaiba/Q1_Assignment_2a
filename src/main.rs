use std::io;

fn main() {
    question_1();
    question_2();
    question_3();
    question_4();
    question_5();
}

fn question_1() {

	let x = String::from("PIAIC IoT");
	let y = x;

	println!("\n------------");
    println!("Question # 1");
    println!("------------\n");
    
    println!("{}\n", y);
}

fn question_2() {
    let mut s = String::from("PAKISTAN");
    
    println!("------------");
    println!("Question # 2");
    println!("------------\n");

    change(&mut s);
    println!("{}\n", s);
}

fn change(text: &mut String) {
    text.push_str(" ZINDABAD"); 
}

fn question_3() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut number3 = String::new();

    println!("------------");
    println!("Question # 3");
    println!("------------\n");

    println!("Enter first number:");
    io::stdin().read_line(&mut number1).expect("Enter a valid number");
    println!("Enter first number:");
    io::stdin().read_line(&mut number2).expect("Enter a valid number");
    println!("Enter first number:");
    io::stdin().read_line(&mut number3).expect("Enter a valid number");

    let mut number1: f32 = number1.trim().parse().expect("Enter an integer number");
    let mut number2: f32 = number2.trim().parse().expect("Enter an integer number");
    let mut number3: f32 = number3.trim().parse().expect("Enter an integer number");

    let average = (number1 + number2 + number3) / 3.0;
    println!("Average of {}, {} and {} is {}\n", number1, number2, number3, average);
}

fn question_4() {
    let mut s1 = String::new();
    
    println!("------------");
    println!("Question # 4");
    println!("------------\n");
    
    println!("Enter text:");
    io::stdin().read_line(&mut s1).expect("Enter a valid input");
    let mut s1: String = s1.trim().parse().expect("Enter a valid string");

      
    let (s2, len) = calculate_length(s1);
    
    println!("The length of string '{}' is {}.\n", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn question_5() {
    let mut option = String::new();
    let mut counter = 1;
    let shape = '*';
     
        
    println!("------------");
    println!("Question # 5");
    println!("------------\n");
    
    println!("Enter a number:");
    io::stdin().read_line(&mut option).expect("Enter a valid number");
    let mut option: i32 = option.trim().parse().expect("Enter an integer");
     
    for number in 0..option {
        println!("{}", shape);
        counter = counter + 1;
    }      
}