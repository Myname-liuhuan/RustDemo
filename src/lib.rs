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

    //文本填充打印
    #[test]
    fn print_test() {
        let _text = "《我是填充文本》";
        println!("打印填充的文本：{}", _text);
    }

    //类型转换
    #[test]
    fn type_convert() {
        //整数转浮点
        let num = 124;
        let fnum = num as f64;
        println!("整数转浮点:{}", fnum);

        //浮点转整数
        let fnum2 = 124.54321;
        let num2 = fnum2 as i32;
        println!("浮点转整数:{}", num2);

        //整数转字符串
        let snum = num.to_string();
        println!("整数转字符串：{}", snum);

        //字符串转整数
        let snum2 = "123".to_string();
    }

    //数组
    #[test]
    fn arrays(){
        let arr =  [1,6,8,3434,39,0,1];
        for i in arr {
           println!("{}", i);
        }
    }

    //数组填充打印
    #[test]
    fn print_arrays(){
        let arr = [1,6,8,3434,39,0,1];
        //let arr_str =  arr.iter().map(f);
        //println!("打印数组：", String::from_utf8(arr) );
    }
}