include!(concat!(env!("OUT_DIR"), "/words.rs"));

#[inline(never)]
fn get_word(i: usize) -> String {
    String::from(WORDS[i])
}

fn main() {
    let i = 123;
    println!("Word {}: {}", i, get_word(i));
}
