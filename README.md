# Markov algorithm in Rust

### What is it?

In theoretical computer science, a Markov algorithm is a string rewriting system that uses grammar-like rules to operate on strings of symbols. Markov algorithms have been shown to be Turing-complete, which means that they are suitable as a general model of computation and can represent any mathematical expression from its simple notation. Markov algorithms are named after the Soviet mathematician Andrey Markov, Jr. 
[Wikipedia](https://en.wikipedia.org/wiki/Markov_algorithm)

### Project structure

#### nma 
Main library project which contains function ***run*** - main function used to execute the Markov algorithm on a given word. It takes 2 arguments: ***original_word: &String*** and ***commands: &Vec<Command>***
* ***original_word: &String***: the word to modificate with algorithm
* ***commands: &Vec<Command>***: the vector of commands that modificate the word. Commands must be applied in the correct order

#### nma_models
Library that contains models, the main of which is ***Command***
It ha 3 fields:
* ***pub find: String***: the word to find
* ***pub change: String***: the word to change the found one with
* ***pub exit: bool***: the bool witch defines if the program should end after this command. ***true*** if yes, else ***false***

And one method: 
* ***pub fn new(find: String, change: String, exit: bool) -> Command*** which creates a new ***Command***

But lets be honest - using this method to create each one ***Command*** is not really convenient. For example, we want to create algorithm which duplicates the last letter in word of {a, b} symbols. Standard algorithm which does this looks like:

\*a -> a*\
\*b -> b*\
a* !-> aa\
b* !-> bb\
\* -> \
 -> * 

If we implement commands by creating structs it will look like:

***vec![Command::new("\*a", "a\*", false), Command::new("\*b", "b\*", false), Command::new("a*", "aa", true), Command::new("b*", "bb", true), Command::new("\*", "", true), Command::new("", "*", false)]***

Not really convenient, isn't it? Thats why ***nma_command*** exists!

#### nma_command

It implements the macro ***command!***. Inside it you should write the command in default MA syntax: 
* ***str*** for words to find and to change, for example: ***"\*a"***
* => to indicate which word to change from which to which
* !=> same as previous but also stops the program after executing
* \<nothing\> for an empty symbol, for example ***command!(=> "a")***, ***command!("a" =>)***

For example, ***command!("aa" => "bb")*** is equivalent to ***Command::new("aa", "bb", false)*** and ***command!("aa" !=> "bb")*** is equivalent to ***Command::new("aa", "bb", true)***


### Examples

For example, here is the program that duplicates last letter:

```rust
use nma_command::command;
use nma_models::Command;

fn main() {
    let word = String::from("aabbababa");

    let result = nma::run(&word, &vec![command!("*a" => "a*"), command!("*b" => "b*"), command!("a*" !=> "aa"), command!("b*" !=> "bb"), command!("*" !=>), command!(=> "*")]);

    println!("{}", result);
}
```
