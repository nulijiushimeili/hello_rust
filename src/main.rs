extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
//    rand_number();
//    compare();
//    test_loop();
//    diverges();
//    // 发散函数可以被用作任何类型
//    let x : i32 = diverges();
//    let y : String = diverges();
//    let sum = func1(3,4);
//    println!("The sum : {}" , func1(5,6));
//    let f :fn(i32) -> i32 = plus_one;
//    // 把函数赋值给变量
//    let f = plus_one;
//    println!("{}",f(32))
//    test_arr1()
//    test_tuple();
//    test_if();
//    rust_loop();
//    test_iterator();
//    shutdown_iterator();
    test_loop_labels();

}

// 循环标签
fn test_loop_labels(){
    'outer: for x in 0 .. 10{
        'inner: for y in 0 .. 10{
            if x %2 == 0 {continue 'outer;}
            if y % 2 == 0 {continue 'inner;}
            println!("x: {}\t y: {}",x,y);
        }
    }
}


// 提早结束迭代
fn shutdown_iterator(){
    let mut x = 5;
//    let mut done = false;
//
//    while !done{
//        x += x-3;
//        println!("{}",x);
//        if x % 5 == 0{
//            done = true;
//        }
//    }

    // 这样写效果更好
    loop{
        x += x-3;
        println!("{}",x);
        if x % 5 == 0 {break;}
    }
}


// 迭代器
fn test_iterator(){
    let lines = "hello\nword!".lines();
    for(linenumber,line ) in lines.enumerate(){
        println!("{}:{}",linenumber,line);
    }
}


// 循环
fn rust_loop(){
    let  mut x = 5;
    // loop 远比 while true 更适合处理死循环
    loop{
        x -=1;
        println!("{}",x);
        if x <= 0{
            break;
        }
    }

    println!("------------------");
    // while
    while x <= 0{
        x -=1;
        println!("{}",x);
        if x <= 0{
            break;
        }
    }

    // for i
    println!("--------------------");
    for x in 0 .. 10{
        println!("{}",x);
    }

    println!("-------------------");
    // for each
    let arr = [3,4,5,6,7];
    for v in arr.iter(){
        println!("{}",v);
    }
}

// if
fn test_if(){
    let x = 5;
    if x == 5{
        println!("x is five.");
    }else if x == 6{
        println!("x is six.");
    }else{
        println!("x is not five or six.");
    }

//    let y = if x == 5 {
//        10
//    }else {
//        20
//    };
    let y = if x == 5 {10}else{20};
    println!("{}",y);
}


// 元祖
fn test_tuple(){
    let x = (1,"hello");
    let y:(i32,&str) = (2,"word");
    let mut z = (1,"rust");
    // 相同的元祖可以赋值
    z = x ;

    // 元祖的解构
    let (a,b,c ) = (4,5,6);
    println!("a is {}",a);

    // 元祖的索引
    let s = (3,4,5);
    let d = s.0;
    println!("{}",d);
    println!("{}",s.2);
}


// 数组的切片,切片的作用是访问数组的一部分
fn test_slice(){
    let a = [12,3,4,5,6,6];
    //  获取数组的整个视图
    let complete = &a[..];
    let middle = &a[2..4];
}


// 数组练习
fn test_arr1(){
    // 定义一个定长数组
    let a = [0,1,2,3,4];
    println!("{}",a[0]);
    println!("{}",a.len());
    let mut  b = [4,5,6];
    // 数组的泛型, i32数据类型,使用0填满20个位置
    let c = [0;20];
}

// 函数指针 let f = plus_one;
fn plus_one(i: i32) -> i32{
    i+1
}


// 发散函数,发散函数可以被用作任何类型,赋值给任何类型
fn diverges()->!{
    // panic!() 是导致当前线程奔溃,里面的信息是打印的返回的原因
    panic!("This is a function never returns!");
}

// sum
fn func1(x: i32, y :i32)->i32{
    x+y
}

// 循环猜数字
fn test_loop(){
    println!("Guess the number.");
    let rand_num = rand::thread_rng().gen_range(1,101);
    println!("The rand number is : {}" , rand_num);
    loop{
        println!("Please input your guess:");
        let mut guess = String::new ();
        io::stdin().read_line(& mut guess)
            .expect("Failed read line.");
//        let guess : u32 = guess.trim().parse()
//            .expect("Please type a number.");
        let guess : u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number:");
                continue;
            },
        };
        println!("Your guessed : {} ", guess );
        match guess.cmp(& rand_num){
            Ordering:: Less => println!("Too little!"),
            Ordering :: Greater => println!("Too big!"),
            Ordering:: Equal =>{
                println!("You win!");
                break;
            },
        }

    }
}


// 比较猜到的数和随机数的大小
fn compare(){
    println!("Guess the number.");
    let rand_num = rand::thread_rng().gen_range(1,101);
    println!("The rand number is : {}",rand_num);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed read line.");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number.");
    println!("You guessed : {}",guess);
    match guess.cmp(&rand_num){
        Ordering::Less => println!("Too lettle!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal=> println!("You win!"),
    }
}

// 生成一个随机数
fn rand_number() {
    println!("Guess the number!");
    let rand_num = rand::thread_rng().gen_range(1, 101);
    println!("The rand number is :{}", rand_num);
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line .");
    println!("You guessed : {}", guess);
}


// 从控制台读取输入并输出
fn show_number() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line...");
    println!("Your guessed : {}", guess);
}