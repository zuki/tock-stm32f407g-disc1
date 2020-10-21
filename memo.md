# tock-stm32f407g-disc1の構築

## Projectの作成

```
$ mkdir tock-stm32f407g-disc1
$ cd tock-stm32f407g-disc1
$ mkdir boards chips
$ cp -r tock/boards/stm32f407g_disc1 boards/stm32f407g_disc1
$ cp -r tock/chips/stm32f407vg chips/stm32f407vg
$ vi files  # tock内のファイルへのリンクを修正
$ git submodule add https://github.com/tock/tock.git tock
Cloning into '/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock'...
remote: Enumerating objects: 40, done.
remote: Counting objects: 100% (40/40), done.
remote: Compressing objects: 100% (28/28), done.
remote: Total 66784 (delta 23), reused 21 (delta 12), pack-reused 66744
Receiving objects: 100% (66784/66784), 143.47 MiB | 3.77 MiB/s, done.
Resolving deltas: 100% (49490/49490), done.
```

## コンパイル1回目

```
$ make all
   Compiling tock-cells v0.1.0 (/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/libraries/tock-cells)
   Compiling tock-registers v0.5.0 (/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/libraries/tock-register-interface)
   Compiling enum_primitive v0.1.0 (/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/libraries/enum_primitive)
   Compiling tock-rt0 v0.1.0 (/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/libraries/tock-rt0)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> libraries/tock-cells/src/lib.rs:3:1
  |
3 | #![feature(const_fn)]
  | ^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> libraries/tock-cells/src/lib.rs:11:1
   |
11 | #![feature(option_result_contains)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> libraries/tock-register-interface/src/lib.rs:5:1
  |
5 | #![feature(const_fn)]
  | ^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0554`.
error: could not compile `tock-cells`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0554`.
error: build failed
make: *** [/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1] Error 101
```

## nightlyに変更してコンパイル2回目

```
$ rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin unchanged - rustc 1.48.0-nightly (e2be5f568 2020-09-09)

$ make all
...
    Finished release [optimized] target(s) in 1.42s
   text	   data	    bss	    dec	    hex	filename
  86017	   2036	   6156	  94209	  17001	/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
a6c737c433fb852a765231d90bac462e1861cb6256e58b5cd3c71b45b72fcc95  /Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.bin
```

## tockをプログラム

```
$ make flash
   Compiling stm32f407g_disc1 v0.1.0 (/Users/dspace/develop/rust/tock-stm32f407g-disc1/boards/stm32f407g_disc1)
    Finished release [optimized] target(s) in 1.45s
   text	   data	    bss	    dec	    hex	filename
  86017	   2036	   6156	  94209	  17001	/Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
openocd -f openocd.cfg -c "init; reset halt; flash write_image erase /Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.elf; verify_image /Users/dspace/develop/rust/tock-stm32f407g-disc1/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.elf; reset; shutdown"
Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
WARNING: interface/stlink-v2-1.cfg is deprecated, please switch to interface/stlink.cfg
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
srst_only separate srst_nogate srst_open_drain connect_deassert_srst

Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.896161
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Error: mem2array: Read @ 0xe0042004, w=4, cnt=1, failed
Error executing event examine-end on target stm32f4x.cpu:
/usr/local/Cellar/open-ocd/HEAD-762ddcb/bin/../share/openocd/scripts/mem_helper.tcl:6: Error:
in procedure 'ocd_process_reset'
in procedure 'ocd_process_reset_inner' called at file "embedded:startup.tcl", line 288
in procedure 'mmw' called at file "/usr/local/Cellar/open-ocd/HEAD-762ddcb/bin/../share/openocd/scripts/target/stm32f4x.cfg", line 79
in procedure 'mrw' called at file "/usr/local/Cellar/open-ocd/HEAD-762ddcb/bin/../share/openocd/scripts/mem_helper.tcl", line 36
at file "/usr/local/Cellar/open-ocd/HEAD-762ddcb/bin/../share/openocd/scripts/mem_helper.tcl", line 6
Info : Previous state query failed, trying to reconnect
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : device id = 0x10076413
Info : flash size = 1024 kbytes
Info : Flash write discontinued at 0x080157f4, next section at 0x08080000
Polling target stm32f4x.cpu failed, trying to reexamine
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
shutdown command invoked
```

## アプリをロード

```
$ cd libtock-c/examples
$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Installing app bundle. Size: 2048 bytes.
[INFO   ] Finished in 3.909 seconds
```


