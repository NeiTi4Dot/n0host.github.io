fn main() {

    // (let) Variable "v" with: type = "string", content "This is variable with type String with content."
    let v: String = String::from("This is variable with: type 'String' and with content.");

    println!("v: {}", v);
 
    // Using syntax "as" 
    let x = 25 as u64;
    let y = (x + 2) as u64;

    println!("x: {}, y: {}", x, y);
 
    // Using synax ".into()"
    let z: i32 = 75;
    let c: i64 = z.into();

    println!("z: {}, c: {}", z, c);

}
