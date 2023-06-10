fn main() {
    {
        let a = 12;
        println!("a is {}", a);

        println!("a is {}, a again is {}", a, a);
        println!("a is {0}, a again is {0}", a);

        println!("{{}}");
        println!("\"\'\\");

        let a = 123; //变量的值可以"重新绑定"
                     //a = "abc";//a是整数
                     //a = 4.56; //精度损失
                     //a = 456;//不可变变量

        let mut a = 123; //可变变量，变量的值可以"重新绑定"（重影）
                         //a = 456;

        const b: i32 = 123; //常量，值不可以"重新绑定"
                            //let b = 456;

        let a: u64 = 123; //声明类型

        //变量"重新绑定"（重影）
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);

        let mut s = "123";
        //s = s.len();//不能给字符串变量赋整型值

        //整数，实数，布尔，字符
        //复合，元组()，数组[]

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // tup.0 等于 500
        // tup.1 等于 6.4
        // tup.2 等于 1
        let (x, y, z) = tup;
        // y 等于 6.4

        let aa = [1, 2, 3, 4, 5];
        // aa 是一个长度为 5 的整型数组

        let bb = ["January", "February", "March"];
        // bb 是一个长度为 3 的字符串数组

        let c: [i32; 5] = [1, 2, 3, 4, 5];
        // c 是一个长度为 5 的 i32 数组

        let d = [3; 5];
        // 等同于 let d = [3, 3, 3, 3, 3];

        let first = aa[0];
        let second = aa[1];
        // 数组访问

        //aa[0] = 123; // 错误：数组 a 不可变
        let mut aa = [1, 2, 3];
        aa[0] = 4; // 正确
    }

    // 这是第一种注释方式
    /* 这是第二种注释方式 */
    /*
     * 多行注释
     * 多行注释
     * 多行注释
     */

    println!("Hello, world!");
    another_function();
    another_function2(5, 6);

    let x = 5;

    let y = {
        //表达式块
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    fn five() -> i32 {
        //函数体表达式,不能使用 return
        5
    }
    println!("five() 的值为: {}", five());

    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

    {
        let a = 12;
        let b10;
        if a > 0 {
            b10 = 1;
        } else if a < 0 {
            b10 = -1;
        } else {
            b10 = 0;
        }
        println!("b is {}", b10);
    }
    {
        let a = 3;
        let number = if a > 0 { 1 } else { -1 }; //类型要一致
        println!("number 为 {}", number);
    }

    {
        let mut number = 1;
        while number != 4 {
            println!("{}", number);
            number += 1;
        }
    }

    {
        let a = [10, 20, 30, 40, 50];
        for i in a.iter() {
            println!("值为 : {}", i);
        }
        for i in 0..5 {
            println!("a[{}] = {}", i, a[i]);
        }
    }

    {
        let s = ['R', 'U', 'N', 'O', 'O', 'B'];
        let mut i = 0;
        loop {
            let ch = s[i];
            if ch == 'O' {
                break;
            }
            println!("\'{}\'", ch);
            i += 1;
        }

        let location = loop {
            let ch = s[i];
            if ch == 'O' {
                break i;
            }
            i += 1;
        };
        println!(" \'O\' 的索引为 {}", location);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        //println!("{}, world!", s1); // 错误！s1 已经失效
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); //克隆
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello");
        // s 被声明有效

        takes_ownership(s);
        // s 的值被当作参数传入函数
        // 所以可以当作 s 已经被移动，从这里开始已经无效

        let x = 5;
        // x 被声明有效

        makes_copy(x);
        // x 的值被当作参数传入函数
        // 但 x 是基本类型，依然有效
        // 在这里依然可以使用 x 却不能使用 s
    } // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放

    {
        let s1 = gives_ownership();
        // gives_ownership 移动它的返回值到 s1

        let s2 = String::from("hello");
        // s2 被声明有效

        let s3 = takes_and_gives_back(s2);
        // s2 被当作参数移动, s3 获得返回值所有权
    } // s3 无效被释放, s2 被移动, s1 无效被释放.

    {
        let s1 = String::from("hello");
        let s2 = &s1; //引用
        println!("s1 is {}, s2 is {}", s1, s2);
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let s1 = String::from("hello");
        let s2 = &s1;
        let s3 = s1;
        //println!("{}", s2);//错误， s2租借的s1的所有权已经转移到s3
    }

    {
        let s1 = String::from("hello");
        let mut s2 = &s1;
        let s3 = s1;
        s2 = &s3; // 重新从 s3 租借所有权
        println!("{}", s2);
    }
    {
        let s1 = String::from("run");
        let s2 = &s1;
        println!("{}", s2);
        //s2.push_str("oob"); // 错误，禁止修改租借的值，租借的所有权不能修改所有者的值
        println!("{}", s2);
    }

    {
        let mut s1 = String::from("run");
        // s1 是可变的

        let s2 = &mut s1;
        // s2 是可变的引用

        s2.push_str("oob");
        println!("{}", s2);
    }

    //可变引用不允许多重引用，但不可变引用可以
    {
        let mut s = String::from("hello");
        //let r1 = &mut s;
        //let r2 = &mut s;
        //println!("{}, {}", r1, r2);
    }

    {
        let a = [2, 4, 6, 3, 1];
        println!("max = {}", max(&a));
    }



}

fn another_function() {
    println!("Hello, runoob!");
}

fn another_function2(x: i32, y: i32) -> i32 {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
    return x + y;
}

//如果将变量当作参数传入函数，那么它和移动的效果是一样的。
fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

//被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放。
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string // a_string 被当作返回值移出函数
}

//引用不会获得值的所有权。
//引用只能租借（Borrow）值的所有权。
//引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：
fn calculate_length(s: &String) -> usize {
    s.len()
}

//禁止垂悬引用
/* 
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

//函数泛型
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}
/* 
fn max<T>(array: &[T]) -> T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}*/