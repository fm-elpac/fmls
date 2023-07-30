# FMLS 常见问题

目录:

- FMLS 和 Matter 协议的对比 ?

- FMLS 加密传输数据的原则 ?

## FMLS 和 Matter 协议的对比 ?

参考资料: <https://csa-iot.org/all-solutions/matter/>

1. Matter 标准由许多大公司共同制定 (CSA), 而 FMLS 只是一个普通的开源项目.

   Matter 协议的使用, 需要成为 CSA 的会员, 具有一定的费用及门槛. Matter
   设备需要经过中心机构 (CSA) 的认证.

   FMLS 以 LGPLv3 许可发布, 没有使用费用. FMLS 是去中心化的,
   公钥/私钥生成以及证书签发, 都在本地进行.

2. Matter 协议专注于连接物联网 (IoT) 设备, 而 FMLS 包括连接非物联网设备,
   比如台式机, 笔记本, 手机, 平板, 服务器等.

   在功能以及协议复杂度方面, FMLS 要简单很多.

   FMLS 更注重低成本. 比如 fmls_r2c3p 可以支持仅 0.8 元的 ch32v003 单片机.

3. Matter 主要使用 C/C++ 编程语言, 而 FMLS 使用 rust 编程语言.

4. Matter 更偏好使用无线网络 (wifi, Thread, BLE), 而 FMLS 更偏好使用有线网络
   (以太网, USB, UART).

5. Matter 将设备加入网络 (commissioning) 需要通过 BLE 进行
   (扫描二维码或输入数字码), 而 FMLS 的将设备加入网络 (信任域) 通过签发证书
   (用户确认操作) 进行.

6. Matter 对发送的每条消息单独加密, 而 FMLS 首先在设备之间建立安全连接 HTTPS
   (HTTP/3 QUIC), 然后发送数据.

7. FMLS 能适应更复杂的网络环境. FMLS 的设备分为 r0 和 r1 设备, r1
   设备可用于连接不能直接通信的 IP 网络.

   FMLS 优先使用 IPv6, 但同时也支持 IPv4. Matter 仅使用 IPv6.

8. FMLS 和 Matter 的相同点:

   - 都基于 IP 网络, 适用于局域网

   - 都具有 "将一个设备加入网络" 的操作

   - 都使用 mDNS/DNS-SD 进行设备发现

   - 都使用证书来验证通信双方的身份, 确保加密传输数据的安全

## FMLS 加密传输数据的原则 ?

1. 所有经过 IP 网络传输的数据, 都需要加密.

2. 所有经过无线网络传输的数据, 都需要加密.

3. 只有非 IP 的有线网络 (比如 USB, UART), 才可以明文传输.

---

TODO
