# .align 3 # 2^3=8; 2^2=4 is OK, but less performance 
.text
.global __m_trap_entry
__m_trap_entry:
# __save_frame:
    csrrw sp, mscratch, sp

    # store ra, then register ra is free to use, we just want a free register
    sd ra, -34*8(sp)

    # ra = mscratch, mscratch = sp
    # that is, we restored mscratch, sp is still mscratch, ra stored sp
    # the mscratch is restored, no more restore ops needed later.
    csrrw ra, mscratch, sp

    addi sp, sp, -34*8

    # store this ra(actually sp) to (mscratch)[1]:usize
    sd ra, 8*1(sp)

    sd gp, 8*2(sp)
    sd tp, 8*3(sp)
    sd t0 , 8*4(sp)
    sd t1 , 8*5(sp)
    sd t2 , 8*6(sp)
    sd s0 , 8*7(sp)
    sd s1 , 8*8(sp)
    sd a0 , 8*9(sp)
    sd a1 , 8*10(sp)
    sd a2 , 8*11(sp)
    sd a3 , 8*12(sp)
    sd a4 , 8*13(sp)
    sd a5 , 8*14(sp)
    sd a6 , 8*15(sp)
    sd a7 , 8*16(sp)
    sd s2 , 8*17(sp)
    sd s3 , 8*18(sp)
    sd s4 , 8*19(sp)
    sd s5 , 8*20(sp)
    sd s6 , 8*21(sp)
    sd s7 , 8*22(sp)
    sd s8 , 8*23(sp)
    sd s9 , 8*24(sp)
    sd s10, 8*25(sp)
    sd s11, 8*26(sp)
    sd t3 , 8*27(sp)
    sd t4 , 8*28(sp)
    sd t5 , 8*29(sp)
    sd t6 , 8*30(sp)

    # t0 is free to use now
    csrr t0, mepc
    sd t0, 8*31(sp)
    
    csrr t0, mstatus
    sd t0, 8*32(sp)

    csrr t0, satp
    sd t0, 8*33(sp)
    

    # // to do ...

    mv a0, sp

    call m_trap_handler

    # # Context ret from m_trap_handler  -- unneeded
    # mv sp, a0

__restore_frame:
    # // to do ...

    # restore satp
    ld t0, 8*33(sp)
    csrw satp, t0
    sfence.vma

    # restore mstatus
    ld t0, 8*32(sp)
    csrw mstatus, t0

    # restore mepc
    ld t0, 8*31(sp)
    csrw mepc, t0

    ld ra, 8*0(sp)
    # sp? restore it later
    ld gp, 8*2(sp)
    ld tp, 8*3(sp)
    ld t0 , 8*4(sp)
    ld t1 , 8*5(sp)
    ld t2 , 8*6(sp)
    ld s0 , 8*7(sp)
    ld s1 , 8*8(sp)
    ld a0 , 8*9(sp)
    ld a1 , 8*10(sp)
    ld a2 , 8*11(sp)
    ld a3 , 8*12(sp)
    ld a4 , 8*13(sp)
    ld a5 , 8*14(sp)
    ld a6 , 8*15(sp)
    ld a7 , 8*16(sp)
    ld s2 , 8*17(sp)
    ld s3 , 8*18(sp)
    ld s4 , 8*19(sp)
    ld s5 , 8*20(sp)
    ld s6 , 8*21(sp)
    ld s7 , 8*22(sp)
    ld s8 , 8*23(sp)
    ld s9 , 8*24(sp)
    ld s10, 8*25(sp)
    ld s11, 8*26(sp)
    ld t3 , 8*27(sp)
    ld t4 , 8*28(sp)
    ld t5 , 8*29(sp)
    ld t6 , 8*30(sp)
    
    # addi sp, sp, 32*8 # 0-31  # unnecessary

    # restore sp
    ld sp , 8*1(sp)

    mret



# Uart  write c_char ops
    # sd a0, 0(sp)
    # sd a1, 8(sp)
    # li a0, 0x10000000
    # li a1, 0x30  # '0'
    # sd a1, 0(a0)
    # addi a1, x0, 0x31 # '1'
    # sd a1, 0(a0)

    # ld a0, 0(sp)
    # ld a1, 8(sp)
