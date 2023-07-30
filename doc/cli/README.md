# fmls-cli

FMLS 命令行界面.

命令行界面设计为高级用户使用, 并不对一般用户进行优化.

## 线程

`fmls-cli` 使用单线程设计.

## 全局命令参数

TODO

## 环境变量

- `FMLSD=` 指定连接到 fmlsd 的本地应用接口的地址

  比如 `FMLSD=/run/user/1000/fmls/fmlsd2.s`

TODO

## 版本和帮助

- `fmls --version`

  查看版本信息.

- `fmls --help`

  查看命令行帮助信息.

## 命令缩写

在同级命令不混淆的情况下, 支持缩写.

比如 `fmls status` (完整命令), 可以缩写为 `fmls st`.

## 一级命令

- `fmls status` 状态查看

- `fmls ca` 证书签发

- `fmls neighbor` 邻居发现

- `fmls info` 节点自定义信息

- `fmls nc` 命名通道

- `fmls tc` 透明通道

## 状态查看命令

```sh
> fmls status
```

- `fmls status`

  当前状态简单总结.

- `fmls status dev`

  查看本机网络接口信息.

- `fmls status neighbor`

  查看邻居状态表.

- `fmls status route`

  查看路由转发表.

- `fmls status local`

  查看本地应用接口状态.

TODO

## 证书签发命令

```sh
> fmls ca
```

- `fmls ca`

  查看证书状态 (信任域).

- `fmls ca list`

  查看可用的 CA 邻居 (服务器).

- `fmls ca request`

  请求签发证书 (上传公钥).

  HTTP 请求: `POST /csr`

- `fmls ca get`

  下载签发的证书.

  HTTP 请求: `GET /crt/PK`

  PK = sha256(K)

  其中 K 是公钥的完整文本表示.

- `fmls ca server`

  启动 CA 服务器.

  上传证书请求文件 (`XXX.csr`) 的限制 (默认值):

  - 单个文件大小: 最大 4KB

    超出限制将返回: `HTTP 413 Payload Too Large`

    如果请求不包含 `Content-Length` 头, 则返回 `HTTP 411 Length Required`.

  - 上传文件的频率: 1 个/秒

    超出限制将返回: `HTTP 429 Too Many Requests`

  由于上传证书的临时通道, 客户端的身份未经过验证, 因此默认所有客户端都是恶意的,
  需要严密防范.

- `fmls ca sign`

  进行签发证书 (自己是 CA).

- `fmls ca rm`

  删除证书.

- `fmls ca enable`

  启用/禁用自己已经加入的信任域的证书.

  `fmls ca en ID 1` 表示启用, `fmls ca en ID 0` 表示禁用.

## 邻居设备发现

```sh
> fmls neighbor
```

- `fmls neighbor`

  查看邻居节点列表.

- `fmls neighbor R`

  查看某个邻居节点的设备信息.

## 节点自定义信息

```sh
> fmls info
```

- `fmls info`

  查看当前实例的自定义信息.

- `fmls info R`

  查看别的设备的自定义信息.

- `fmls info set K V`

  设置一个自定义信息.

- `fmls info rm K`

  移除一个自定义信息.

## 命名通道功能

```sh
> fmls nc
```

- `fmls nc`

  查看当前已经注册的所有命名通道.

- `fmls nc add N`

  注册一个命名通道.

- `fmls nc rm N`

  删除一个命名通道.

- `fmls nc send R N`

  向某个设备 (公钥) 的某个命名通道 (名称) 发送消息.

- `fmls nc listen N`

  从某个命名通道接收消息.

## 透明通道功能

```sh
> fmls tc
```

- `fmls tc`

  查看当前已连接的透明通道.

- `fmls tc new R`

  创建一个新的透明通道 (连接目标设备 R).

- `fmls tc close ID`

  关闭一个透明通道.

- `fmls tc send ID`

  通过透明通道发送数据.

- `fmls tc recv ID`

  通过透明通道接收数据.

---

TODO
