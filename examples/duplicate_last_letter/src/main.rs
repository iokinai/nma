use nma_command::command;
use nma_models::Command;

fn main() {
    let word = String::from("aabbababa");

    let result = nma::run(&word, &vec![command!("*a" => "a*"), command!("*b" => "b*"), command!("a*" !=> "aa"), command!("b*" !=> "bb"), command!("*" !=>), command!(=> "*")]);

    println!("{}", result);
}
