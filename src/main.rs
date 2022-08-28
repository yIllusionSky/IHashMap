mod hashmap;
use hashmap::IHashMap;
fn main()
{
    let mut eq=IHashMap::<String>::new();
    eq.insert("蔡徐坤",String::from("一名打篮球及其厉害的鸡哥"));
    eq.insert("乔碧萝",String::from("一名化妆及其厉害的颜值主播"));
    println!("{:?}",eq.get("乔碧萝"));
}