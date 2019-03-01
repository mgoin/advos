/* mem.S
 * 
 * Simple assembly file to keep the definition of |HEAP_START| and |HEAP_END|
 * out of the other irrelevant code. Exports both |HEAP_START| and |HEAP_END|
 * so they're accessable from Rust.
 *
 */
.option norvc

.section .rodata
.global HEAP_START
HEAP_START: .word __heap_start
.global HEAP_END
HEAP_END: .word __heap_end
