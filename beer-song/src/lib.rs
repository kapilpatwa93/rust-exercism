const BOTTLES_OF_BEER: &str = "bottles of beer";
const BOTTLE_OF_BEER: &str = "bottle of beer";
const BOTTLES_ON_THE_WALL: &str = "bottles of beer on the wall";
const BOTTLE_ON_THE_WALL: &str = "bottle of beer on the wall";
const TAKE_AND_PASS: &str = "Take one down and pass it around";
const TAKE_IT_AND_PASS: &str = "Take it down and pass it around";
const GO_TO_STORE: &str = "Go to the store and buy some more";

fn get_verse(bottle: u32) -> String {
    match bottle {
        0 => format!(
            "{} {}, {} {}.\n{}, {} {}.\n",
            "No more",
            BOTTLES_ON_THE_WALL,
            "no more",
            BOTTLES_OF_BEER,
            GO_TO_STORE,
            "99",
            BOTTLES_ON_THE_WALL
        ),
        1 => format!(
            "{} {}, {} {}.\n{}, {} {}.\n",
            "1",
            BOTTLE_ON_THE_WALL,
            "1",
            BOTTLE_OF_BEER,
            TAKE_IT_AND_PASS,
            "no more",
            BOTTLES_ON_THE_WALL
        ),
        2 => format!(
            "{} {}, {} {}.\n{}, {} {}.\n",
            bottle,
            BOTTLES_ON_THE_WALL,
            bottle,
            BOTTLES_OF_BEER,
            TAKE_AND_PASS,
            "1",
            BOTTLE_ON_THE_WALL
        ),
        _ => format!(
            "{} {}, {} {}.\n{}, {} {}.\n",
            bottle,
            BOTTLES_ON_THE_WALL,
            bottle,
            BOTTLES_OF_BEER,
            TAKE_AND_PASS,
            bottle - 1,
            BOTTLES_ON_THE_WALL
        ),
    }
}
pub fn verse(n: u32) -> String {
    get_verse(n)
}
pub fn sing(start: u32, end: u32) -> String {
    let mut song: Vec<String> = vec![];
    for i in (end..start + 1).rev() {
        song.push(verse(i))
    }
    song.join("\n")
}
