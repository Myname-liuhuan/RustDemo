use std::fs;
use std::path;

//实现读取文件夹下面所有文件并替换其中内容的功能
fn main(){
    read_directory();
}

//读取文件内容
fn read_file(){
    let file_path = "D:/tmp/document/test.txt";

    let contents = fs::read_to_string(file_path)
        .expect("没有读取到文件");

    println!("file contents is {}", contents);
}

//打印出文件夹下所有文件的文件名称
fn read_directory(){
    //给定文件夹
    let dir_path = "D:/tmp/document/";

    //创建path对象
    let path = path::Path::new(dir_path);

    //读取目录中的所有文件
    let paths = fs::read_dir(path).unwrap();

    //打印所有文件名称
    for entity in paths {
        let entity = entity.unwrap();
        println!("{:?}", entity.file_name());
    }


   
}