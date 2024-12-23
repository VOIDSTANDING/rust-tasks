fn main() {
    let day = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];
    
    let text = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese-a-laying",
        "seven swans-a-swimming",
        "eight maids-a-milking",
        "nine drummers drumming",
        "ten pipers piping",
        "eleven ladies dancing",
        "twelve lords-a-leaping",
    ];

    for i in 0..12 {
        println!("\x1b[36;1mOn the {} day of Christmas, my true love sent to me:\x1b[0m", day[i]);

        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                println!("and {}", text[j]);
            } 
            else {
                println!("{}", text[j]);
            }
        }

    }
}