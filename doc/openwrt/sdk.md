# OpenWrt SDK

<https://openwrt.org/docs/guide-developer/toolchain/using_the_sdk>

SDK 可用于编译单个软件包.

各个 CPU 架构 / 目标平台的 SDK 需要分别单独下载, 比如:

- <https://downloads.openwrt.org/releases/22.03.5/targets/x86/64/>

  目标 (target): `x86/64`

  软件包架构: [`x86_64`](https://openwrt.org/docs/techref/instructionset/x86_64)

  SDK:
  <https://downloads.openwrt.org/releases/22.03.5/targets/x86/64/openwrt-sdk-22.03.5-x86-64_gcc-11.2.0_musl.Linux-x86_64.tar.xz>

  适用设备:

  - x86_64 虚拟机

- <https://downloads.openwrt.org/releases/22.03.5/targets/mediatek/mt7622/>

  目标 (target): `mediatek/mt7622`

  软件包架构:
  [`aarch64_cortex-a53`](https://openwrt.org/docs/techref/instructionset/aarch64_cortex-a53)

  SDK:
  <https://downloads.openwrt.org/releases/22.03.5/targets/mediatek/mt7622/openwrt-sdk-22.03.5-mediatek-mt7622_gcc-11.2.0_musl.Linux-x86_64.tar.xz>

  适用设备:

  - 路由器 [红米 ax6s](https://openwrt.org/toh/xiaomi/ax3200)

    128MB SPI NAND flash, 256MB RAM

- <https://downloads.openwrt.org/releases/22.03.5/targets/ramips/mt7621/>

  目标 (target): `ramips/mt7621`

  软件包架构:
  [`mipsel_24kc`](https://openwrt.org/docs/techref/instructionset/mipsel_24kc)

  SDK:
  <https://downloads.openwrt.org/releases/22.03.5/targets/ramips/mt7621/openwrt-sdk-22.03.5-ramips-mt7621_gcc-11.2.0_musl.Linux-x86_64.tar.xz>

  适用设备:

  - 路由器
    [小米 cr6606](https://openwrt.org/toh/hwdata/xiaomi/xiaomi_mi_router_cr6606)

    128MB NAND flash, 256MB RAM

- <https://downloads.openwrt.org/releases/22.03.5/targets/ramips/mt76x8/>

  目标 (target): `ramips/mt7628`

  软件包架构:
  [`mipsel_24kc`](https://openwrt.org/docs/techref/instructionset/mipsel_24kc)

  SDK:
  <https://downloads.openwrt.org/releases/22.03.5/targets/ramips/mt76x8/openwrt-sdk-22.03.5-ramips-mt76x8_gcc-11.2.0_musl.Linux-x86_64.tar.xz>

  适用设备:

  - 路由器
    [小米 wifi nano (小米路由器青春版) R1CL](https://openwrt.org/toh/xiaomi/miwifi_nano)

    16MB SPI NAND flash, 64MB RAM

## 编译 rust 程序

此处以 `mipsel_24kc` (ramips/mt76x8) 举例.

1. 使用 cargo 创建 rust 项目.

   ```sh
   > cargo new h1
   > cd h1
   ```

2. 配置 SDK 环境变量.

   ```sh
   > export OPENWRT_SDK=/home/s2/openwrt-sdk-22.03.5-ramips-mt76x8_gcc-11.2.0_musl.Linux-x86_64
   > export CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER=$OPENWRT_SDK/staging_dir/toolchain-mipsel_24kc_gcc-11.2.0_musl/bin/mipsel-openwrt-linux-gcc
   ```

3. 编译

   ```sh
   > cargo build --target mipsel-unknown-linux-musl --release
      Compiling h1 v0.1.0 (/home/s2/h1)
       Finished release [optimized] target(s) in 0.29s
   > $OPENWRT_SDK/staging_dir/toolchain-mipsel_24kc_gcc-11.2.0_musl/bin/mipsel-openwrt-linux-strip target/mipsel-unknown-linux-musl/release/h1
   ```

   别忘了用 `strip` 减小目标文件大小.

4. 测试 (运行)

   ```
   root@OpenWrt:~# cd /tmp
   root@OpenWrt:/tmp# wget http://192.168.5.148:3000/h1
   Downloading 'http://192.168.5.148:3000/h1'
   Connecting to 192.168.5.148:3000
   Writing to 'h1'
   h1                   100% |*******************************|   382k  0:00:00 ETA
   Download completed (391880 bytes)
   root@OpenWrt:/tmp# chmod +x h1
   root@OpenWrt:/tmp# ls -l h1
   -rwxr-xr-x    1 root     root        391880 Jun 30 15:58 h1
   root@OpenWrt:/tmp# ./h1
   Hello, world!
   root@OpenWrt:/tmp#
   ```

TODO
