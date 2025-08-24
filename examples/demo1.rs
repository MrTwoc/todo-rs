fn main() {
    println!("u8_max:{}", u8::MAX);
    let _s: MyType = 100.into();
}

struct MyType;
impl From<i32> for MyType {
    fn from(item: i32) -> Self {
        println!("from i32:{}", item);
        MyType
    }
}
