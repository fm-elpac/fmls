# Termux 中 Avahi 的安装和配置

参考资料:

- <https://termux.dev/en/>
- <https://github.com/termux/termux-packages/issues/8757>

## 1 安装 Avahi

执行命令:

```sh
> pkg install avahi
```

## 2 配置 dbus 和 avahi

2.1 执行命令, 获取当前用户名:

```sh
> whoami
u0_a234
```

2.2 配置文件: `/data/data/com.termux/files/usr/share/dbus-1/system.conf`

将其中:

```
<!-- Run as special user -->
<user>messagebus</user>
```

改为 (使用上一步获得的用户名):

```
<!-- Run as special user -->
<user>u0_a234</user>
```

2.3 配置文件:
`/data/data/com.termux/files/usr/etc/dbus-1/system.d/avahi-dbus.conf`

将其中:

```
<policy user="avahi">
  <allow own="org.freedesktop.Avahi"/>
</policy>
```

改为 (使用上面获得的用户名):

```
<policy user="u0_a234">
  <allow own="org.freedesktop.Avahi"/>
</policy>
```

2.4 配置文件: `/data/data/com.termux/files/usr/etc/avahi/avahi-daemon.conf`

将其中:

```
enable-dbus=no
```

改为:

```
enable-dbus=yes
```

## 3 运行 dbus 和 avahi-daemon

执行命令:

```sh
> mkdir -p /data/data/com.termux/files/usr/var/run/dbus
> rm /data/data/com.termux/files/usr/var/run/dbus/pid
> dbus-daemon --system
```

有报错 `Unknown group "netdev" in message bus configuration file`, 不用管,
忽略即可.

执行命令:

```sh
> avahi-daemon
```

## 4 测试

打开一个新的 termux 会话窗口, 执行命令:

```sh
> avahi-browse -art
```

TODO
