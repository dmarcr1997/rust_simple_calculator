use std::io;
enum CalcType{
    M,
    D,
    A,
    S
}
struct Calculations{
    number1: u32,
    number2: u32,
    answer: u32
}
impl Calculations{
    fn setNums (&mut self, num1: u32, num2: u32){
        self.number1 = num1;
        self.number2 = num2;
    }
    fn multiply(&mut self, num1: u32, num2: u32) -> u32{
        self.answer = num1 * num2;
        println!("{} x {} = {}", num1, num2, self.answer)
    }
    fn divide(&mut self, num1: u32, num2: u32) -> u32{
        if(num2 == 0){
            return "Cannot Divide by Zero!"
        }
        else{
            self.answer = num1 / num2;
            println!("{} / {} = {}", num1, num2, self.answer)
        }
    }
    fn addition(&mut self, num1: u32, num2: u32) -> u32{
        self.answer = num1 + num2;
        println!("{} + {} = {}", num1, num2, self.answer)
    }
    fn subtract(num1: u32, num2: u32) -> u32{
        self.answer = num1 - num2;
        println!("{} - {} = {}", num1, num2, self.answer)
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
        getNums(&op);        
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

fn getNums(choice: &String) -> u32{
    match choice{
        CalcType::M =>  
    }
}
