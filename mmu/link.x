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

    /* M mode stack */
    PROVIDE(m_stack_start = _bss_end);
    PROVIDE(m_stack_end = m_stack_start + 0x80000);
    PROVIDE(m_stack_top = m_stack_end);

    /* S mode stack */
    PROVIDE(s_stack_start = m_stack_end);
    PROVIDE(s_stack_end = s_stack_start + 0x80000);
    PROVIDE(s_stack_top = s_stack_end);

    /* U mode stack */
    PROVIDE(u_stack_start = s_stack_end);
    PROVIDE(u_stack_end = u_stack_start + 0x80000);
    PROVIDE(u_stack_top = u_stack_end);

    PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
    
    PROVIDE(_heap_start = u_stack_end);
    PROVIDE(_heap_size = _memory_end - _heap_start);
}
