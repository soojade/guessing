extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字猜谜游戏!");
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("请输入你要猜的数字.");

        let mut guess = String::new();// 标准库提供的String字符串类型，是可增长 的utf-8的文本。 :: 关联String本身而不是实例，即静态方法。
        io::stdin().read_line(&mut guess) // stdin 返回一个指向终端标准输入的句柄。rust中的引用是不可变的，因此需要加上mut
            .expect("读取失败"); // 前面的调用会返回一个io::Result来处理错误信息，它有一个expect()方法获取调用它的值，如果程序崩溃，显示传递的信息

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; // trim去除开头、结尾的空格，通过parse解析成数字

        println!("你猜的数字是:{}", guess); // {}为占位符，保存传递的参数，如有多个参数，既使用多个{}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"), // Ordering是一个枚举
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }
}
