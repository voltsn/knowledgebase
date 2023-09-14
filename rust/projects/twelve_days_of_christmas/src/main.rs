fn main() {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-layin",
        "Seven swans a-swimming",
        "Eight maids a-miliking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drumers dumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fith", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut output: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < 12 {
        println!("On the {} day of Christmas my true love sent me", days[i]);

        output.insert(0, lyrics[i]);
        println!("{}", output.join("\n"));
        println!("");
        i += 1;
    }
}
