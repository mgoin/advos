// trap.S
//
// This file contains relevant assembly labels for handling
// {traps, interrupts, exceptions}. When any of these traps are taken
// trap_handler will begin executing by saving the current register context
// and jumping to the Rust |handle_trap| function, which will determine how
// to handle the given cause.

// Macro to save a register to memory
.macro sv z, jacob=t6
    sw  x\z, ((\z)*4)(\jacob)
.endm

// Macro to load a register from memory
.macro lv z, jacob=t6
    lw  x\z, ((\z)*4)(\jacob)
.endm

// This is called from Rust, so we know we're in machine mode
// Once this returns, {traps, interrupts, exceptions} are enabled
// and can be triggered at any time.
.global enable_interrupts
enable_interrupts:
  // Enable interrupts with MPIE and MIE bits in mstatus
  // MPIE => Machine Previous Interrupt Enable
  //      => bit 7
  // MIE  => Machine Interrupt Enable
  //      => bit 3
  csrr  t0, mstatus
  li    t1, 0x88
  or    t0, t0, t1
  csrw  mstatus, t0

  // Enable HW|CLINT|SW interrupts
  // (bits) 11     7  3 in mie = 0x888
  li    t1, (1 << 11) | (1 << 7) | (1 << 3)
  csrrw zero, mie, t1

  // Set RISC-V trap handler function to be the mtvec
  la    t1, trap_handler
  
  // ensure that the trap handler is aligned by 4 since the lower 2 bits of
  // mtvec _must_ be 0 for MODE to be set to direct since our trap handler
  // handles all interrupts/traps
  li    t0, ~( (1 << 1) | 1 )
  and   t1, t1, t0 
  csrw  mtvec, t1

  ret

// RISC-V trap handler function has to be aligned by 4. This function will jump
// to the rust trap_handler where the trap will _actually_ be handled. This
// label simply saves registers and restores them upon the trap being
// successfully handled.
.global trap_handler
.align 4
trap_handler:
  // save the register context before overwriting anything
  // t6 will be the temporary register used to hold the stack
  // pointer as the registers are all saved, and 
  addi sp, sp, 4
  sw   t6, (sp)

  // restore sp to it's original value
  li   t6, 4
  sub  sp, sp, 4

  // make t6 the new "sp" to store the rest of the registers
  add  t6, sp, 4

  .set i, 0
  .rept 31
    sv %i
  .endr

  addi  sp, sp, 4
  lw    t6, (sp)
  sv 32, t5

  // save mepc - the instruction that was interrupted by the trap.
  // arg0 = mcause
  // arg1 = mepc
  csrr a0, mcause
  csrr a1, mepc
  mv   s0, ra

  call handle_trap

  // restore the old return address to the mepc so we jump back to the correct
  // location that we left when we trapped
  csrw mepc, s0

  mret

get_register_context:
  addi sp,sp,4
  sw   ra, (sp)
  addi sp,sp,4

  mv   ra,a0

  //mv   t6,sp
  .set i, 0
  .rept 31
    ssv %i
    .set i, i+1
  .endr


  li t6, 136
  sub sp, sp, t6
  lw  t6, (sp)
  ssv 31

  auipc t6, 0
  //mv t6, pc
  sw t6, (sp)

  add a0, sp, zero

  ret
