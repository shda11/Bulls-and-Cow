use std::io;
use std::array;
use rand::Rng;

struct PrintProtocol {
    strike: i32,
    ball: i32
}

struct CheckProtocol {
    answer: [i32; 4],
    guess: [i32; 4]
}

impl CheckProtocol {

    fn start(&mut self) {
        for i in 1..5 {
            self.answer[i-1] = rand::thread_rng().gen_range(1, 9);
        }
    }

    fn check_answer(&mut self) -> PrintProtocol {
        let mut protocol = PrintProtocol {
            strike: 0,
            ball: 0
        };
        for j in 1..5 {
            if self.guess[j-1] == self.answer[j-1] {
                protocol.strike += 1
            } else {
                if self.guess[j-1] == self.answer[1] || self.guess[j-1] == self.answer[2] || self.guess[j-1] == self.answer[3] ||  self.guess[j-1] == self.answer[0] { 
                    protocol.ball += 1
                }
            }
        }
        return protocol;
    }
}

fn main() {
    let mut secrete = CheckProtocol{answer: [0; 4], guess: [0; 4]};
    CheckProtocol::start(&mut secrete);
    
    println!("Let`s play a Bulls and Cow!");
    loop {
        let mut question = String::new();
        println!("Guess the number array");
        io::stdin().read_line(&mut question).expect("Failed to read line");
        println!("{:?}", secrete.answer);
        for i in 0..4 {
            let a: u8 = question.as_bytes()[i];
            let b: char = a as char;
            let c: i32 = b as i32 - 48;
            secrete.guess[i] = c;
        }

        let result = secrete.check_answer();
        println!("{} strike, and {} ball", result.strike, result.ball);
    
        if result.strike == 0 {
            println!("Sorry...but it`s 'Out'.🥺 Try again.")
        }

        if result.strike == 4 {
            println!("🎉Congratulation! You won!🎉");
            break
        }
    }
}