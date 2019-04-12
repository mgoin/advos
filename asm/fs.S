/* fs.S
 *
 * Simple assembly file to keep the definition of |FILE_SYSTEM|
 * out of the other irrelevant code. Export |FILE_SYSTEM|
 * so it's accessible from Rust.
 *
 */
.option norvc

.section .rodata
.align 4
.global FILE_SYSTEM
FILE_SYSTEM: .incbin "asm/filesystem.bin"
