# 关于 OpenWrt

[OpenWrt](https://openwrt.org/) 和普通的 GNU/Linux 差别很大,
需要对这个系统单独进行适配.

## 重要参考资料

- 概述 <https://openwrt.org/docs/guide-developer/overview>

- 文件系统 <https://openwrt.org/docs/techref/filesystems>

- flash 布局 <https://openwrt.org/docs/techref/flash.layout>

- 文件系统层次 <https://openwrt.org/docs/techref/file_system>

- 软件包管理 (opkg)
  <https://openwrt.org/docs/guide-user/additional-software/opkg>

- 镜像构建器
  <https://openwrt.org/docs/guide-user/additional-software/imagebuilder>

## OpenWrt SDK

<https://openwrt.org/docs/guide-developer/toolchain/using_the_sdk>

(详见 [sdk.md](./sdk.md))

## procd

<https://openwrt.org/docs/techref/procd>

代替 `systemd`, 用于管理系统服务的启动和运行.

TODO

## ubus

<https://openwrt.org/docs/techref/ubus>

代替 `dbus`, 用于系统各组件间的通信.

TODO

## uci

<https://openwrt.org/docs/techref/uci>

统一配置界面, 用于系统及各组件的配置.

TODO

## umdns

<https://openwrt.org/docs/guide-developer/mdns>
<https://openwrt.org/docs/guide-user/network/zeroconfig/zeroconf>

代替 `Avahi`, 实现 mDNS/DNS-SD 功能.

TODO