# LIS3DSHの温度測定値について

[LIS3DSHの温度出力に関する質問と回答](https://community.st.com/s/question/0D50X00009Xke66/lis3dsh-temperature-output)によると、この測定値は未公開の基準値に対する差を表す数値らしい。変化を知ることはできるが絶対温度はわからないらしい。現在は測定値が"-1"となってしまい修正をしようと思ったが、あまり意味がないのでソースはこのまま残すが使用しないことにした。

# LIS3DSHの措定値がおかしい

ようやく測定できるようになったが、値がおかしい。以下の測定値はボードを置いた状態で測定したもので、数値は（生測定値、変換値)
のペア。

1. 測定毎に大きくバラけている。
2. 動かしていないので変換値は理想的には(0, 0, 1)になるはずがそのような測定回がない。

```
x: (-31745,  -1.904) y: (9202,  0.552) z: (-20732,  -1.243)
x: (2815,  0.168) y: (6647,  0.398) z: (-3580, -0.214)
x: (7935,  0.476) y: (-11781, -0.706) z: (30980,  1.858)
x: (-5377, -0.322) y: (-19978,  -1.198) z: (16129,  0.967)
x: (25855,  1.551) y: (-17929,  -1.75) z: (-3837, -0.230)
x: (30975,  1.858) y: (3573,  0.214) z: (-12284, -0.737)
x: (-7425, -0.445) y: (12020,  0.721) z: (14084,  0.845)
x: (25855,  1.551) y: (5366,  0.321) z: (-5372, -0.322)
x: (28415,  1.704) y: (4856,  0.291) z: (27909,  1.674)
x: (25599,  1.535) y: (-24328,  -1.459) z: (-27644,  -1.658)
```

## DMAを使わないspiドライバ

上の値は`chips/stm32f4xx`のspiドライバを元に作成したspiドライバによるものである。この元ドライバは
DMAを使っているが、SPI３がつながっているDMA1コントローラしか対応していなかった。TM32F407G-DISC1の
LIS3DSHはSPI1につながっており、SPI1はDMA2に接続しているため、spiドライバはそのままでは使えず、対応を
試みたのが上の結果だった。`chips/stm32f303xc`のspiドライバを見たところ、SPI1を使っておりDMAは
使っていないがそのまま使えそうだったので、今度はこのドライバを使うことにした。

しかし、stm32f3はRx, TxバッファがFIFOになっており、stm32f303xcのspiドライバは`SPIx_SR::FRLVL`を
見て連続受信するようになっているが、stm32f4のRx, Txバッファは1バイトしかなく`SPIx_SR::FRLVL`フラグも
ないため、この部分を改造する必要があった。DMAを使用しない改造版も最初は結果はDMAを使ったものと同じように
バラけた値が得られた。

調査した結果、そもそも読み取りがうまく行っていないことが判明した。たとえば、WHO_AM_I idの値を
読み込むと`0xFF`という意味のない値であった。なお、DMAを使用したドライバも読み取りがうまく行っていない
ことが判明した（2バイト目の取得前に処理が終わっているように思われる）がこちらは修正方法がまだわからない。

修正後は、以下のように上向きにした場合のZ軸の値がほぼ1で、下向きにした場合はほぼ−1になっている。また、
WHO_AM_I idの値も`正しく`0x3F`になったので、これで正しいと思われる。

### (1) 上向き

```
LIS3DSH sensor is present: id = 3F
LIS3DSH device set power mode
LIS3DSH device set scale

x: (2162,  0.129) y: (-51,  -0.003) z: (16243,  0.974)
x: (2332,  0.139) y: (-33,  -0.001) z: (16186,  0.971)
x: (2692,  0.161) y: (135,   0.008) z: (17300,  1.037)
x: (2928,  0.175) y: (-112, -0.006) z: (16511,  0.990)
x: (2953,  0.177) y: (-391, -0.023) z: (16205,  0.972)
x: (2733,  0.163) y: (-369, -0.022) z: (16811,  1.008)
x: (2779,  0.166) y: (94,    0.005) z: (16768,  1.006)
x: (3205,  0.192) y: (-96,  -0.005) z: (16826,  1.009)
x: (2517,  0.151) y: (-18,  -0.001) z: (17199,  1.031)
x: (2826,  0.169) y: (31,    0.001) z: (16682,  1.000)

