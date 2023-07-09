# fmls_r2c3p protocol 0.1.0

Simple message send protocol like single line text, clear text transfer without
encryption.

- `r2`: for low resource device (MCU)

- `c3`: use crc32 as message checksum

- `p`: point to point communication, send through UART (serial port) or USB.

Design reference:

- MIN protocol <https://github.com/min-protocol/min>

fmls_r2c3p protocol version: `0.1.0`

- Semantic Versioning 2.0.0 <https://semver.org/>

## Table of contents

- 1 Message format
  - 1.1 Message byte escape and transfer
  - 1.2 Process when receive message
- 2 Message type
  - 2.1 Process of request response message
  - 2.2 Pre-defined message type
    - 2.2.1 v
    - 2.2.2 V
    - 2.2.3 E
    - 2.2.4 K
    - 2.2.5 c
    - 2.2.6 C
  - 2.3 Pre-defined error code
    - 2.3.1 E-1
    - 2.3.2 E-2
    - 2.3.3 E-3
    - 2.3.4 E-4
    - 2.3.5 E-5
  - 2.4 Pre-defined config
    - 2.4.1 m
    - 2.4.2 T
    - 2.4.3 t
- 3 Multi-channel mode
- 4 Implementation requirement

## 1 Message format

- 1 Byte: **Message type** (type)

- N Byte: **Extra data** (data)

  Can be 0 Byte.

- 4 Byte (UART): crc32 **checksum**, calculate of `type+data`.

  No need of CRC checksum for USB.

  If the length of the message (type+data) is no more than 32 Byte, then use
  crc16 checksum (2 Byte).

Limit of message length: One message MUST finish send within 900ms.

### 1.1 Message byte escape and transfer

Only for UART, no need of escape for USB.

The message before escape: type + data + crc

The Byte need to escape: `\n` (0x0a), `\\` (0x5c)

- when `\n` Byte, then send `\\n` (0x5c 0x6e)

- when `\\` Byte, then send `\\s` (0x5c 0x73)

After send the message, send one `\n` (0x0a) Byte to end the message.

### 1.2 Process when receive message

If escape error (escape can not recognize) or CRC checksum error, just drop the
message.

## 2 Message type

- **Request message**: MUST send a response message when receive a request
  message.

  Lowercase letter (`a` ~ `z`), byte value 0x61 to 0x7a.

- **Response message**: for reply the request message.

  Uppercase letter (`A` ~ `Z`), byte value 0x41 to 0x5a.

- **Silent message**: no need to send a response message when receive.

  Other byte value.

### 2.1 Process of request response message

When send request message, MUST be one ask and one answer, can not send new
request message before receive a response message.

After send request message, if not receive a response message within 1 second,
then send fail, auto retry send. If still fail after send 3 times, not retry,
report error to up layer application.

When use request response message, the up layer application SHOULD notice, if
receive the same request message more than once, the application still works ok.

### 2.2 Pre-defined message type

- 2.2.1 `v` 0x76 (request message) Get version information of device firmware

  This message SHOULD NOT check the CRC checksum, and SHOULD ignore the extra
  data (if any).

  After receive this message, SHOULD send `V` message.

- 2.2.2 `V` 0x56 (response message) Return version information of device
  firmware

  Return content include:
  - fmls_r2c3p protocol version
  - firmware name, firmware version
  - device hardware information, device unique sequence number (if support)

  join with `\n` (0x0a) byte.

  This message MUST use crc32 checksum, no matter the length of the message.

  Example: (request+response, no CRC)

  ```
  v
  Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 66665555
  ```

- 2.2.3 `E` 0x45 (response message) Return error

  Extra data format:
  - error code (decimal number text)
  - Space 0x20 (need if use error information)
  - error information (optional)

  Example `E-2 32` means error code `-2`, error information `32`.

- 2.2.4 `K` 0x4b (response message) Return success (ok)

  Extra data is optional.

- 2.2.5 `c` 0x63 (request message) For device config

  format:

  - `cK` get current config value (config item is K)

  - `cK=V` set config value (config item is K, value is V)

    K and V can be multi byte.

  After receive this message, if no error SHOULD send `C` message. If error,
  SHOULD send `E` message.

- 2.2.6 `C` 0x43 (response message) Return device config

  format: `CK=V`

  Example: (# for comment)

  ```
  cm    # get current value of config item m
  Cm=0  # the value of m is 0
  cm=1  # set the value of m to 1
  Cm=1  # set ok, the current value of m is 1
  ```

### 2.3 Pre-defined error code

Error code range:

- `> 0` application define

- `<= 0` pre-defined

For low end MCU, the data type of error code can be `i8`. For more powerful
device, the data type of error code can be `i16` or `i32` as need.

- 2.3.1 `E-1` reserved (unknown error)

- 2.3.2 `E-2` Message too long

  Error information (optional) can return the max length of a message can be
  receive.

  Example `E-2 32` means the max length of a message is 32 Byte.

- 2.3.3 `E-3` Unknown message type

  Error information (optional) can return which message type not support.

  Example `E-3 c` means message type `c` is not support.

- 2.3.4 `E-4` Error message format

  The extra data of the message can not be parse. Error information is optional.

- 2.3.5 `E-5` Error message argument

  The format of extra data is ok, but the content can not be accept. Error
  information is optional.

### 2.4 Pre-defined config

- 2.4.1 `m` for multi-channel mode, default value `0`

  - value `0` means this function is disabled

  - value `1` means this function is enabled

- 2.4.2 `T` for get the device system time

  Format is hexadecimal number text, no limit of length. The device specific
  time counter.

  Example:

  ```
  cT
  CT=018905baffa7
  ```

- 2.4.3 `t` for get the device short time (16 bit)

  Format is the same as `T`. Can be used to monitor the online status of the
  device.

  Example: (# for comment)

  ```
  ct       # UART transfer, the total length of this message is 5 Byte (crc16)
  Ct=a8f8  # UART transfer, the total length of this message is 10 Byte (crc16)
  ```

## 3 Multi-channel mode

This function is optional.

TODO

## 4 Implementation requirement

- MCU SHOULD support receive message with length no more than 8 Byte (exclude
  CRC).

- The implementation of fmls_r2c3p protocol MUST support message type of `v`,
  `V`, `E`, `K`.

TODO
