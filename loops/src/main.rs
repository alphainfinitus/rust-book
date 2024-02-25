fn main() {
    counting_up_loop();
    println!("\n");

    while_loop();
    println!("\n");

    for_loop();
    println!("\n");

    ranged_for_loop();
    println!("\n");

    twelve_days_of_christmas();
    println!("\n");

    println!("All Loops Executed!");
}

fn counting_up_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining  = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn while_loop() {
    let mut number = 5;

    while number != 0 {
        println!("Number is {number}");
        number -= 1;
    }

    println!("I have looped myself!");
}

fn for_loop() {
    const ARR: [usize; 10] = [5; 10]; // don't really need this array, just testing :P

    let mut multiplier: usize = 1;

    for element in ARR {
        println!("{element} times {multiplier} is {}", multiplier * element);
        multiplier += 1;
    }
}

fn ranged_for_loop() {
    for number in 1..5 {
        println!("looped {number} times today");
    }

    println!("Done loopin for the day. Or am I ?")
}

fn twelve_days_of_christmas() {
    const LYRICS: [(&str, &str); 12] = [
        ("first", ""),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five golden rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    // ik i'm not supposed to "know" all of this at this stage of the book;
    // but I  got curious so just implementing stuff

    let mut lyrics_vector: Vec<&str> = Vec::new();

    for day_lyrics in LYRICS {
        let (day_name, day_lyric) = day_lyrics;

        lyrics_vector.push(day_lyric);

        print!("On the {day_name} day of Christmas,\nmy true love gave to me");

        for lyric in lyrics_vector.iter() {
            println!("{lyric}");
        }

        println!(
            "{} partridge in a pear tree.\n",
            if day_name == "first" { "A" } else { "And a" }
        );
    }
}
