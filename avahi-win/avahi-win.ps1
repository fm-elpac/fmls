# avahi-win.ps1
#
# 注意 !!! .ps1 文件必须以换行 CRLF 保存, 否则会有奇怪的 BUG
#
# 用法:
# TODO

# 获取命令行参数
$a = New-Object System.Collections.ArrayList
$a.AddRange($args)

echo "= $($a -join ' ')"

# C# 代码
$代码 = @'
using System;
using System.Collections;

public class Aw {
  // 入口
  public static void main(ArrayList a) {
    // TODO
    Console.WriteLine("ok " + a.Count);
  }
}
'@

# 使用 Add-Type 编译 C# 代码
$m = Add-Type -PassThru -TypeDefinition $代码

# 调用 C# 代码入口
$m::main($a)
