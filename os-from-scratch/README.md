## OS from scratch (based on rv64)
A rust-based OS from scratch based on rv64.

## Targets

To realize an simple Rust-based OS that can run multiple user applications.

Stage 1: With task scheduler support to run multiple simple tasks with MMU enabled.

Stage 2: Run simple GUI applications.

Stage 3: With FFT support to run simple digital signal processing applications.


## Tasks Scheduling

思路一： 编译时计算对应处理器的不同时候的频率、功耗等，预先将各程序的指令和数据混合、排列，编译时安排好以至于运行时无需运行调度器。
系统调用处理加入重排指令程序，每当有新的用户程序加入，重新安排内存、插入指令和数据，无法提前编排的指令另外开一块动态内存配合使用，如接收未知大小的数据包。
（思路不够清晰）

思路二： 参考宏内核/微内核。略。


## Task Decompostion & Completions

Before having the ability to decompose tasks, read the risc-v documents and try some coding to get the abilities.

Decompose the task into some exercises, and complete them. 

Finally, design the target OS and realize it with Rust.

0. Small Exercises


| Exercise                                  | Progress     |
|-------------------------------------------|--------------|
| Uart                                      | :heavy_check_mark: |
| CLint(Timer)                              | ::heavy_check_mark:: |
| PLIC(External Interrupt)                  | :: |
| PMP                                       | ::heavy_check_mark:: |
| M/S/U Mode Switches                       | ::heavy_check_mark:: |
| MMU                                       | :: |
| Page Heap                                 | :: |
| Heap (virtual address)                    | :: |
| Multi-Hart                                | :: |
| Atomic Load/Store                         | :: |
| Block driver                              | :: |
<!-- | Support of Rust Vec & String              | :: | -->
<!-- | Exception-Handle: ecall                   | :: | -->
<!-- | Filesystems                               | :: | -->

1. Target of Stage 1
...

2. Target of Stage 2
todo!()

2. Target of Stage 3
todo!()


## Daily Records

+ 2022-07-02 rustlings 44/84
+ 2022-07-03 rustlings 68/84
+ 2022-07-04 rustlings 83/84 +  Lock free concurrency
+ 2022-07-05 rustlings 84/84


## Documentation

todo!()

## Files Hierarchy
    .
    ├── build.rs
    ├── Cargo.toml
    ├── 
    todo!()




todo!()


### References

todo!()