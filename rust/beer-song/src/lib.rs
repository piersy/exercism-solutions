// I think the verses below cover all the edge cases
//
// 3 bottles of beer on the wall, 3 bottles of beer.
// Take one down and pass it around, 2 bottles of beer on the wall.

// 2 bottles of beer on the wall, 2 bottles of beer.
// Take one down and pass it around, 1 bottle of beer on the wall.

// 1 bottle of beer on the wall, 1 bottle of beer.
// Take it down and pass it around, no more bottles of beer on the wall.

// No more bottles of beer on the wall, no more bottles of beer.
// Go to the store and buy some more, 99 bottles of beer on the wall.

pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    format!("{0} of beer on the wall, {0} of beer.\nTake {1} down and pass it around, {2} of beer on the wall.\n", bottle_string(n),one_string(n),bottle_string(n-1))
}

pub fn sing(start: u32, end: u32) -> String {
    let result = (end..=start)
        .rev()
        .fold(String::new(), |acc, v| acc + &verse(v) + "\n");
    result[..result.len() - 1].to_string()
}

fn one_string(n: u32) -> &'static str {
    if n > 1 {
        return "one";
    } else {
        return "it";
    }
}

fn bottle_string(n: u32) -> String {
    match n {
        0 => String::from("no more bottles"),
        1 => format!("{} bottle", n),
        _ => format!("{} bottles", n),
    }
}
