use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("words.rs");
    let mut f = File::create(&dest_path).unwrap();
    let mut words = vec!();
    for i in 0..1000000 {
        let mut word = format!("word{}", i);
        words.push(word);
    }
    f.write_all(b"const WORDS: [&str; ").unwrap();
    f.write_all(format!("{}", words.len()).as_bytes()).unwrap();
    f.write_all(b"] = [\n").unwrap();
    for word in words.into_iter() {
        f.write_all(b"  r#\"").unwrap();
        f.write_all(word.as_bytes()).unwrap();
        f.write_all(b"\"#,\n").unwrap();
    }
    f.write_all(b"];\n").unwrap();
}
