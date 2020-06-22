use std::io;
struct Calculations{
    number1: u32,
    number2: u32,
    answer: u32
}
impl Calculations{
    fn multiply(&mut self){
        self.answer = self.number1 * self.number2;
        println!("{} x {} = {}", self.number1, self.number2, self.answer);
    }
    fn divide(&mut self){
        if self.number2 == 0{
            println!("Cannot Divide by Zero!");
        }
        else{
            self.answer = self.number1 / self.number2;
            println!("{} / {} = {}", self.number1, self.number2, self.answer);
        }
    }
    fn addition(&mut self){
        self.answer = self.number1 + self.number2;
        println!("{} + {} = {}", self.number1, self.number2, self.answer)
    }
    fn subtract(&mut self){
        self.answer = self.number1 - self.number2;
        println!("{} - {} = {}", self.number1, self.number2, self.answer)
    }
}
fn main() {
    println!("Hello, Here my Simple Calculator");
    loop{
        let mut op = String::new();
        println!("Enter Name of Operator You want to use(A/S/D/M):");
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line");
        let op: char = op.trim().parse().expect("character");
        let mut calc_struct = getNums();
        make_choice(op, &mut calc_struct);        
        println!("Would You like to convert again(y/n): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: char = choice.trim().parse().expect("character");
        if choice != 'y'{
            break;
        }
    }
}

fn getNums() -> Calculations{
    println!("Enter Your First Number:");
    let mut numb1 = String::new();
    io::stdin()
        .read_line(&mut numb1)
        .expect("Failed to read line");
    let numb1: u32 = numb1.trim().parse().expect("Please type a number!");
    println!("Enter Your Second Number:");
    let mut numb2 = String::new();
    io::stdin()
        .read_line(&mut numb2)
        .expect("Failed to read line");
    let numb2: u32 = numb2.trim().parse().expect("Please type a number!");
    let zero: u32 = 0;
    let newCalc = Calculations{
        number1: numb1,
        number2: numb2,
        answer: zero
    };
    return newCalc
}

fn make_choice(op: char, calc: &mut Calculations){
    if op == 'M'{
        println!("YES");
    }
    match op{
        'M' => calc.multiply(),
        'D' => calc.divide(),
        'A' => calc.addition(),
        'S' => calc.subtract(),
        _ => println!("DIDNT MATCH")
    };
}
