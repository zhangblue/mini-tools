# 用于将csv文件转成一个json文件的小工具

```shell
Usage: rcli <COMMAND>

Commands:
  csv   将CSV文件转换成其他格式的文件
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```shell
将CSV文件转换成其他格式的文件

Usage: rcli csv [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          输入文件路径
  -o, --output <OUTPUT>        输出文件路径 [default: output.json]
  -d, --delimiter <DELIMITER>  分隔符 [default: ,]
      --header                 CSV文件是否有头
  -h, --help                   Print help
```