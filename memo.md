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