// trap.S
//
// This file contains relevant assembly labels for handling
// {traps, interrupts, exceptions}. When any of these traps are taken
// trap_handler will begin executing by saving the current register context
// and jumping to the Rust |handle_trap| function, which will determine how
// to handle the given cause.

.altmacro

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
  sw   t6, 4(sp)

  // make t6 the new "sp" to store the rest of the registers
  add  t6, sp, 4

  // start at 1 because we don't need to save 0 register
  .set i, 1
  .rept 30
    sv %i
    .set i, i+1
  .endr

  // save mepc - the instruction that was interrupted by the trap.
  // arg0 = mcause
  // arg1 = mepc
  csrr a0, mcause
  csrr a1, mepc

  call handle_trap

  // Save the return value, because that's where we need to return to.
  // The checks to determine if the next instruction is offset 2 or 4
  // due to compressed instructions is done in Rust
  sw  a0, 4*32(sp)

  // start at 1 because we don't need to load 0 register
  .set i, 1
  .rept 30
    lv %i
    .set i, i+1
  .endr

  // restore t6, which was saved at the beginning
  lw t6, 4(sp)

  // restore the old return address to the mepc so we jump back to the correct
  // location that we left when we trapped
  lw a0, 4*32(sp)

  csrw mepc,a0

  mret
