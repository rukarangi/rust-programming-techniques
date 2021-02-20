fn hio(i: i32) -> Result<i32, String> {
    match i {
        i if i >= 0 => Ok(i + 10),
        _ => Err(format!("Input to h less than 0, found: {}", i))
    }
}

fn main() {
    // ---- result example -----
    let input: i32 = 2;
    match hio(input) {
        Ok(i) => println!("Result: {}", i),
        Err(e) => println!("Error: {}", e)
    }
    // let i = hio(input)?; - toit syntax
    // can change to catch errors on lots of functions
    // println!("{}", i);
    // ----
}
