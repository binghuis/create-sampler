# filepick

实现对一堆文件随机抽样的 CLI 工具。

![rust](https://img.shields.io/badge/Rust-000000?logo=rust&labelColor=263238)

## install

`cargo install filepick`

## uasge

`filepick`

使用 rust 实现一个方法

1. 输入目录，递归遍历这个目录下的所有文件
2. 如果文件是压缩格式那么解压缩遍历
3. 根据文件后缀名小写方式进行分类
4. 打印每种文件的文件个数

然后根据上面已分类文件

1. 输入一个抽样数字
2. 对已分类文件进行按比例抽样
3. 将抽样文件写入一个新的目录
4. 打印抽样结果
