
pub fn verse(n: u32) -> String {
    match n {
        (3..=99) => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        _ => panic!("Only values between 0 and 99.")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = "".to_owned();
    for n in (end..=start).rev() {
        println!("{:?}", n);
        result.push_str(&verse(n));
        result.push_str("\n");
    }
    result.to_owned()
}


fn main() {
    print!("{}", sing(8,6));


}
