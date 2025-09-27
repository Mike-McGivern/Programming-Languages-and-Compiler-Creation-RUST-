fn slicing() {
    let s = String::from("rusty things");
    let a = &s[0..5];
    println!("{a}");
}

fn main() {
    slicing();


}