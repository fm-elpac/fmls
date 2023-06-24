# fmls-cli

FMLS 命令行界面.

命令行界面设计为高级用户使用, 并不对一般用户进行优化.


## 全局命令参数

TODO


## 环境变量设置

TODO


## 版本和帮助

+ `fmls --version`

  查看版本信息.

+ `fmls --help`

  查看命令行帮助信息.


## 命令缩写

在同级命令不混淆的情况下, 支持缩写.

比如 `fmls status` (完整命令),
可以缩写为 `fmls st`.


## 状态查看命令

```sh
> fmls status
```

TODO


## 证书签发命令

```sh
> fmls ca
```

+ `fmls ca status`

  查看证书状态 (信任域).

+ `fmls ca list`

  查看可用的 CA 邻居 (服务器).

+ `fmls ca request`

  请求签发证书 (上传公钥).

+ `fmls ca get`

  下载签发的证书.

+ `fmls ca server`

  启动 CA 服务器.

+ `fmls ca sign`

  进行签发证书 (自己是 CA).

+ `fmls ca rm`

  删除证书.


----

TODO
