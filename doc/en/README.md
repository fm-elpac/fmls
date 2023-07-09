# en

This is the English translate of FMLS document. Only for reference.

## project README

FMLS

formal name: "Toxorhynchites edwardsi" series, "Pholcus phalangioides" software.

---

## Overview

FMLS: based on "the self-organization principle of the system", used for small
device network. Device discovery, connect, resource share.

Based on IP network, decentralization (not depend on public IP, or DNS), mainly
used for local area network.

### The problem and target

The target is easy connect of multi-type device, multi operating system, multi
CPU type. The device type include desktop PC, laptop PC, handset (smart phone),
tablet PC, rack server, wifi router, TV box, single board computer (SBC), VR
device, MCU, etc. The operating system include GNU/Linux, Android, Windows,
OpenWrt, fuchsia, etc. The CPU include x86_64, aarch64, rv64gc, rv32imc, rv32ec,
etc.

IP network (and wire Ethernet, wireless wifi) can connect multi device, but it
has problem:

1. IP address (and port number) is usually dynamic, the address of one device
   can change, input IP address (and port) by hand is not easy (and IPv6). DNS
   is centralized, so not good for small local area network.

2. TCP/IP do not care security by default. It do not verify the identity of
   device, and data transport security.

3. IP network is stateless, it do not care target device is online or not, it do
   not care route is reachable or not. Only after wait a long time, timeout,
   many retry failed, and know it not work.

4. No easy method for application to send message cross platform.

The improvement of FMLS is:

1. Use the public key (and private key) as the identity of a device. So however
   its IP address (and port) change, it can be identify and connect.

   When first time run, FMLS auto generate public key / private key on the
   device local (with OpenSSL, GPG). Certificate is also signed locally, so FMLS
   is decentralized.

   With device auto discovery technology (mDNS/DNS-SD), it is easy to find out
   the IP address of target device and connect.

2. FMLS use public key (private key) to verify the identity of a device, use
   certificate to build the trust pool of device, and secure protocol (HTTPS,
   SSH) to transport data. So there is better security.

3. FMLS track the online status of each device. When route is need, FMLS
   calculate the route is reachable or not first.

4. FMLS run as system service at most platform, and provide the same interface
   for connect and send message. The application based on FMLS can easy connect
   and send message to other device cross platform.

### Requirement of network performance

FMLS require the following performance of the local area network:

- packet lose rate is under 10% (the success probability of one transmit exceed
  90%)

- the end to end latency (RTT) is under 1000ms

The protocol and implement of FMLS will optimise on these condition.

---

Detail document please see `doc/` directory.

## The high level design goal

1. **Simple**: As simple as possible. Because FMLS is cross platform, the simple
   design is easy to implement. Use simple technology, reduce the difficult of
   development, increase develop speed.

   (Such as JSON API, sqlite database, command line interface)

2. **Automation**: To do more auto work, reduce the work has to operate by hand.

   (Such as mDNS/DNS-SD)

3. **Security**: Use security programming language, security network protocol,
   security cryptography algorithm, etc.

   (Such as rust, HTTPS (HTTP/3 QUIC), SSH, EC, sha256, AES-GCM)

4. **Extend interface**: Provide simple API for other program, for easy
   development of application based on FMLS.

5. **Based on web technology**: The web technology is better for cross platform,
   and easy to development.

   (Such as vue, electron, GeckoView)

6. Use IPv6 first.

## Git repository

- **fmls** (this repo)

  The main document of FMLS, and core base code. Programming language: rust.
  LICENSE: LGPLv3+

  Include these component:

  - `libfmlsc`: (no_std) common base library (include code shared by libfmlsm
    and libfmls)

  - `libfmlsm`: (no_std) for (r2) low resource device (such as MCU)

  - `libfmls`: base library (include most code of FMLS)

  - `fmlsd`: daemon process, run in background

  - `fmls-cli`: command line tool

- **fmls-vue**

  Graphical interface. Programming language: js, rust. LICENSE: GPLv3+

  For PC platform such as GNU/Linux, Windows.

  Main dependencies:

  - vue3

  - electron

  - sqlite3

- **fmls-apk**

  Android application. Programming language: kotlin. LICENSE: GPLv3+

  For handset (smart phone), tablet PC, etc.

  Main dependencies:

  - GeckoView

## LICENSE

[GNU Lesser General Public License v3.0 or later](https://www.gnu.org/licenses/lgpl-3.0-standalone.html)
(SPDX Identifier: `LGPL-3.0-or-later`)
