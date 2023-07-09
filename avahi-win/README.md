# avahi-win

与 avahi 大致兼容的命令行工具, 但是使用 win32 API (`DnsServiceBrowse`,
`DnsServiceRegister`).

注意: 仅支持 Windows 10+, 因为旧版 Windows 没有所需的 API.

## 用法

```bat
> powershell.exe -File avahi-win.ps1 ...
```

TODO

## 说明

旧版 Windows 没有自带 mDNS/DNS-SD 功能, 需要手动安装第三方软件来使用
mDNS/DNS-SD.

Windows 10 及更新版本, 系统终于添加了相关 win32 API (`DnsServiceBrowse`,
`DnsServiceRegister`).

使用这些系统 API 有多种方式, 比如编写 C/C++ 程序, 比如编写 rust 程序, 比如编写
C# 程序等.

比较了一圈之后, 觉得不需要编译的 PowerShell 可能是最简单的方式. 同时对性能
(运行速度) 没有要求, PowerShell 也能满足需求.

在 GNU/Linux 平台通过 Avahi 软件包来使用 mDNS/DNS-SD 功能.
那么将此工具的命令行参数基本按照 Avahi 的格式, 可以简化跨平台实现.

## PowerShell 执行策略设置

默认情况下, PowerShell 可能禁止运行脚本, 如下:

```
PS C:\Windows\system32> Get-ExecutionPolicy
Restricted
```

更改执行策略来允许运行 (此命令需要以管理员身份运行):

```
> Set-ExecutionPolicy RemoteSigned
```

参考文档:
<https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.3>

## 相关链接

- Avahi: <https://avahi.org/>

- win32 API:
  - `DnsServiceBrowse`
    <https://learn.microsoft.com/en-us/windows/win32/api/windns/nf-windns-dnsservicebrowse>
  - `DnsServiceRegister`
    <https://learn.microsoft.com/en-us/windows/win32/api/windns/nf-windns-dnsserviceregister>

- 参考代码:
  <https://github.com/svbnet/NativeDnssd/blob/master/Svbnet.NativeDnssd/Interop/Win32/NativeMethods.cs>

- 编程语言: PowerShell (C#) <https://learn.microsoft.com/en-us/powershell/>

- 在 PowerShell 中调用 win32 API:
  <https://devblogs.microsoft.com/scripting/use-powershell-to-interact-with-the-windows-api-part-1/>

TODO
