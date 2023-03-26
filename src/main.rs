fn main() {
    println!("The 12 days of christmas song:");
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
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
    for i in 0..days.len() {
        println!("\nOn the {} day of Christmas, my true love gave to me:", days[i]);
        (0..=i).rev().for_each(|j| {
            if i == 0 && j == 0 {
                println!("{}", gifts[j]);
            };
        });
    };
}