```

### (2) 下向き

```
x: (-734,  -0.044) y: (-167, -0.010) z: (-16061,  -0.963)
x: (3,      0.000) y: (1597,  0.095) z: (-17361,  -1.041)
x: (-907,  -0.054) y: (-832, -0.049) z: (-15690,  -0.941)
x: (-810,  -0.048) y: (33,    0.001) z: (-16318,  -0.979)
x: (-386,  -0.023) y: (-695, -0.041) z: (-15987,  -0.959)
x: (-915,  -0.054) y: (-466, -0.027) z: (-15948,  -0.956)
x: (-651,  -0.039) y: (-401, -0.024) z: (-16078,  -0.964)
x: (-1354, -0.081) y: (-290, -0.017) z: (-16205,  -0.972)
x: (-988,  -0.059) y: (-579, -0.034) z: (-15824,  -0.949)
x: (-1010, -0.060) y: (-714, -0.042) z: (-15771,  -0.946)
```

## NinedorfドライバからLIS3DSHを使用する

NinedofrドライバによるLIS3DSHの読み取りもできていた。下記の出力で単位はミリGである。

```
[Sensors] Starting Ninedorf App.
Acceleration: X: 117 Y: 5 Z: 1050
```

ただし、LIS3DSHを使う複数のアプリを同時実行させるとpanicとなる。

```
[Sensors] Starting Ninedorf App.
LIS3DSH sensor is present: id = 3F
Acceleration: X: 136 Y: 27 Z: 1000

panicked at 'Process lis3dsh had a fault', kernel/src/process.rs:987:17
        Kernel version 1a353a3

---| No debug queue found. You can set it with the DebugQueue component.

---| Fault Status |---
Data Access Violation:              true
Forced Hard Fault:                  true
Faulting Memory Address:            0x0000000C
Fault Status Register (CFSR):       0x00000082
Hard Fault Status Register (HFSR):  0x40000000

---| App Status |---
App: lis3dsh   -   [Fault]
 Events Queued: 0   Syscall Count: 16   Dropped Callback Count: 0
 Restart Count0

 ╔═══════════╤═════════════════════╤════════════════════╗
 ║  Address  │ Region Name    Used | Allocated (bytes)  ║
 ╚0x20006000═╪═════════════════════╪════════════════════╝
             │ ▼ Grant        1140 |   1140          
  0x20005B8C ┼─────────────────────────────────────────
             │ Unused
  0x20005088 ┼─────────────────────────────────────────
             │ ▲ Heap         1536 |   4356             S
  0x20004A88 ┼───────────────────────────────────────── R
             │ Data            648 |    648             A
  0x20004800 ┼───────────────────────────────────────── M
             │ ▼ Stack         616 |   2048          
  0x20004598 ┼─────────────────────────────────────────
             │ Unused
  0x20004000 ┴─────────────────────────────────────────
             .....
  0x08084000 ┬───────────────────────────────────────── F
             │ App Flash     16340                      L
  0x0808002C ┼───────────────────────────────────────── A
             │ Protected        44                      S
  0x08080000 ┴───────────────────────────────────────── H

  R0 : 0x00080010    R6 : 0x08082A08
  R1 : 0x20004000    R7 : 0x00080010
  R2 : 0x00000024    R8 : 0x00000000
  R3 : 0x00080083    R10: 0x00000000
  R4 : 0xFFFFFFFF    R11: 0x00000000
  R5 : 0x20004000    R12: 0x00000000
  R9 : 0x000001C2 (Static Base Register)
  SP : 0x20004758 (Process Stack Pointer)
  LR : 0x08080E73
  PC : 0x08080028
 YPC : 0x08080630

 APSR: N 0 Z 0 C 0 V 0 Q 0
       GE 0 0 0 0
 EPSR: ICI.IT 0x00
       ThumbBit true 

 Cortex-M MPU
  Region 0: [0x20004000:0x20006000], length: 8192 bytes; ReadWrite (0x3)
    Sub-region 0: [0x20004000:0x20004400], Enabled
    Sub-region 1: [0x20004400:0x20004800], Enabled
    Sub-region 2: [0x20004800:0x20004C00], Enabled
    Sub-region 3: [0x20004C00:0x20005000], Enabled
    Sub-region 4: [0x20005000:0x20005400], Enabled
    Sub-region 5: [0x20005400:0x20005800], Disabled
    Sub-region 6: [0x20005800:0x20005C00], Disabled
    Sub-region 7: [0x20005C00:0x20006000], Disabled
  Region 1: [0x08080000:0x08084000], length: 16384 bytes; UnprivilegedReadOnly )
    Sub-region 0: [0x08080000:0x08080800], Enabled
    Sub-region 1: [0x08080800:0x08081000], Enabled
    Sub-region 2: [0x08081000:0x08081800], Enabled
    Sub-region 3: [0x08081800:0x08082000], Enabled
    Sub-region 4: [0x08082000:0x08082800], Enabled
    Sub-region 5: [0x08082800:0x08083000], Enabled
    Sub-region 6: [0x08083000:0x08083800], Enabled
    Sub-region 7: [0x08083800:0x08084000], Enabled
  Region 2: Unused
  Region 3: Unused
  Region 4: Unused
  Region 5: Unused
  Region 6: Unused
  Region 7: Unused

