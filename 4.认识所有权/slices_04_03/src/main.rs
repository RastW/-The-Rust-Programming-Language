fn main() {
    let mut s = String::from("hello world");

    let wordindex = first_word(&s);

    s.clear();
    print!("{}", wordindex)
}

/**
 * 采用 &str 代替&String，这样可以同时接收&String和&str两种类型的擦书
 * 1、使用as_bytes方法转化为byte数组
 * 2、使用iter在byte数组上创建迭代器
 *      - iter方法返回集合的每一个元素，enumerate对结果进行包装
 * 3、通过字面值语法来寻找空格，如果找到一个空格则返回他的位置，否则返回s.len的长度
 */
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
