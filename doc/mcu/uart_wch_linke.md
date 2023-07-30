# 关于 UART WCH-LINKE

操作系统 (host): ArchLinux (`x86_64-unknown-linux-gnu`)

WCH-LINKE 自带一个 UART 串口 (TX, RX 针脚). 由于相关资料 (linux) 很少,
在此处对其使用进行说明.

## 1 系统配置

权限配置:

```sh
> sudo groupadd uart  # 创建用户组 `uart` (用于串口权限)
> sudo gpasswd -a s2 uart  # 将用户 `s2` (自己) 加入用户组
```

udev 规则配置:

```
> cat /etc/udev/rules.d/50-wch.rules
SUBSYSTEM=="usb", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="8010", GROUP="uart"
SUBSYSTEM=="tty", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="8010", GROUP="uart"
```

## 2 验证配置

(重新插上 WCH-LINKE 设备)

```
> ls -l /dev/ttyACM0
crw-rw---- 1 root uart 166, 0 Jul 27 06:49 /dev/ttyACM0
```

(重新登录, 使用户组生效)

查看串口当前配置参数:

```
> stty -a -F /dev/ttyACM0
speed 9600 baud; rows 0; columns 0; line = 0;
intr = ^C; quit = ^\; erase = ^?; kill = ^U; eof = ^D; eol = <undef>;
eol2 = <undef>; swtch = <undef>; start = ^Q; stop = ^S; susp = ^Z; rprnt = ^R;
werase = ^W; lnext = ^V; discard = ^O; min = 1; time = 0;
-parenb -parodd -cmspar cs8 hupcl -cstopb cread clocal -crtscts
-ignbrk -brkint -ignpar -parmrk -inpck -istrip -inlcr -igncr icrnl ixon -ixoff
-iuclc -ixany -imaxbel -iutf8
opost -olcuc -ocrnl onlcr -onocr -onlret -ofill -ofdel nl0 cr0 tab0 bs0 vt0 ff0
isig icanon iexten echo echoe echok -echonl -noflsh -xcase -tostop -echoprt
echoctl echoke -flusho -extproc
```

串口回显测试:

用一根杜邦线两端连接 WCH-LINKE 的 TX 和 RX 针脚.

```
> sudo pacman -S uucp
```

测试:

```
> cu -l /dev/ttyACM0
```

退出输入: `~.`

TODO
