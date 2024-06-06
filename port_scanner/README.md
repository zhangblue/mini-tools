# 端口扫描器

## 功能介绍

用于对某一个ip地址的服务器进行端口扫描

## 源码编译

```shell
cargo build -r
```

## 运行方式

```shell
Usage: port_scanner [OPTIONS] --addr <ADDR>

Options:
      --addr <ADDR>              扫描地址
      --port-start <PORT_START>  扫描的开始端口 [default: 1]
      --port-end <PORT_END>      扫描的结束端口 [default: 1024]
  -h, --help                     Print help
  -V, --version                  Print version
```



## 未完善功能

目前执行一次只能对一个ip地址进行扫描，后续会修改为可以输入一个ip地址范围
