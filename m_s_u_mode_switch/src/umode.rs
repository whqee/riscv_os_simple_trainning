use crate::println;

pub fn umain() -> ! {
    println!("[U][Info] hello umain! ");
    // println!("[U][Info] hello ! mstatus:{:x}", crate::smode::read_sstatus());

    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_M });
    println!("[U][Info] 第二次 ");
    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_M });
    println!("[U][Info] 第三次 ");
    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_M });

    todo!()
}
