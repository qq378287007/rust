fn main() {
    //不可变变量
    {
        let a = 123;

        //a = "abc"; //类型不一致
        //a = 4.56; //自动数据类型转换，精度损失
        //a = 456; //不可变变量

        println!("The value of a is: {}", a);
    }

    //可变变量
    {
        let mut a = 123;
        println!("The value of a is: {0}", a);

        a = 456;
        println!("The value of a is: {}", a);
    }

    //常量
    {
        const A: i32 = 123;
        //let A = 456;
        println!("The value of a is: {}", A);
    }

    //声明类型
    {
        let a = 123; //自动判断类型
        println!("The value of a is: {0}", a);

        let a: u64 = 123;
        println!("The value of a is: {0}", a);
    }

    //重影（Shadowing）：变量"重新绑定"(类型、可变属性，值)
    {
        let x = 5;
        println!("The value of x is: {0}", x);
        let x = x + 1;
        println!("The value of x is: {0}", x);
        let x = x * 2;
        println!("The value of x is: {0}", x);
        let x = "456";
        println!("The value of x is: {0}", x);

        let mut x = 12;
        println!("The value of x is: {0}", x);
        x = 22;
        println!("The value of x is: {0}", x);
    }

    //数据类型
    //整数，实数，布尔，字符
    //复合类型，元组()，数组[]
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        println!("The value of tup is: {0}, {1}, {2}", tup.0, tup.1, tup.2);

        let (x, y, z) = tup;
        println!("The value of (x, y, z) is: {0}, {1}, {2}", x, y, z);

        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        println!("Value: {0}", first);

        // 数组访问
        //a[0] = 123; // 错误：数组 a 不可变
        let mut a = [1, 2, 3];
        a[0] = 4; // 正确
        println!("Value: {0}", a[0]);

        let b = ["January", "February", "March"];
        println!("Value: {0}", b[0]);

        let c: [i32; 5] = [1, 2, 3, 4, 5];
        println!("Value: {0}", c[0]);

        let d = [3; 5]; //[3, 3, 3, 3, 3];
        println!("Value: {0}", d[0]);
    }

    //注释
    {
        // 这是第一种注释方式
        /* 这是第二种注释方式 */
        /*
         * 多行注释
         * 多行注释
         * 多行注释
         */
    }

    //函数：fn <函数名> ( <参数> ) <函数体>
    {
        another_function1();
        another_function2(5, 6);
    }

    //表达式块
    {
        let x = 5;

        let y = {
            let x = 3;
            x + 1 //表达式，没有分号，否则将变成一条语句
        }; //函数体表达式，不等同于函数体，它不能使用return

        println!("x 的值为 : {}", x);
        println!("y 的值为 : {}", y);
    }

    //函数定义嵌套
    {
        fn five() -> i32 {
            5
        }
        println!("five() 的值为: {}", five());

        fn five2(x: i32) -> i32 {
            5 + x
        }
        println!("five() 的值为: {}", five2(2));

        fn five3(x: i32) -> i32 {
            if x == 2 {
                return 2;
            }
            return 5 + x;
        }
        println!("five() 的值为: {}", five3(3));
    }

    //条件语句
    {
        let a = 12;
        let b;
        if a > 0 {
            b = 1;
        } else if a < 0 {
            b = -1;
        } else {
            b = 0;
        }
        println!("b is {}", b);
    }

    //函数体表达式，类型必须一样
    {
        let a = 3;
        let number = if a > 0 { 1 } else { -1 };
        println!("number 为 {}", number);
    }

    //while循环
    {
        let mut number = 0;
        while number < 4 {
            println!("{}", number);
            number += 1;
        }
        println!("EXIT");
    }

    //for循环
    {
        let a = [10, 20, 30, 40, 50];
        for i in a.iter() {
            println!("值为 : {}", i);
        }

        let a = [10, 20, 30, 40, 50];
        for i in 0..5 {
            println!("a[{}] = {}", i, a[i]);
        }
    }

    //loop 循环
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
    }

    //loop 循环2
    {
        let s = ['R', 'U', 'N', 'O', 'O', 'B'];
        let mut i = 0;
        let location = loop {
            let ch = s[i];
            if ch == 'O' {
                break i;
            }
            i += 1;
        };
        println!(" \'O\' 的索引为 {}", location);
    }

    //所有权规则
    //Rust 中的每个值都有一个变量，称为其所有者。
    //一次只能有一个所有者。
    //当所有者不在程序运行范围时，该值将被删除。

    //变量范围
    {
        // 在声明以前，变量 s 无效
        let s = "runoob";
        // 这里是变量 s 的可用范围
        println!("\"{}\"", s);
    } // 变量范围已经结束，变量 s 无效

    //移动
    //"基本数据"类型的数据，不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制
    //所有整数类型，例如 i32 、 u32 、 i64 等。
    //布尔类型 bool，值为 true 或 false 。
    //所有浮点类型，f32 和 f64。
    //字符类型 char。
    //仅包含以上类型数据的元组（Tuples）。
    {
        let x = 5;
        let y = x;
        println!("\"{}\"", y);
    }

    {
        let s1 = String::from("hello"); // String 对象，长度不确定的数据，需要在堆中存储
        let s2 = s1; //s1 已经失效
                     //println!("{}, world!", s1); // error
        println!("{}, world!", s2);
    }

    //克隆,将数据单纯的复制一份以供他用
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    //函数参数的所有权机制
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

    //函数返回值的所有权机制
    {
        let s1 = gives_ownership();
        // gives_ownership 移动它的返回值到 s1

        let s2 = String::from("hello");
        // s2 被声明有效

        let s3 = takes_and_gives_back(s2);

        println!("s1 = {0}, s3 = {1}", s1, s3); //s2无效
                                                // s2 被当作参数移动, s3 获得返回值所有权
    } // s3 无效被释放, s2 被移动, s1 无效被释放.

    //引用与租借
    {
        let s1 = String::from("hello");
        let s2 = &s1; //& 运算符可以取变量的"引用"。
        println!("s1 is {}, s2 is {}", s1, s2);
    }

    //引用租借所有权
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    //所有权转移
    {
        let s1 = String::from("hello");
        let s2 = &s1;
        println!("{}", s2);

        let s3 = s1;
        println!("{}", s3);
        //println!("{}", s2);//因为 s2 租借的 s1 已经将所有权移动到 s3，所以 s2 将无法继续租借使用 s1 的所有权。
    }

    //重新租借
    {
        let s1 = String::from("hello");
        let mut s2 = &s1;
        println!("{}", s2);

        let s3 = s1;
        s2 = &s3; // 重新从 s3 租借所有权
        println!("{}", s2);
    }

    //不可变引用
    {
        let s1 = String::from("run");
        let s2 = &s1;
        //s2.push_str("oob"); // 错误，禁止修改租借的值
        println!("{}", s2);
    }

    //可变引用
    {
        let mut s1 = String::from("run");
        // s1 是可变的

        let s2 = &mut s1;
        // s2 是可变的引用

        s2.push_str("oob");
        println!("{}", s2);
    }

    //可变引用不允许多重引用，但不可变可以多重引用
    {
        let mut s = String::from("hello");
        let r = &mut s;
        println!("{}", r);

        //error
        //let r1 = &mut s;
        //let r2 = &mut s;
        //println!("{}, {}", r1, r2);
    }
    {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
    }

    //垂悬引用（Dangling References），指针内存已释放
    //{
    //    let reference_to_nothing = dangle();
    //    fn dangle() -> &String {
    //        let s = String::from("hello");
    //        &s
    //    }
    //}

    //切片（Slice），数据值的部分引用。

    //字符串切片（String Slice）
    {
        let s = String::from("broadcast");

        let part1 = &s[0..5];
        let part2 = &s[5..9];

        println!("{}={}+{}", s, part1, part2);
    }

    //被切片引用的字符串禁止更改其值
    {
        let mut s = String::from("runoob");
        let slice = &s[0..3];
        //s.push_str("yes!"); // 错误
        println!("slice = {}", slice);
    }

    //常用的字符串类型
    //str是字符串切片（String Slice），常常以引用的形式出现（&str）。
    //凡是用双引号包括的字符串常量整体的类型性质都是 &str：
    //let s = "hello";

    //String类型的功能更完善——它支持字符串的追加、清空等实用的操作。
    //String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
    //String 和 str 都支持切片，切片的结果是 &str 类型的数据。
    //注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
    //let slice = &s[0..3];

    //String 转换成 &str
    {
        let s1 = String::from("hello");
        let s2 = &s1[..];
        println!("slice = {}", s2);
    }

    //非字符串切片-数组
    {
        let arr = [1, 3, 5, 7, 9];
        let part = &arr[0..3];
        for i in part.iter() {
            println!("{}", i);
        }
    }

    //结构体（Struct）和元组（Tuple）捆绑若干数据类型(不一定相同)
    //结构体的成员（"字段"）和其本身都有名字
    //元组常用于非定义的多值传递，而结构体用于规范常用的数据结构

    //结构体
    {
        //struct语句仅用来定义，不能声明实例，结尾不需要;符号(有也没影响)
        struct Site {
            domain: String,
            name: String,
            nation: String,
            found: u32, //最后一项后面有,也可以
        }

        let runoob = Site {
            domain: String::from("www.runoob.com"),
            name: String::from("RUNOOB"),
            nation: String::from("China"),
            found: 2013,
        };

        //正在实例化的结构体有字段名称和现存变量名称一样的，可以简化书写
        let domain = String::from("www.runoob.com");
        let name = String::from("RUNOOB");
        let runoob2 = Site {
            domain, // 等同于 domain : domain,
            name,   // 等同于 name : name,
            nation: String::from("China"),
            found: 2013,
        };

        //结构体更新语法
        let site = Site {
            domain: String::from("www.runoob.com"),
            name: String::from("RUNOOB"),
            ..runoob
        };
    }

    //元组结构体
    //与元组的区别是它有名字和固定的类型格式。
    //它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
    {
        struct Color(u8, u8, u8);
        struct Point(f64, f64);

        let black = Color(0, 0, 0);
        let origin = Point(0.0, 0.0);

        println!("black = ({}, {}, {})", black.0, black.1, black.2);
        println!("origin = ({}, {})", origin.0, origin.1);
    }

    //结构体所有权
    //结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
    //这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。
    //但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。

    //输出结构体
    {
        #[derive(Debug)] //导入调试库

        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {:?}", rect1);
        println!("rect1 is {:#?}", rect1);
    }

    //结构体方法
    //方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
    //结构体方法的第一个参数必须是 &self，不需声明类型
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            fn wider(&self, rect: &Rectangle) -> bool {
                self.width > rect.width
            }
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("rect1's area is {}", rect1.area());

        let rect2 = Rectangle {
            width: 40,
            height: 20,
        };
        println!("{}", rect1.wider(&rect2));
    }

    //结构体关联函数
    //这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn create(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }
        }

        let rect = Rectangle::create(30, 50);
        println!("{:?}", rect);
    }

    //单元结构体
    //结构体可以只作为一种象征而无需任何成员(无身体)
    //struct UnitStruct;

    //结构体成员赋值时，注意所有权的转移
    {
        struct Dog {
            name: String,
            age: i8,
        }
        let mydog = Dog {
            name: String::from("wangcai"),
            age: 3,
        };
        //let str = mydog.name;//error
        let str = mydog.name.clone();
        println!("str={}", str);
        println!("mydog: name={}, age={}", mydog.name, mydog.age);
    }

    //枚举类
    {
        #[derive(Debug)]

        enum Book {
            Papery,
            Electronic,
        }

        let book = Book::Papery;
        println!("{:?}", book);
    }

    //枚举类成员添加元组属性描述
    {
        #[derive(Debug)]

        enum Book {
            Papery(u32),
            Electronic(String),
        }

        let book = Book::Papery(1001);
        let ebook = Book::Electronic(String::from("url://..."));
        println!("book={:?}, ebook={:?}", book, ebook);
    }

    //为属性命名，不能像访问结构体字段一样访问枚举类绑定的属性
    {
        #[derive(Debug)]
        enum Book {
            Papery { index: u32 },
            Electronic { url: String },
        }
        let book = Book::Papery { index: 1001 };
        println!("book={:?}", book);
    }

    //match 处理枚举类
    {
        enum Book {
            Papery { index: u32 },
            Electronic { url: String },
        }

        let book = Book::Papery { index: 1001 };
        //let ebook = Book::Electronic{url: String::from("url...")};

        match book {
            Book::Papery { index } => {
                println!("Papery book {}", index);
            }
            Book::Electronic { url } => {
                println!("E-book {}", url);
            }
        }
    }

    //match 块也可以当作函数表达式来对待，它可以有返回值（类型必须一致）
    //match 枚举类实例 {
    //    分类1 => 返回值表达式,
    //    分类2 => 返回值表达式,
    //    ...
    //}

    //把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字
    {
        enum Book {
            Papery(u32),
            Electronic { url: String },
        }
        let book = Book::Papery(1001);

        match book {
            Book::Papery(i) => {
                println!("{}", i);
            }
            Book::Electronic { url } => {
                println!("{}", url);
            }
        }
    }

    //match对非枚举类进行分支选择
    {
        let t = "abc";
        match t {
            "abc" => println!("Yes"),
            _ => {}
        }
    }

    //Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
    //enum Option<T> {
    //    Some(T),
    //    None,
    //}
    {
        let opt = Option::Some("Hello");
        match opt {
            Option::Some(something) => {
                println!("{}", something);
            }
            Option::None => {
                println!("opt is nothing");
            }
        }
    }

    //初始值为空的 Option 必须明确类型
    {
        let opt: Option<&str> = Option::None;
        match opt {
            Option::Some(something) => {
                println!("{}", something);
            }
            Option::None => {
                println!("opt is nothing");
            }
        }
    }

    //省略 Option::
    {
        let t = Some(64);
        match t {
            Some(64) => println!("Yes"),
            _ => println!("No"),
        }
    }

    //if let 语法
    {
        let i = 0;
        match i {
            0 => println!("zero"),
            _ => {}
        }

        if let 0 = i {
            println!("zero");
        }
    }

    {
        enum Book {
            Papery(u32),
            Electronic(String),
        }
        let book = Book::Electronic(String::from("url"));
        if let Book::Papery(index) = book {
            println!("Papery {}", index);
        } else {
            println!("Not papery book");
        }
    }

    //组织管理
    //箱
    //二进制程序文件（不一定是可执行文件）或者库文件，存在于"包"中。
    //树状结构，根是源文件所编译的程序。

    //包
    //工程就是包，由Cargo.toml管理，描述了包的基本信息以及依赖项。
    //一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
    //默认src/main.rs文件为二进制箱的根，编译之后的二进制箱将与包名相同。

    //模块
    //组织单位
    {
        //mod nation {
        //    mod government {
        //        fn govern() {}
        //    }
        //    mod congress {
        //        fn legislate() {}
        //    }
        //    mod court {
        //        fn judicial() {}
        //    }
        //}

        //nation
        // ├── government
        // │     └── govern
        // ├── congress
        // │     └── legislate
        // └── court
        //       └── judicial

        //// 绝对路径从 crate 关键字开始描述。
        //crate::nation::government::govern();

        ////相对路径从 self 或 super 关键字或一个标识符开始描述。例如：
        //nation::government::govern();
    }

    //访问权限
    //默认private，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。
    //公共权限，需要使用 pub 关键字。

    //mod nation {
    //    pub mod government {
    //        pub fn govern() {}
    //    }

    //    mod congress {
    //        pub fn legislate() {}
    //    }

    //    mod court {
    //        fn judicial() {
    //            super::congress::legislate();
    //        }
    //    }
    //}

    //fn main() {
    //    nation::government::govern();
    //}

    //结构体除了其本身是私有的以外，其字段也默认是私有的
    //mod back_of_house {
    //    pub struct Breakfast {
    //        pub toast: String,
    //        seasonal_fruit: String,
    //    }

    //    impl Breakfast {
    //        pub fn summer(toast: &str) -> Breakfast {
    //            Breakfast {
    //                toast: String::from(toast),
    //                seasonal_fruit: String::from("peaches"),
    //            }
    //        }
    //    }
    //}
    //pub fn eat_at_restaurant() {
    //    let mut meal = back_of_house::Breakfast::summer("Rye");
    //    meal.toast = String::from("Wheat");
    //    println!("I'd like {} toast please", meal.toast);
    //}
    //fn main() {
    //    eat_at_restaurant()
    //}

    //枚举类枚举项可以内含字段，但不具备访问权限的性质:
    //mod SomeModule {
    //    pub enum Person {
    //        King {
    //            name: String
    //        },
    //        Quene
    //    }
    //}

    //fn main() {
    //    let person = SomeModule::Person::King{
    //        name: String::from("Blue")
    //    };
    //    match person {
    //        SomeModule::Person::King {name} => {
    //            println!("{}", name);
    //        }
    //        _ => {}
    //    }
    //}

    //模块引用
    {
        // main.rs
        //mod second_module;
        //
        //fn main() {
        //    println!("This is the main module.");
        //    println!("{}", second_module::message());
        //}

        //// second_module.rs
        //pub fn message() -> String {
        //    String::from("This is the 2nd module.")
        //}
    }

    //use 关键字，将模块标识符引入当前作用域
    //{
    //    mod nation {
    //        pub mod government {
    //            pub fn govern() {}
    //        }
    //    }

    //    use crate::nation::government::govern;

    //    fn main() {
    //        govern();
    //    }
    //}

    //as 关键字，为标识符添加别名
    //{
    //    mod nation {
    //        pub mod government {
    //            pub fn govern() {}
    //        }
    //        pub fn govern() {}
    //    }

    //    use crate::nation::government::govern;
    //    use crate::nation::govern as nation_govern;

    //    fn main() {
    //        nation_govern();
    //        govern();
    //    }
    //}

    //use 关键字可以与 pub 关键字配合使用：
    //{
    //mod nation {
    //    pub mod government {
    //        pub fn govern() {}
    //    }
    //    pub use government::govern;
    //}
    //
    //fn main() {
    //    nation::govern();
    //}
    //}

    //引用标准库
    //{
    //    use std::f64::consts::PI;
    //    fn main() {
    //        println!("{}", (PI / 2.0).sin());
    //    }
    //}

    //错误处理
    //可恢复错误，比如访问文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。Result<T, E> 类来处理
    //enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //}

    //{
    //    use std::fs::File;
    //    fn main() {
    //        let f = File::open("hello.txt");

    //        match f {
    //            Ok(file) => {
    //                println!("File opened successfully.");
    //            }
    //            Err(err) => {
    //                println!("Failed to open the file.");
    //            }
    //        }

    //          if let Ok(file) = f {
    //              println!("File opened successfully.");
    //          } else {
    //              println!("Failed to open the file.");
    //          }

    //    }
    //}

    //可恢复错误按不可恢复错误处理，使用unwrap() 和 expect(message: &str)
    //{
    //    use std::fs::File;

    //    fn main() {
    //        //Result 为 Err 时调用 panic! 宏。两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息。
    //        let f1 = File::open("hello.txt").unwrap();
    //        let f2 = File::open("hello.txt").expect("Failed to open.");
    //    }
    //}

    //可恢复错误的传递
    //{
    //    fn f(i: i32) -> Result<i32, bool> {
    //        if i >= 0 {
    //            Ok(i)
    //        } else {
    //            Err(false)
    //        }
    //    }
    //    fn g(i: i32) -> Result<i32, bool> {
    //        let t = f(i);
    //        return match t {
    //            Ok(i) => Ok(i),
    //            Err(b) => Err(b),
    //        };
    //    }
    //
    //    fn g(i: i32) -> Result<i32, bool> {
    //        //? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
    //        //所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
    //        let t = f(i)?; //? 操作符将同类的 Err 直接传递出去
    //        Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
    //    }
    //
    //    fn main() {
    //        let r = f(10000);
    //        if let Ok(v) = r {
    //            println!("Ok: f(-1) = {}", v);
    //        } else {
    //            println!("Err");
    //        }
    //    }
    //
    //    let r = g(10000);
    //    if let Ok(v) = r {
    //        println!("Ok: g(10000) = {}", v);
    //    } else {
    //        println!("Err");
    //    }
    //}

    //kind 方法，获取 Err 类型
    //{
    //    use std::fs::File;
    //    use std::io;
    //    use std::io::Read;
    //
    //    fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    //        let mut f = File::open(path)?;
    //        let mut s = String::new();
    //        f.read_to_string(&mut s)?;
    //        Ok(s)
    //    }
    //
    //    fn main() {
    //        let str_file = read_text_from_file("hello.txt");
    //        match str_file {
    //            Ok(s) => println!("{}", s),
    //            Err(e) => match e.kind() {
    //                io::ErrorKind::NotFound => {
    //                    println!("No such file");
    //                }
    //                _ => {
    //                    println!("Cannot read the file");
    //                }
    //            },
    //        }
    //    }
    //}

    //不可恢复错误，无法解决的逻辑错误，例如访问数组越界。panic! 宏来处理
    //导致程序受到致命的打击而终止运行
    {
        //panic!("error occured");
        println!("Hello, Rust");
    }

    //函数中定义泛型
    //{
    //    fn max(array: &[i32]) -> i32 {
    //        let mut max_index = 0;
    //        let mut i = 1;
    //        while i < array.len() {
    //            if array[i] > array[max_index] {
    //                max_index = i;
    //            }
    //            i += 1;
    //        }
    //        array[max_index]
    //    }
    //
    //    fn max<T>(array: &[T]) -> T {
    //        let mut max_index = 0;
    //        let mut i = 1;
    //        while i < array.len() {
    //            if array[i] > array[max_index] {
    //                max_index = i;
    //            }
    //            i += 1;
    //        }
    //        array[max_index]
    //    }
    //
    //    fn main() {
    //        let a = [2, 4, 6, 3, 1];
    //        println!("max = {}", max(&a));
    //    }
    //}

    //结构体与枚举类中的泛型
    //{
    //    struct Point<T> {
    //        x: T,
    //        y: T,
    //    }
    //    //let p1 = Point { x: 1, y: 2 };//自动类型推导
    //    let p2 = Point { x: 1.0, y: 2.0 };
    //    //let p = Point {x: 1, y: 2.0};//error，类型不一致
    //
    //    struct Point<T1, T2> {
    //        //允许类型不一致
    //        x: T1,
    //        y: T2,
    //    }
    //
    //    enum Option<T> {
    //        Some(T),
    //        None,
    //    }
    //
    //    enum Result<T, E> {
    //        Ok(T),
    //        Err(E),
    //    }
    //}

    //泛型的类的方法实现
    //{
    //    struct Point<T> {
    //        x: T,
    //        y: T,
    //    }
    //
    //    impl<T> Point<T> {//impl 关键字的后方必须有 <T>
    //        fn x(&self) -> &T {
    //            &self.x
    //        }
    //    }
    //
    //    impl Point<f64> {//为其中的一种泛型添加方法
    //        fn x(&self) -> f64 {
    //            self.x
    //        }
    //    }
    //
    //    impl<T, U> Point<T, U> {//内部方法具有泛型的能力
    //        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    //            Point {
    //                x: self.x,
    //                y: other.y,
    //            }
    //        }
    //    }
    //
    //    fn main() {
    //        let p = Point { x: 1, y: 2 };
    //        println!("p.x = {}", p.x());
    //    }
    //}

    //特性，接口，行为规范
    //{
    //    trait Descriptive {
    //        fn describe(&self) -> String;
    //    }
    //
    //    struct Person {
    //        name: String,
    //        age: u8
    //    }
    //
    //    //同一个类可以实现多个特性，每个 impl 块只能实现一个。
    //    impl Descriptive for Person {
    //        fn describe(&self) -> String {
    //            format!("{} {}", self.name, self.age)
    //        }
    //    }
    //}

    //特性可以定义默认方法
    //{
    //    trait Descriptive {
    //        fn describe(&self) -> String {
    //            String::from("[Object]")
    //        }
    //    }
    //
    //    struct Person {
    //        name: String,
    //        age: u8,
    //    }
    //
    //    impl Descriptive for Person {//块中内容去掉，调用特性默认方法
    //        fn describe(&self) -> String {
    //            format!("{} {}", self.name, self.age)
    //        }
    //    }
    //
    //    fn main() {
    //        let cali = Person {
    //            name: String::from("Cali"),
    //            age: 24,
    //        };
    //        println!("{}", cali.describe());
    //    }
    //}

    //特性做参数
    //{
    //    fn output(object: impl Descriptive) {
    //        //实现Descriptive特性的对象就行，此函数内也无法使用其他的属性与方法
    //        println!("{}", object.describe());
    //    }
    //
    //    fn output<T: Descriptive>(object: T) {
    //        //等效语法
    //        println!("{}", object.describe());
    //    }
    //
    //    fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    //        //多个特性参数
    //        println!("{}", arg1.describe());
    //        println!("{}", arg2.describe());
    //    }
    //
    //    //涉及多个特性
    //    //+仅用于表示类型的时候，不可以在 impl 块中使用。
    //    fn notify(item: impl Summary + Display) {}
    //    fn notify<T: Summary + Display>(item: T) {}
    //
    //    //where简化
    //    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
    //    fn some_function<T, U>(t: T, u: U) -> i32
    //    where
    //        T: Display + Clone,
    //        U: Clone + Debug,
    //    {
    //    }
    //}

    //取最大值
    //{
    //    trait Comparable {
    //        fn compare(&self, object: &Self) -> i8;// Self （注意大小写）关键字就代表了当前类型（不是实例）本身。
    //    }
    //
    //    fn max<T: Comparable>(array: &[T]) -> &T {
    //        let mut max_index = 0;
    //        let mut i = 1;
    //        while i < array.len() {
    //            if array[i].compare(&array[max_index]) > 0 {
    //                max_index = i;
    //            }
    //            i += 1;
    //        }
    //        &array[max_index]
    //    }
    //
    //    impl Comparable for f64 {
    //        fn compare(&self, object: &f64) -> i8 {
    //            if &self > &object {
    //                1
    //            } else if &self == &object {
    //                0
    //            } else {
    //                -1
    //            }
    //        }
    //    }
    //
    //    fn main() {
    //        let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    //        println!("maximum of arr is {}", max(&arr));
    //    }
    //}

    //特性做返回值
    //{
    //    fn person() -> impl Descriptive {
    //        Person {
    //            name: String::from("Cali"),
    //            age: 24
    //        }
    //    }
    //
    //    //error，同一个函数中所有可能的返回值类型必须完全一样。
    //    //即使结构体 A 与结构体 B 都实现了特性 Trait
    //    fn some_function(bool bl) -> impl Descriptive {
    //        if bl {
    //            return A {};
    //        } else {
    //            return B {};
    //        }
    //    }
    //}

    //有条件实现方法
    //{
    //    struct A<T> {}
    //
    //    //A<T> 类型必须在 T 已经实现 B 和 C 特性的前提下才能有效实现此 impl 块。
    //    impl<T: B + C> A<T> {
    //        fn d(&self) {}
    //    }
    //}

    //生命周期
    //与所有权同等重要的资源管理机制。
    //应对复杂类型系统中资源管理的问题。
    //{
    //    let r;
    //    {
    //        let x = 5;
    //        r = &x;
    //    }
    //    //r引用的x已经释放
    //    println!("r: {}", r);
    //}

    //{
    //    //error,返回值引用可能会返回过期的引用：
    //    fn longer(s1: &str, s2: &str) -> &str {
    //        if s2.len() > s1.len() {
    //            s2
    //        } else {
    //            s1
    //        }
    //    }
    //
    //    fn main() {
    //        let r;
    //        {
    //            let s1 = "rust";
    //            let s2 = "ecmascript";
    //            r = longer(s1, s2);
    //        }
    //        println!("{} is longer", r);
    //}

    //生命周期注释,描述引用生命周期的办法。
    //不能够改变引用的生命周期，但可以在合适的地方声明两个引用的生命周期一致。
    //单引号开头，跟着一个小写字母单词
    {
        //&i32        // 常规引用
        //&'a i32     // 含有生命周期注释的引用
        //&'a mut i32 // 可变型含有生命周期注释的引用
    }

    //{
    //    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    //        if s2.len() > s1.len() {
    //            s2
    //        } else {
    //            s1
    //        }
    //    }
    //
    //    fn main() {
    //        let r;
    //        {
    //            let s1 = "rust";
    //            let s2 = "ecmascript";
    //            r = longer(s1, s2);//函数返回值的生命周期将与两个参数的生命周期一致
    //            println!("{} is longer", r);
    //        }
    
    //rust"、"ecmascript"都是字符串字面常量(string literals)，生命周期持续到整个程序运行期间。
    //s1、s2都是借用形式&str，r没有拷贝整个字符串，只是“拷贝”了对字符串字面常量的引用；
    //        println!("{} is longer", r);//success,s1，s2 是被 copy 给了 r。

    //      {
    //          let s1 = String::from("rust");
    //          let s2 = String::from("ecmascript");
    //          r = longer(s1, s2);
    //          println!("{} is longer", r);//success
    //      }
    //      println!("{} is longer", r);//error, s1，s2 没有 copy trait
    //    }
    //}

    //结构体中使用字符串切片引用
    //{
    //    fn main() {
    //        struct Str<'a> {
    //            content: &'a str,
    //        }
    //        let s = Str {
    //            content: "string_slice",
    //        };
    //        println!("s.content = {}", s.content);
    //    }
    //    impl<'a> Str<'a> {
    //        fn get_content(&self) -> &str { //&'a str
    //        }
    //    }
    //}

    //静态生命周期
    //所有用双引号包括的字符串常量所代表的精确数据类型都是 &'static str
    //'static 所表示的生命周期从程序运行开始到程序运行结束。

    //泛型、特性与生命周期协同作战
    //{
    //    use std::fmt::Display;
    //
    //    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    //    where
    //        T: Display,
    //    {
    //        println!("Announcement! {}", ann);
    //        if x.len() > y.len() {
    //            x
    //        } else {
    //            y
    //        }
    //    }
    //}

    //命令行参数
    {
        let args = std::env::args();
        println!("{:?}", args);
        for arg in args {
            println!("{}", arg);
        }
    }

    //命令行输入
    {
        use std::io::stdin;

        let mut str_buf = String::new();

        stdin()
            .read_line(&mut str_buf)
            .expect("Failed to read line.");

        println!("Your input line is \n{}", str_buf);
    }

    //文件读取
    {
        use std::fs;
        let text = fs::read_to_string("main.rs").unwrap();
        println!("{}", text);

        //二进制
        //let content = fs::read("text.dat").unwrap();
        //println!("{:?}", content);

        //文件流读取
        /*
        use std::io::prelude::*;
        let mut buffer = [0u8; 5];
        let mut file = fs::File::open("text.dat").unwrap();
        file.read(&mut buffer).unwrap();
        println!("{:?}", buffer);
        file.read(&mut buffer).unwrap();
        println!("{:?}", buffer);
        */
    }

    //文件写入
    {
        //不存在则创建，存在则覆盖
        use std::fs;
        fs::write("out.txt", "FROM RUST PROGRAM").unwrap();

        //创建
        /*
            use std::fs::File;
            use std::io::prelude::*;
            let mut file = File::create("D:\\text.txt").unwrap();
            file.write(b"FROM RUST PROGRAM").unwrap();
        */

        //追加
        /*
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        fn main() -> std::io::Result<()> {
            let mut file = OpenOptions::new().append(true).open("D:\\text.txt")?;
            //let mut file = OpenOptions::new().read(true).write(true).open("D:\\text.txt")?;//读写权限
            file.write(b" APPEND WORD")?;
            Ok(())
        }
        */
    }

    //向量
    {
        //let vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的空向量
        let mut vector = vec![1, 2, 4, 8]; // 通过数组创建向量
        vector.push(16);
        vector.push(32);
        vector.push(64);
        println!("{:?}", vector);

        let mut v1: Vec<i32> = vec![1, 2, 4, 8];
        let mut v2: Vec<i32> = vec![16, 32, 64];
        v1.append(&mut v2);
        println!("{:?}", v1);

        let mut v = vec![1, 2, 4, 8];
        println!(
            "{}",
            match v.get(0) {
                Some(value) => value.to_string(),
                None => "None".to_string(),
            }
        );

        println!("{}", v[1]);

        for i in &v {
            println!("{}", i);
        }

        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }

    //字符串
    {
        let string = String::new();

        let one = 1.to_string(); // 整数到字符串
        let float = 1.3.to_string(); // 浮点数到字符串
        let slice = "slice".to_string(); // 字符串切片到字符串

        let hello1 = String::from("Hello");
        let hello2 = String::from("你好");

        let mut s = String::from("run");
        s.push_str("oob"); // 追加字符串切片
        s.push('!'); // 追加字符

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = String::from("toe");
        //let s4 = s1 + &s2;
        //let s5 = s1 + "-" + &s2 + "-" + &s3;
        let s6 = format!("{}-{}-{}", s1, s2, s3);

        let len = s.len();

        let len2 = hello2.chars().count();

        for c in hello2.chars() {
            println!("{}", c);
        }

        let a = hello2.chars().nth(2);
        println!("{:?}", a);

        let sub = &s[0..2];
        println!("{}", sub);
    }

    //Map
    {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        map.insert("color", "red");
        map.insert("size", "10 m^2");

        println!("{}", map.get("color").unwrap());

        for p in map.iter() {
            println!("{:?}", p);
        }

        //不存在时插入，否则跳过
        map.entry("color").or_insert("red");

        //存在时，修改值
        if let Some(x) = map.get_mut("color") {
            *x = "blue";
        }
    }

    //可以使用结构体或枚举类来实现类的功能：
    {
        /*
        pub struct ClassName {
            pub field: Type,
        }

        pub impl ClassName {
            fn some_method(&self) {
                // 方法函数体
            }
        }

        pub enum EnumName {
            A,
            B,
        }

        pub impl EnumName {
            fn some_method(&self) {

            }
        }
        */
    }
    //完整的类
    {
        /*
        second.rs
        pub struct ClassName {
            field: i32,
        }

        impl ClassName {
            pub fn new(value: i32) -> ClassName {
                ClassName {
                    field: value
                }
            }

            pub fn public_method(&self) {
                println!("from public method");
                self.private_method();
            }

            fn private_method(&self) {
                println!("from private method");
            }
        }
        main.rs
        mod second;
        use second::ClassName;

        fn main() {
            let object = ClassName::new(1024);
            object.public_method();
        }
        */
    }

    //std::thread::spawn 函数（无参）创建新线程
    {
        use std::thread;
        use std::time::Duration;

        fn spawn_function() {
            for i in 0..5 {
                println!("spawned thread print {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }

        thread::spawn(spawn_function);

        for i in 0..3 {
            println!("main thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    //使用闭包（closures）来传递函数作为参数
    {
        use std::thread;
        use std::time::Duration;

        thread::spawn(|| {
            for i in 0..5 {
                println!("spawned thread print {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..3 {
            println!("main thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    //闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。闭包相当于 Rust 中的 Lambda 表达式
    //|参数1, 参数2, ...| -> 返回值类型 {
    //    // 函数体
    //}
    {
        //let inc = |num: i32| -> i32 { num + 1 };
        //let inc = |num|  { num + 1 };//自动类型推断
        let inc = |num: i32| -> i32 { num + 1 };
        println!("inc(5) = {}", inc(5));
    }

    //join方法
    {
        use std::thread;
        use std::time::Duration;

        let handle = thread::spawn(|| {
            for i in 0..5 {
                println!("spawned thread print {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..3 {
            println!("main thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    //move 强制所有权迁移
    {
        use std::thread;
        let s = "hello";

        //let handle = thread::spawn(|| { //子线程中尝试使用当前函数的资源，error
        let handle = thread::spawn(move || {
            println!("{}", s);
        });

        handle.join().unwrap();
    }

    //消息传递，Rust 中一个实现消息传递并发的主要工具是通道（channel），通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
    {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); //子线程获得了主线程的发送者 tx，并调用了它的 send 方法发送了一个字符串
        });

        let received = rx.recv().unwrap(); //主线程就通过对应的接收者 rx 接收到了
        println!("Got: {}", received);
    }
}

fn another_function1() {
    println!("Hello, runoob!");
}

fn another_function2(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

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
//引用只能租借（Borrow）值的所有权
//引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：
fn calculate_length(s: &String) -> usize {
    s.len()
}
