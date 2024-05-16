use inquire::Text;

fn main() {
    let name = Text::new("请输入要抽样的文件目录").prompt();

    match name {
        Ok(name) => println!("Hello {}", name),
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }
}

// 读文件
// 文件分类
// 打印文件分类情况
// 输入抽样个数
// 文件抽样
