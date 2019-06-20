/*
 * RustのコレクションString型。
 * String型はchar型の可変配列。
 * CreatedAt: 2019-06-20
 * https://doc.rust-jp.rs/book/second-edition/ch08-02-strings.html
 * http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
 */
fn main() {
    // +演算子は所有権を奪う（先頭にある変数の）
    let s1 = "A".to_string();
    let s2 = "B".to_string();
    let s3 = "C".to_string();
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // format!()は所有権を奪わない
    let s1 = "A".to_string();
    let s2 = "B".to_string();
    let s3 = "C".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

