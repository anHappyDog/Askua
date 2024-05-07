.section .text.exc_gen_entry
#save all the registers and set a0 to current sp to 
#pass the context param
.macro SAVE_ALL
    csrrw sp, sscratch, sp
    bnez sp,trap_from_user

trap_from_kernel:
    csrr sp, sscratch
trap_from_user:
# for the context size ,should store carefully
    addi sp, sp, -36*8    
    sd ra , 1*8(sp)
#    sd sp , 2*8(sp)
    sd gp , 3*8(sp)
    sd tp , 4*8(sp)
    sd t0 , 5*8(sp)
    sd t1 , 6*8(sp)
    sd t2 , 7*8(sp)
    sd s0 , 8*8(sp)
    sd s1 , 9*8(sp)
    sd a0 , 10*8(sp)
    sd a1 , 11*8(sp)
    sd a2 , 12*8(sp)
    sd a3 , 13*8(sp)
    sd a4 , 14*8(sp)
    sd a5 , 15*8(sp)
    sd a6 , 16*8(sp)
    sd a7 , 17*8(sp)
    sd s2 , 18*8(sp)
    sd s3 , 19*8(sp)
    sd s4 , 20*8(sp)
    sd s5 , 21*8(sp)
    sd s6 , 22*8(sp)
    sd s7 , 23*8(sp)
    sd s8 , 24*8(sp)
    sd s9 , 25*8(sp)
    sd s10 , 26*8(sp)
    sd s11 , 27*8(sp)
    sd t3 , 28*8(sp)
    sd t4 , 29*8(sp)
    sd t5 , 30*8(sp)
    sd t6 , 31*8(sp)
    csrrw s0,sscratch,sp 
    csrr s1,sstatus
    csrr s2,sepc
    csrr s3,stval
    csrr s4,scause
    sd s0,2*8(sp)
    sd s1,32*8(sp)
    sd s2,33*8(sp)
    sd s3,34*8(sp)
    sd s4,35*8(sp)
.endm 

.macro RESTORE_ALL
    ld s1, 32*8(sp)
    ld s2, 33*8(sp)
    andi s0,s1,1<<8
    bnez s0, _to_kernel 
_to_user:
    addi s0,sp,36*8
    csrw sscratch,s0
_to_kernel:
    csrw sstatus,s1
    csrw sepc,s2
     
    ld ra , 1*8(sp)
    ld gp , 3*8(sp)
    ld tp , 4*8(sp)
    ld t0 , 5*8(sp)
    ld t1 , 6*8(sp)
    ld t2 , 7*8(sp)
    ld s0 , 8*8(sp)
    ld s1 , 9*8(sp)
    ld a0 , 10*8(sp)
    ld a1 , 11*8(sp)
    ld a2 , 12*8(sp)
    ld a3 , 13*8(sp)
    ld a4 , 14*8(sp)
    ld a5 , 15*8(sp)
    ld a6 , 16*8(sp)
    ld a7 , 17*8(sp)
    ld s2 , 18*8(sp)
    ld s3 , 19*8(sp)
    ld s4 , 20*8(sp)
    ld s5 , 21*8(sp)
    ld s6 , 22*8(sp)
    ld s7 , 23*8(sp)
    ld s8 , 24*8(sp)
    ld s9 , 25*8(sp)
    ld s10 , 26*8(sp)
    ld s11 , 27*8(sp)
    ld t3 , 28*8(sp)
    ld t4 , 29*8(sp)
    ld t5 , 30*8(sp)
    ld t6 , 31*8(sp)
    ld sp , 2*8(sp)
.endm
    .align 8
    .global exc_gen_entry
    .type exc_gen_entry, @function
exc_gen_entry:
.align 2
    j trap_entry
.align 2
    j handle_s_software_int
.align 2
    j handle_reserved_int
.align 2
    j handle_m_software_int
.align 2
    j handle_reserved_int
.align 2
    j handle_s_timer_int
.align 2
    j handle_reserved_int
.align 2
    j handle_m_timer_int
.align 2
    j handle_reserved_int
.align 2
    j handle_s_extern_int
.align 2
    j handle_reserved_int
.align 2
    j handle_m_extern_int
.global trap_entry
.type trap_entry, @function
.align 8
trap_entry:
    SAVE_ALL
    mv a0,sp
    call trap_handler
    call exc_return
    .global exc_return
    .type exc_return, @function
exc_return:
    RESTORE_ALL
    sret
    .align 1

