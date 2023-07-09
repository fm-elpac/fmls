# 关于 Avahi

相关链接:

- <https://avahi.org/>
- <https://github.com/lathiat/avahi>

Avahi 是 GNU/Linux 平台的 mDNS/DNS-SD 实现. 比如 ArchLinux 默认安装了 avahi.

FMLS 在 GNU/Linux 平台使用 avahi 进行 mDNS/DNS-SD 邻居发现.

## Avahi 的安装和配置 (ArchLinux)

参考资料: <https://wiki.archlinux.org/title/Avahi>

1 安装软件包:

```sh
> sudo pacman -S avahi nss-mdns
```

2 禁用 `systemd-resolved` 服务, 因为这个会和 avahi 功能冲突:

```sh
> sudo systemctl stop systemd-resolved
> sudo systemctl disable systemd-resolved
> sudo systemctl mask systemd-resolved
```

3 启用 `avahi-daemon` 服务:

```sh
> sudo systemctl enable avahi-daemon.socket
> sudo systemctl start avahi-daemon
```

4 配置文件: `/etc/nsswitch.conf`

把其中的:

```
hosts: mymachines resolve [!UNAVAIL=return] files myhostname dns
```

改为:

```
hosts: mymachines mdns_minimal [NOTFOUND=return] resolve [!UNAVAIL=return] files myhostname dns
```

5 测试:

发布一个服务, 比如:

```sh
> avahi-publish-service a666 _666_test._udp 666 r=r0 PK=test HOSTNAME=$(hostname)
Established under name 'a666'
```

> 注: 用 `Ctrl+C` 结束 `avahi-publish-service` 的运行, 即可停止发布.

查看 mDNS/DNS-SD 服务:

```sh
> avahi-browse -art
```

TODO
