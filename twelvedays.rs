use std::ops::Index;

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (i, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas my true love gave to me, ");
        for gift in gifts[0..i + 1].iter().rev() {
            if *gift == "a partridge in a pear tree" {
                println!("{gift}.");
            } else if *gift == "two turtle doves" {
                println!("{gift}, and")
            } else {
                println!("{gift},")
            }
        }
    }
}
