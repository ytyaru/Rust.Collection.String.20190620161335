/*
 * RustのコレクションString型。
 * String型はchar型の可変配列。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html
 * http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
 */
fn main() {
    let mut s = String::new();
    let mut s = "init".to_string();
    let mut s = String::from("init");
    println!("{}", s);

    // 末尾に追記
    s.push('A'); // 引数はchar(文字列型でない)
    println!("{}", s);
    s.push_str("BC");
    println!("{}", s);

    let s1 = "DEF";
    s.push_str(s1); // push_str()はs1の所有権を奪わない
    println!("{} {}", s, s1);
}

