use std::io::stdin;

use numbers_to_words::to_faroese_words;

fn main() {
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).unwrap();
        let n = s.replace(<char>::is_whitespace, "");
        let n = n.trim();

        if !n.is_empty() {
            println!("{}", to_faroese_words(n).unwrap_or_else(|| "Malformed number".to_owned()));
            s.clear();
        } else {
            break
        }
    }
}
