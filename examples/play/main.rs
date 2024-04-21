use std::ops::Deref;

fn main() {
    println!("{}", 123);

    let a = MyU32(123);

    let s = String::from("456");

    let c: MyU32 = s.into();

    let d = MyU32::from("21521521".to_string());

    println!("{} {}", c.0, d.0);
}

struct MyU32(u32);

impl From<String> for MyU32 {
    fn from(value: String) -> Self {
        MyU32(value.parse().unwrap())
    }
}

// 定义一个结构体 MyString 包装了 String
struct MyString(String);

// 实现 AsRef<str> trait
impl AsRef<str> for MyString {
    fn as_ref(&self) -> &str {
        // 返回 String 的引用，通过解引用操作符转换为 &str
        &self.0
    }
}
