
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 7
    # .quad 1
    # .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_6_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "/home/whqee/tmp/test.bin"
    # .incbin "/home/whqee/devel/rust/riscv_os/app_linux/print_hello/print_hello.bin"
    # .incbin "../app_linux/print_hello/test.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_bad_instructions.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_bad_register.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_hello_world.bin"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_power_3.bin"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_power_5.bin"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "/home/whqee/devel/rust/lab1-os3-whqee/user/build/bin/ch2b_power_7.bin"
app_6_end:
