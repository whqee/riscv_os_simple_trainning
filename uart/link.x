/*
link.x:   whqee#qq.com, 2022-08-19(YYYY-MM-DD) created.

Fork from https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds

Descriptions:   Linker script for outputting to RISC-V QEMU "virt" machine.
Author:         Stephen Marz
Date:           6 October 2019

*/

OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    /* writable, executable, allocatable */
    ram  (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/* program headers. */
/* PT_LOAD tells the linker that these will be loaded from the file into Mem */
PHDRS
{
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS
{
    .text : {
        PROVIDE(_text_start = .);
        *(.text.init) *(.text .text.*)
        PROVIDE(_text_end = .);
    } >ram AT>ram :text

    PROVIDE(_global_pointer = .);

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >ram AT>ram :text

    .data : {
        . = ALIGN(4096);
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*) *(.data .data.*)
        PROVIDE(_data_end = .);
    } >ram AT>ram :data

    .bss : {
        *(.bss.stack)
        PROVIDE(_bss_start = .);
        PROVIDE(sbss = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
        PROVIDE(ebss = .);
    } >ram AT>ram :bss


    PROVIDE(_memory_start = ORIGIN(ram));

    PROVIDE(_stack_start = _bss_end);
    PROVIDE(_stack_end = _stack_start + 0x80000);
    PROVIDE(_stack_top = _stack_end);

    PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
    
    PROVIDE(_heap_start = _stack_end);
    PROVIDE(_heap_size = _memory_end - _heap_start);
}
