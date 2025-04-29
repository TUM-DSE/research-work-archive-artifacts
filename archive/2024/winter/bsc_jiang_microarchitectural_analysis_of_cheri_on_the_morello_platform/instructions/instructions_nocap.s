.global ldr_throughput_loop
.global ldr_latency_loop
.global str_throughput_loop
.global str_inplace_throughput_loop
.global ldar_throughput_loop
.global stlr_throughput_loop
.global ldp_throughput_loop
.global ldp_latency_loop
.global stp_throughput_loop
.global stp_inplace_throughput_loop

stp_inplace_throughput_loop:
    cbz x0, stpit_end
        sub x0, x0, #1
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
        stp x1, x2, [x1]
    b stp_inplace_throughput_loop

    stpit_end:
        ret

stp_throughput_loop:
    cbz x0, stpt_end
        sub x0, x0, #1
        stp x1, x2, [x1, #0]
        stp x1, x2, [x1, #16]
        stp x1, x2, [x1, #32]
        stp x1, x2, [x1, #48]
        stp x1, x2, [x1, #64]
        stp x1, x2, [x1, #80]
        stp x1, x2, [x1, #96]
        stp x1, x2, [x1, #112]
        stp x1, x2, [x1, #0]
        stp x1, x2, [x1, #16]
        stp x1, x2, [x1, #32]
        stp x1, x2, [x1, #48]
        stp x1, x2, [x1, #64]
        stp x1, x2, [x1, #80]
        stp x1, x2, [x1, #96]
        stp x1, x2, [x1, #112]
    b stp_throughput_loop

    stpt_end:
        ret

ldp_latency_loop:
    cbz x0, ldpl_end
        sub x0, x0, #1
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
        ldp x1, x2, [x1]
    b ldp_latency_loop

    ldpl_end:
        ret

ldp_throughput_loop:
    cbz x0, ldpt_end
        sub x0, x0, #1
        ldp x10, x11, [x1]
        ldp x12, x13, [x1]
        ldp x14, x15, [x1]
        ldp x9, x10, [x1]
        ldp x11, x12, [x1]
        ldp x13, x14, [x1]
        ldp x15, x9, [x1]
        ldp x10, x11, [x1]
        ldp x12, x13, [x1]
        ldp x14, x15, [x1]
        ldp x9, x10, [x1]
        ldp x11, x12, [x1]
        ldp x13, x14, [x1]
        ldp x15, x9, [x1]
        ldp x10, x11, [x1]
        ldp x12, x13, [x1]
    b ldp_throughput_loop

    ldpt_end:
        ret

ldr_throughput_loop:
    cbz x0, ldrt_end
        sub x0, x0, #1
        ldr x10, [x1]
        ldr x13, [x1]
        ldr x14, [x1]
        ldr x9, [x1]
        ldr x12, [x1]
        ldr x11, [x1]
        ldr x15, [x1]
        ldr x9, [x1]
        ldr x12, [x1]
        ldr x11, [x1]
        ldr x13, [x1]
        ldr x10, [x1]
        ldr x14, [x1]
        ldr x15, [x1]
        ldr x9, [x1]
        ldr x10, [x1]
    b ldr_throughput_loop

    ldrt_end:
        ret

ldr_latency_loop:
    mov x9, #0
    ldrl_loop:
    cbz x0, ldrl_end
        sub x0, x0, #1
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
        ldr x9, [x1, x9]
    b ldrl_loop
    ldrl_end:
        ret

str_inplace_throughput_loop:
    mov x9, #0
    strit_loop:
    cbz x0, strit_end
        sub x0, x0, #1
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
        str x9, [x1]
    b strit_loop

    strit_end:
        ret

str_throughput_loop:
    mov x9, #0
    strt_loop:
    cbz x0, strt_end
        sub x0, x0, #1
        str x9, [x1, #0]
        str x9, [x1, #8]
        str x9, [x1, #16]
        str x9, [x1, #24]
        str x9, [x1, #32]
        str x9, [x1, #40]
        str x9, [x1, #48]
        str x9, [x1, #56]
        str x9, [x1, #64]
        str x9, [x1, #72]
        str x9, [x1, #80]
        str x9, [x1, #88]
        str x9, [x1, #96]
        str x9, [x1, #104]
        str x9, [x1, #112]
        str x9, [x1, #120]
    b strt_loop

    strt_end:
        ret

ldar_throughput_loop:
    cbz x0, ldart_end
        sub x0, x0, #1
        ldar x10, [x1]
        ldar x13, [x1]
        ldar x14, [x1]
        ldar x9, [x1]
        ldar x12, [x1]
        ldar x11, [x1]
        ldar x15, [x1]
        ldar x9, [x1]
        ldar x12, [x1]
        ldar x11, [x1]
        ldar x13, [x1]
        ldar x10, [x1]
        ldar x14, [x1]
        ldar x15, [x1]
        ldar x9, [x1]
        ldar x10, [x1]
    b ldar_throughput_loop

    ldart_end:
        ret

stlr_throughput_loop:
    mov x9, #42
    stlrt_loop:
    cbz x0, stlrt_end
        sub x0, x0, #1
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
        stlr x9, [x1]
    b stlrt_loop

    stlrt_end:
        ret
