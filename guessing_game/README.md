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

一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。Rust 获取提供给 match 的值并挨个检查每个分支的模式。

* 静态强类型系统

```guess.cmp(&secret_number)```这里编译会报```mismatched types```( 不匹配的类型)，```guess``` 是 ```String``` 类型，而```secret_number```，是数字类型。

```Rust
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

这里创建了一个叫做 guess 的变量。不过等等，不是已经有了一个叫做 guess 的变量了吗？确实如此，不过 Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值。这个功能常用在需要转换值类型之类的场景。它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类。

* 使用循环来允许多次猜测

```Rust
    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
```

* 猜测正确后退出

```
  match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
```

* 处理无效输入

```Rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
将 expect 调用换成 match 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。须知 parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。这里使用的 match 表达式，和之前处理 cmp 方法返回 Ordering 时用的一样。












