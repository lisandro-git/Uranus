use super::*;

pub fn decode(input: String) -> String {
    let text = input.replace("*", ".").trim().to_string();
    let mut result = String::new();
    let words = text.split("/");
    for word in words {
        let chars = word.trim().split(" ");
        for c in chars {
            let letter = match c {
                ".-" => 'A',
                "-..." => 'B',
                "-.-." => 'C',
                "-.." => 'D',
                "." => 'E',
                "..-." => 'F',
                "--." => 'G',
                "...." => 'H',
                ".." => 'I',
                ".---" => 'J',
                "-.-" => 'K',
                ".-.." => 'L',
                "--" => 'M',
                "-." => 'N',
                "---" => 'O',
                ".--." => 'P',
                "--.-" => 'Q',
                ".-." => 'R',
                "..." => 'S',
                "-" => 'T',
                "..-" => 'U',
                "...-" => 'V',
                ".--" => 'W',
                "-..-" => 'X',
                "-.--" => 'Y',
                "--.." => 'Z',
                "..---" => '2',
                "...--" => '3',
                "....-" => '4',
                "....." => '5',
                "-...." => '6',
                "--..." => '7',
                "-...-" => '=',
                "/" => ' ',
                _ => {
                    println!("Could not parse: {}", c);
                    return String::new();
                }
            };
            result.push(letter);
        }
        result.push(' ');
    }
    result.pop();
    return result;
}