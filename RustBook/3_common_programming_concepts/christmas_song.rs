fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", 
    "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("Welcome to 'The Twelve Days of Christmas'!\n");

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("My true love gave to me:");

        for gift_day in (0..=day).rev() {
            if day == 0 && gift_day == 0 {
                println!("{}.", gifts[gift_day]);
            } else {
                println!("{}{}", if gift_day == 0 && day != 0 { "And " } else { "" }, gifts[gift_day]);
            }
        }

        println!();
    }
}
