#[cfg(test)]
mod tests {
    use std::string;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    //helloword
    #[test]
    fn hello() {
        println!("hello, word!");
    }
//------------------------string------------------------
    #[test]
    fn print_test() {
        let _text = "《我是填充文本》";
        println!("打印填充的文本：{}", _text);

        // 通常情况下，`{}` 会被任意变量内容所替换。
        // 变量内容会转化成字符串。
        // 整数类型转化的时候, 默认的是i32,但是也可以通过在数字后面的添加类型标注来指定
        println!("{} days", 31);//i32
        println!("{} days", 31i64);//i64

        //使用位置参数进行数据填充
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
        //使用命名参数进行数据填充
        println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

        // 在{}内使用参数可以将数据格式化为不同的样式
        // :b：转化为二进制
        // :o：转化为八进制。
        // :x 或 :X：转化为十六进制（小写或大写）。
        // :e 或 :E：转化为科学计数法的格式（小写或大写）。
        // :p：打印指针地址。
        // :?：打印调试格式。
        println!("{} of {:b} people know binary, the other half don't", 1, 3);

        //批量填充（对齐）
        // : 格式化字符串中，: (冒号) 用于指示要应用的格式说明符的开始
        // number 表示实际数据
        // width 表示总长度
        // > 表示右对齐即填充在左侧
        // < 表示左对齐即填充在右侧
        // $ 用于声明这里的width是一个待填充的变量
        // 模板写法是 {数字:默认填充的字符 左对齐/右对齐 宽度}，但是在默认填充字符为0时可以把对齐符号提到冒号后面
        //右对齐，空格填充
        println!("{number:>width$}", number=1, width=6);
        //字符填充
        println!("{number:0>width$}", number=1, width=6);
        //左对齐,字符填充
        println!("{number:9<width$}", number=2, width=6);

        //print会进行参数检查
        //println!("My name is {0}, {1} {0}", "Bond");

    }



    /**
     * string &str,两者的区别主要是所有权的区别，&str是只读的，而string是可变的。
     * str :字符串常量池
     * &str :字符串切片,对常量池中部分数据的引用
     * String :可变字符串
     * &String :字符串切片,对String中部分数据的引用
     */
    #[test]
    fn string_test() {
        //&str往往用来表示固定的文本，就算声明变量的时候定义为mut,也不支持string那样的push()等方法
        let a_str = "我是str";
        println!("打印str：{}", a_str);

        // String的本质是字符数组 Vec<u8>
        let mut b_string: String = String::from("我是string");
        b_string.push('c');
        println!("打印string：{}", b_string);
        b_string.push_str("abc");


    }

    // //数组
    // fn arrays(){
    //     let arr =  [1,6,8,3434,39,0,1];
    //     for i in arr {
           
    //     }
    // }

    // //数组填充打印
    // #[test]
    // fn print_arrays(){
    //     let arr = [1,6,8,3434,39,0,1];
    //     let arr_str =  arr.iter().map(f);
    //     println!("打印数组：", String::from_utf8(arr) );
    // }

