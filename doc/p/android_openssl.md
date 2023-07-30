# 编译 Android 运行的 OpenSSL

参考文档:

- <https://github.com/openssl/openssl/blob/master/NOTES-ANDROID.md>
- <https://github.com/openssl/openssl/blob/master/INSTALL.md>

## 主要命令

```sh
export ANDROID_NDK_ROOT=/opt/android-ndk
export PATH=$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

./Configure android-arm64 -D__ANDROID_API__=28 -static -ffunction-sections -fdata-sections -Wl,--gc-sections

make build_generated
make apps/openssl
```

其中 `ANDROID_NDK_ROOT` 是 ndk 安装位置, `-D__ANDROID_API__=28` 表示 Android 9,
`-static` 表示静态链接.

编译后的可执行文件位于 `apps/openssl`.

```sh
/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip openssl
```

最后别忘了用 `strip` 减小文件大小.

## 静态链接对齐错误

```sh
$ ./openssl
error: "./openssl": executable's TLS segment is underaligned: alignment is 8, needs to be at least 64 for ARM64 Bionic
Aborted
```

修复方法:
<https://github.com/termux/termux-packages/issues/8273#issuecomment-1133861593>

添加 `-ffunction-sections -fdata-sections -Wl,--gc-sections`

## 测试

```sh
> adb push openssl /data/local/tmp
> adb shell
$ cd /data/local/tmp
$ ./openssl version
OpenSSL 3.1.1 30 May 2023 (Library: OpenSSL 3.1.1 30 May 2023)
```

TODO
