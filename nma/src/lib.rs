use nma_models::Command;

pub fn run(original_word: &String, commands: &Vec<Command>) -> String {
    let mut str = original_word.clone();
    let mut i = 0;

    while i < commands.len() {
        if commands[i].find == "" {
            str.insert_str(0, commands[i].change.as_str());
            i = 0;
            continue;
        }

        let ind = str.find(commands[i].find.as_str());

        match ind {
            Some(v) => { 
                str = change(&str, v, commands[i].find.len(), &commands[i].change.to_string());

                if commands[i].exit {
                    break;
                }

                i = 0;
            },

            None => i += 1,
        }
    }

    str
}

pub fn change(word: &String, index: usize, substring_len: usize, change: &String) -> String {
    let mut str = String::new();

    let startw = &word[0..index];

    str.push_str(startw);
    str.push_str(&change);
    str.push_str(&word[index+substring_len..]);

    str
}