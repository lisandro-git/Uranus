use super::*;

pub fn encode(input: String) -> String {
    let words = input.trim().to_string();
    let text = words.chars();
    let mut result = String::new();
    for t in text {
        let morse_letter = match t {
            'A' => ".-",
            'B' => "-...",
            'C' => "-.-.",
            'D' => "-..",
            'E' => ".",
            'F' => "..-.",
            'G' => "--.",
            'H' => "....",
            'I' => "..",
            'J' => ".---",
            'K' => "-.-",
            'L' => ".-..",
            'M' => "--",
            'N' => "-.",
            'O' => "---",
            'P' => ".--.",
            'Q' => "--.-",
            'R' => ".-.",
            'S' => "...",
            'T' => "-",
            'U' => "..-",
            'V' => "...-",
            'W' => ".--",
            'X' => "-..-",
            'Y' => "-.--",
            'Z' => "--..",
            '2' => "..---",
            '3' => "...--",
            '4' => "....-",
            '5' => ".....",
            '6' => "-....",
            '7' => "--...",
            '=' => "-...-",
            ' ' => "/",
            _ => {
                println!("Could not parse: {}", t);
                return String::new();
            }
        };
        result.push_str(morse_letter);
        result.push(' ');
    }
    result.pop(); // edode : removing the trailing "/"
    return result;
}