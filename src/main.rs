use std::io;

fn main() {
        println!("Please input your string.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("Your input was: {}",input);
        word_count(&input);
}

fn word_count(s:&str) -> usize{
        let count =  s.split_whitespace().count();
        println!("Your word count was: {}", count);
        count    
}