    //重影（Shadowing）:后声明的变量可以使用前面使用过的变量名称，将会覆盖前面的变量
    //正式版定义：重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化
    #[test]
    fn shadowing(){
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);
    }


    //进制写法
    //声明一个变量的时候可以使用常见的进制
    #[test]
    fn hex_test(){
        let hex = 0xff; //16进制
        let bin = 0b1111_0000; //二进制
        let oct = 0o77; //8进制
        let dec = 1_255; //10进制
        let hex_str = "ff";//字符串
        println!("hex:{}", hex);
        println!("bin:{}", bin);
        println!("oct:{}", oct);
        println!("dec:{}", dec);
        println!("hex_str:{}", hex_str);
    }

    //元组
    #[test]
    fn tuple_test(){
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("x:{}", x);
        println!("y:{}", y);
        println!("z:{}", z);
    }

    //条件语句
    #[test]
    fn condition_test(){
        let a = 1;
        if a == 1 {
            println!("a is 1");
        } else if a == 2 {
            println!("a is 2");
        } else {
            println!("a is neither 1 nor 2");
        }

        //实现类似三元运算符的功能
        let num = 4;
        let boo = if num < 5 { true } else { false };
        println!("result: {}", boo);
    }

    //循环
    #[test]
    fn loop_test(){
        //loop循环
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("result: {}", result);

        //while循环
        let mut number = 3;
        while number!= 0 {
            println!("{}!", number);
            number -= 1;
        }
    }

    //迭代器
    #[test]
    fn iterator_test(){
        //借用迭代器
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("v1: {}", val);
        }

        //可变借用迭代器
        let mut v2 = vec![1, 2, 3];
        let v2_iter_mut = v2.iter_mut();
        for val in v2_iter_mut {
            *val *= 2;
        }
        println!("v2的值 {:?}", v2);

        //所有权迭代器
        let v3 = vec![1, 2, 3];
        let v3_iter_into = v3.into_iter();
        for val in v3_iter_into {
            println!("v3: {}", val);
        }
        // println!("v3的值 {:?}", v3); //报错，v3的所有权已经被转移

        //迭代器的Map方法
        let v4 = vec![1, 2, 3];
        let v4_iter_map = v4.iter().map(|x| x + 1);
        println!("v4_iter_map: {:?}", v4_iter_map);
        let v4_iter_map_collect:Vec<i32> = v4.iter().map(|x| x * x).collect();
        println!("v4_iter_map_collect: {:?}", v4_iter_map_collect);

        //迭代器的Filter方法
        let v5 = vec![1, 2, 3, 4, 5, 6];
        let v5_iter_filter = v5.iter().filter(|x| *x % 2 == 0);
        println!("v5_iter_filter: {:?}", v5_iter_filter);
        let v5_iter_filter_collect:Vec<i32> = v5.into_iter().filter(|x| *x % 2 == 0).collect();
        println!("v5_iter_filter_collect: {:?}", v5_iter_filter_collect);

        //for循环遍历迭代器
        let v6 = vec![1, 2, 3, 4, 5, 6];
        for &num in v6.iter(){
            println!("v6_iter_for: {}", num);
        }

        //消费迭代器 iter.next()
        let v7 = vec![1, 2, 3, 4, 5, 6];
        let mut v7_iter = v7.iter();
        while let Some(num) = v7_iter.next() {
            println!("v7_iter_next: {}", num);
        }
        

    }

    /**
     * 闭包
     * 一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
     */
    #[test]
    fn closure_test(){
        //读取作用域参数相加
        let x = 5;
        let sum_x = |y| x + y;
        println!("sum_x: {}", sum_x(2));

        //读取多个参数
        let a  = 5;
        let b = 6;
        let sum_ab = |a, b| a + b;
        println!("sum_ab: {}", sum_ab(3, 4));
    }

    /**
     * 所有权（核心） 
     * 所有权规则
     * 1. Rust中的每一个值都有一个被称为其所有者（owner）的变量。
     * 2. 值在任一时刻有且只有一个所有者。
     * 3. 当所有者（变量）离开作用域，这个值将被丢弃。
     * 
     * 所有权是指变量在代码中能使用的范围，在超过范围后就会被回收内存变为无效变量
     * 所有权的设计让rust成为了当前最安全的语言，因为它会强制要求开发者注意内存问题
     * 
     */
    #[test]
    fn ownership_test(){
        //所有权转移
        let s1 = "hello";
        let s2 = s1;
        // println!("s1: {}", s1); //报错，s1的所有权已经被转移
        println!("s2: {}", s2);

        //应用类型的值传递也会造成所有权的传递，即在调用函数后，该参数在当前主代码块中失效
        let s3 = String::from("hello");
        takes_ownership(s3);
        // println!("引用类型所有权测试 s3: {}", s3); //报错，s3的所有权已经被转移

        //基本数据类型不一样，在作为参数传递给函数后，当前主代码块中还可以使用
        let s4 = 9;
        makes_copy(s4);
        println!("基本类型所有权测试 s4: {}", s4);


        //函数返回所有权
        let s5 = String::from("kpop");
        let s6 = gives_ownership(s5);
        println!("函数返回所有权 s6: {}", s6);
        // println!("函数返回所有权 s5: {}", s5); //报错，s5的所有权已经被转移

    
        //引用与租借
        //在使用引用传参调用函数的时候不会改变参数别的所有权
        let s7 = String::from("hello");
        let len = calculate_length(&s7);
        println!("字符 {} 的长度是{}", s7, len);

    }

    fn takes_ownership(some_string: String) {
        println!("takes_ownership： {}", some_string);
    }
    fn makes_copy(some_integer: i32) {
        println!("makes_copy： {}", some_integer);
    }
    fn gives_ownership(some_string: String) -> String {
        println!("gives_ownership： {}", some_string);
        return some_string;
    }
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    
    /**
     * 引用
     * 引用允许你使用值但不获取其所有权，即允许你引用一个值而不用获取其所有权
     */
    #[test]
    fn reference_test(){
        //源值如果失效，那么所有引用它的引用都会失效
        let s1 = String::from("hello");
        let s2 = &s1;
        let s3 = s1;
        //println!("s1: {}", s1); //报错，s1的所有权已经被转移
        //println!("s2: {}", s2); //报错，源值已失效
        println!("s3: {}", s3);


        //如果源值是mut的，那么引用也是mut；如果源值不可变，那么引用的值也不可变
        let s4 = String::from("hello");
        let s5 = &s4;
        //s4.push_str("world");//报错，因为s4不可变
        //s5.push_str("world");//报错，因为s4不可变,所以引用它的s5也是不可变的
        let mut s6 = String::from("hello");
        let s7 = &mut s6;
        s7.push_str("world"); //可以调用，因为源可变
        //println!("s6: {}", s6); //在创建对可变mut变量的引用后，源值不再可用，是为了防止数据竞争确保内存安全
        println!("s7: {}", s7);


        
    }


    /**
     * 切片
     * 切片（Slice）是对数据值的部分引用
     * 
     * 语法 ..
     * ..y 等价于 0..y
     * x.. 等价于位置 x 到数据结束
     * .. 等价于位置 0 到结束
    */
    #[test]
    fn slice_test(){
        let s1 = String::from("hello world");
        let hello = &s1[0..5];
        let world = &s1[6..11];
        println!("{} = {} + {}", s1, hello, world);

        //&str其实就是String的（引用/切片）
        let s2 = String::from("hello world");
        let hello = &s2[..];
        println!("(源){} = (切片){}", s2, hello);

        //因为String的本质就是数组，所以可以推断出 数组也是支持切片的
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[0..3];
        println!("{:?}", slice);

    }


    /**
     * 结构体
    */
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

}