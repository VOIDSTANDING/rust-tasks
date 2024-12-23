fn main() {
    let input = "first apple einz zwei drei vier ichi ni san chi Uno Dos Tres Quatro";
    let pig_latin = convert_to_pig_latin(input);
    println!("{}", pig_latin);
}

fn convert_to_pig_latin(input: &str) -> String {

    input.split_whitespace().map(|word| {
        
        let first_char = word.chars().next().unwrap();

        if "aeiou".contains(first_char) {
            format!("{}-hay", word)
        } 

        else if "AEIOU".contains(first_char) {
            format!("{}-HAY", word)
        } 
        
        else {
            format!("{}-{}ay", &word[1..], first_char)
        }

    }).collect::<Vec<_>>().join(" ")
}