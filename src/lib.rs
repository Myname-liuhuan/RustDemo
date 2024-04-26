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

    //文本填充打印
    #[test]
    fn print_test() {
        let _text = "《我是填充文本》";
        println!("打印填充的文本：{}", _text);
    }

    //数组
    fn arrays(){
        let arr =  [1,6,8,3434,39,0,1];
        for i in arr {
           
        }
    }

    //数组填充打印
    #[test]
    fn print_arrays(){
        let arr = [1,6,8,3434,39,0,1];
        let arr_str =  arr.iter().map(f);
        //println!("打印数组：", String::from_utf8(arr) );
    }
}