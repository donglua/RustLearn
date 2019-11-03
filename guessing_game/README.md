# 编写 猜猜看 游戏

```Rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

* 使用变量储存值

let mut guess 会引入一个叫做 guess 的可变变量。等号（=）的右边是 guess 所绑定的值，它是 String::new 的结果，这个函数会返回一个 String 的新实例。String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块。

如果程序的开头没有 use std::io 这一行，可以把函数调用写成 std::io::stdin。stdin 函数返回一个 std::io::Stdin 的实例，这代表终端标准输入句柄的类型。

* 使用 Result 类型来处理潜在的错误

read_line 将用户输入附加到传递给它的字符串中，不过它也返回一个值——在这个例子中是 io::Result。

```Rust
.expect("Failed to read line");
```

* 生成一个秘密数字

引入一个 rand 依赖
```
[dependencies]

rand = "0.5.5"
```

生成随机数
```
use rand::Rng;

fn main() {
    // 范围应该在 1 到 100 之间
    let number = rand::thread_rng().gen_range(1, 101);

}
```

* 比较猜测的数字和秘密数字

```Rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。Rust 获取提供给 match 的值并挨个检查每个分支的模式。match 结构和模式是 Rust 中强大的功能，它体现了代码可能遇到的多种情形，并帮助你确保没有遗漏处理。这些功能将分别在第六章和第十八章详细介绍。







