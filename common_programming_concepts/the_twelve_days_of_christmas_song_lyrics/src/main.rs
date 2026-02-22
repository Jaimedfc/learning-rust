fn main() {
    println!("Sing with me!");
    println!();

    const DAYS_N_GIFTS: [(&str, &str); 12] = [
        ("first", "a partridge in a pear tree."),
        ("second", "two turtle doves,"),
        ("third", "three French hens,"),
        ("fourth", "four calling birds,"),
        ("fifth", "five gold rings,"),
        ("sixth", "six geese a-laying,"),
        ("seventh", "seven swans a-swimming,"),
        ("eighth", "eight maids a-milking,"),
        ("ninth", "nine ladies dancing,"),
        ("tenth", "ten lords a-leaping,"),
        ("eleventh", "eleven pipers piping,"),
        ("twelfth", "twelve drummers drumming,"),
    ];

    for (i, (day, gift)) in DAYS_N_GIFTS.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love sent to me {gift}");

        for j in (0..i).rev() {
            if i == 0 {
                break;
            }
            if j == 0 {
                print!("and ");
            }
            let prev_gift = DAYS_N_GIFTS[j].1;
            println!("{prev_gift}");
        }
        println!();
    }
}
