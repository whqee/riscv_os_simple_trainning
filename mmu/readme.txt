# RISC-V MMU Test based on qemu-riscv64gc


bootup --> default M mode --> 

set up PMP, all mode free to RWX

map 0x8000_0000 - 0x8800_0000 to itself

switch MMU on

--> switch to S mode -->


RWX test

--> switch to U mode -->

RWX test

--> end

。。。
