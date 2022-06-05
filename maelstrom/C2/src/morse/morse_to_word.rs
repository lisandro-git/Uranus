pub fn decode(input: Vec<u8>) -> Vec<u8> {
    let input_string = String::from_utf8(input).unwrap();

    let text = input_string.replace("*", ".").trim().to_string();
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
                    return vec![];
                }
            };
            result.push(letter);
        }
        result.push(' ');
    }
    result.pop();
    return result.into_bytes().to_owned();
}
