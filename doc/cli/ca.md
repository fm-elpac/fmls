# fmls ca

证书签发命令.

## 栗子: 一个设备加入信任域

假设设备 A 是 CA, 设备 B 要加入信任域.

1. A 设备启动 CA 服务器:

   ```sh
   A> fmls ca server
   ```

2. B 设备查看可用 CA:

   ```sh
   B> fmls ca li
   ```

3. B 设备请求签发证书:

   ```sh
   B> fmls ca req
   ```

4. A 设备查看证书:

   ```sh
   A> fmls ca st
   ```

5. A 设备进行签发证书:

   ```sh
   A> fmls ca sign Y0bZLhLgL8YNQncGsIrdAsQ7o9XRLAL4LmxVHL3lhW8
   ```

   指定需要签发的证书的公钥.

6. B 设备下载签发的证书:

   ```sh
   B> fmls ca get
   ```

---

TODO
