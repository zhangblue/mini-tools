# 照片归类工具

## 功能简介

用于将照片按照拍摄时间进行归类。

## 当前支持的文件类型

应 `kamadak-exif` crate 的限制。当前只支持 `["TIFF", "RAW", "HEIF", "JPEG", "WEBP", "PNG", "JPG"]` 格式的文件类型



## 源码编译方式

```shell
cargo build -r
```

## 运行方式

```shell
Usage: photo_classification --src <SRC> --target <TARGET>

Options:
  -s, --src <SRC>        需要检查的目录
  -t, --target <TARGET>  归类后的目录
  -h, --help             Print help
  -V, --version          Print version
```



