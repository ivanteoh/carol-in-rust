fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let phases = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for number in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[number]);
        if number == 0 {
            println!("A {}", phases[number]);
        } else {
            for index in (0..number+1).rev() {
                if index == 0 {
                    println!("And a {}.", phases[index]);
                } else {
                    println!("{},", phases[index]);
                }
            }
        }
    }
}
