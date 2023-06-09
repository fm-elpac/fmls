# fmls_r2c3p 常见问题

目录:

- r2c3p 和 MQTT 协议的区别 ?

## r2c3p 和 MQTT 协议的区别 ?

参考资料: <https://mqtt.org/>

1. MQTT 协议使用发布/订阅模型, 中心服务器 (broker) 同时连接很多 (百万个)
   终端设备 (client).

   r2c3p 是点对点通信的消息发送协议, 只有收发消息的功能, 没有发布/订阅等功能.
   r2c3p 仅用于 r1 (网关设备) 与其直连的 r2 (低资源设备) 之间.

2. MQTT 协议基于 TCP/IP, 可用于广域网传输, 可选 TLS 加密传输, 复杂度相对较高.

   r2c3p 基于 UART (串口) / USB 传输, 仅用于短距离局域网 (r1 和 r2 直连),
   明文传输不加密, 复杂度很低 (功能更简单).

   所以 r2c3p 可用于资源更少的设备 (比如单片机 ch32v003),
   这些设备甚至没有足够的硬件资源去实现 TCP/IP, 也就不能支持 MQTT.

3. r2c3p 仅提供相当于 MQTT QoS 0 (最多一次, r2c3p 静默消息), MQTT QoS 1
   (最少一次, r2c3p 请求响应消息) 的功能, 不提供 MQTT QoS 2 (有且仅有一次)
   的功能.

---

TODO
