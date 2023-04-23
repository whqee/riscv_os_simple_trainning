#!/bin/bash

cargo build --release && 
rust-objcopy target/riscv64gc-unknown-none-elf/release/virt_pages_with_mmu_on -O binary myos.bin &&
rust-objdump -D target/riscv64gc-unknown-none-elf/release/virt_pages_with_mmu_on > dis.asm