To debug, run `make debug RAM_START=0x20004000 FLASH_INIT=0x8080055`

in the app's folder and open the .lst file.

App: ninedorf   -   [Yielded]

 Events Queued: 0   Syscall Count: 20   Dropped Callback Count: 0
 Restart Count0

 ╔═══════════╤═════════════════════╤════════════════════╗
 ║  Address  │ Region Name    Used | Allocated (bytes)  ║
 ╚0x20008000═╪═════════════════════╪════════════════════╝
             │ ▼ Grant        1176 |   1176          
  0x20007B68 ┼─────────────────────────────────────────
             │ Unused
  0x20007074 ┼─────────────────────────────────────────
             │ ▲ Heap         1536 |   4340             S
  0x20006A74 ┼───────────────────────────────────────── R
             │ Data            628 |    628             A
  0x20006800 ┼───────────────────────────────────────── M
             │ ▼ Stack         472 |   2048          
  0x20006628 ┼─────────────────────────────────────────
             │ Unused
  0x20006000 ┴─────────────────────────────────────────
             .....
  0x08086000 ┬───────────────────────────────────────── F
             │ App Flash      8148                      L
  0x0808402C ┼───────────────────────────────────────── A
             │ Protected        44                      S
  0x08084000 ┴───────────────────────────────────────── H

  R0 : 0x20007068    R6 : 0x00000024
  R1 : 0x20006914    R7 : 0x20007038
  R2 : 0x00000000    R8 : 0x20006A6C
  R3 : 0x00000000    R10: 0x00000000
  R4 : 0x20007068    R11: 0x00000000
  R5 : 0x00000000    R12: 0xF35D77EB
  R9 : 0x20006800 (Static Base Register)
  SP : 0x20006628 (Process Stack Pointer)
  LR : 0x08084267
  PC : 0x08084248
 YPC : 0x08084248

 APSR: N 0 Z 1 C 1 V 0 Q 0
       GE 0 0 0 0
 EPSR: ICI.IT 0x00
       ThumbBit true 

 Cortex-M MPU
  Region 0: [0x20006000:0x20008000], length: 8192 bytes; ReadWrite (0x3)
    Sub-region 0: [0x20006000:0x20006400], Enabled
    Sub-region 1: [0x20006400:0x20006800], Enabled
    Sub-region 2: [0x20006800:0x20006C00], Enabled
    Sub-region 3: [0x20006C00:0x20007000], Enabled
    Sub-region 4: [0x20007000:0x20007400], Enabled
    Sub-region 5: [0x20007400:0x20007800], Disabled
    Sub-region 6: [0x20007800:0x20007C00], Disabled
    Sub-region 7: [0x20007C00:0x20008000], Disabled
  Region 1: [0x08084000:0x08086000], length: 8192 bytes; UnprivilegedReadOnly ()
    Sub-region 0: [0x08084000:0x08084400], Enabled
    Sub-region 1: [0x08084400:0x08084800], Enabled
    Sub-region 2: [0x08084800:0x08084C00], Enabled
    Sub-region 3: [0x08084C00:0x08085000], Enabled
    Sub-region 4: [0x08085000:0x08085400], Enabled
    Sub-region 5: [0x08085400:0x08085800], Enabled
    Sub-region 6: [0x08085800:0x08085C00], Enabled
    Sub-region 7: [0x08085C00:0x08086000], Enabled
  Region 2: Unused
  Region 3: Unused
  Region 4: Unused
  Region 5: Unused
  Region 6: Unused
  Region 7: Unused

To debug, run `make debug RAM_START=0x20006000 FLASH_INIT=0x8084055`
in the app's folder and open the .lst file.
```