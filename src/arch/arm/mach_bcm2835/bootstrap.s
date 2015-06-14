.section .init,"x"
.globl _start
_start:
    mov sp,#0x00010000
    bl main
hang: b hang

