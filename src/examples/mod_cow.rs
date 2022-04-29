use std::borrow::Cow;

// Cow(Clone-on-Write) 版本
pub fn insert_prefix_cow<'a, T>(strs: T, prefix: &'a str) -> Vec<Cow<'a, String>>
where
    T: IntoIterator<Item = &'a String>,
{
    strs.into_iter()
        .map(|s| -> Option<Cow<String>> {
            if let true = s.starts_with(prefix) {
                Some(Cow::Borrowed(s))
            } else {
                Some(Cow::Owned(String::with_capacity(prefix.len() + s.len()) + prefix + s))
            }
        })
        .map(|item| item.unwrap())
        .collect::<Vec<Cow<'a, String>>>()
}

// Clone 版本
#[allow(unused)]
pub fn insert_prefix_clone<'a>(strs: impl IntoIterator<Item = &'a String>, prefix: &'a str) -> Vec<String> {
    strs.into_iter()
        .map(|s| match s.starts_with(prefix) {
            true => Some(s.clone()),
            false => Some(String::with_capacity(prefix.len() + s.len()) + prefix + s),
        })
        .map(|item| item.unwrap())
        .collect::<Vec<String>>()
}

pub fn run() {
    println!("Server is started.");
    // std::thread::sleep(std::time::Duration::from_secs(2000));
    let strs = vec!["row_rust".to_string(), "rust".to_string()];
    let p = "row_";
    let fixed = insert_prefix_cow(&strs, p);

    let s0 = &strs[0]; // 第一个元素已经有指定前缀名了
    let f0 = &*fixed[0]; // Cow实现了Deref, 所以可以直接解引用。

    println!("source addr: {:?}", s0 as *const String); //    0x55aca68ac0
    println!("   cow addr: {:?}", f0 as *const String); //    0x55aca68ac0, 地址相同

    let s1 = &strs[1]; // 第二个元素插入了前缀名
    let f1 = &*fixed[1];

    println!("source addr: {:?}", s1 as *const String); //    0x55aca68ad8
    println!("   cow addr: {:?}", f1 as *const String); //    0x55aca68b88, 地址已经发生了变化
}
