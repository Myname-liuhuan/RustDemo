#[cfg(test)]
mod tests {
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
        let a_str: &str = "我是str";
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
}