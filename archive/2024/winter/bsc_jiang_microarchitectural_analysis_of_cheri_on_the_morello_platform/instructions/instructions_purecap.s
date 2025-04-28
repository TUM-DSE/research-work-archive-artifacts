.global ldr_throughput_loop
.global ldr_latency_loop
.global str_throughput_loop
.global str_cap_throughput_loop
.global ldar_throughput_loop
.global stlr_throughput_loop
.global ldr_cap_throughput_loop
.global ldp_throughput_loop
.global ldp_cap_throughput_loop
.global stp_throughput_loop
.global stp_cap_throughput_loop
.global ldr_cap_latency_loop
.global ldp_latency_loop
.global ldp_cap_latency_loop
.global cvtd_tocap_throughput_loop
.global cvtd_tocap_latency_loop
.global cfhi_throughput_loop
.global cfhi_latency_loop
.global cthi_throughput_loop
.global cthi_latency_loop
.global cvtd_toptr_throughput_loop
.global cvtd_toptr_latency_loop
.global cvtp_tocap_throughput_loop
.global cvtp_tocap_latency_loop
.global cvtp_toptr_throughput_loop
.global cvtp_toptr_latency_loop
.global cvt_tocap_throughput_loop
.global cvt_tocap_latency_loop
.global cvt_toptr_throughput_loop
.global cvt_toptr_latency_loop
.global str_inplace_throughput_loop
.global str_cap_inplace_throughput_loop
.global stp_inplace_throughput_loop
.global stp_cap_inplace_throughput_loop

cvt_toptr_latency_loop:
    cbz x0, cvttpl_end
    sub x0, x0, #1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
        cvt x1, c1, c1
    b cvt_toptr_latency_loop

    cvttpl_end:
        ret

cvt_toptr_throughput_loop:
    cbz x0, cvttpt_end
    sub x0, x0, #1
        cvt x9, c1, c1
        cvt x10, c1, c1
        cvt x11, c1, c1
        cvt x12, c1, c1
        cvt x13, c1, c1
        cvt x14, c1, c1
        cvt x15, c1, c1
        cvt x9, c1, c1
        cvt x10, c1, c1
        cvt x11, c1, c1
        cvt x12, c1, c1
        cvt x13, c1, c1
        cvt x14, c1, c1
        cvt x15, c1, c1
        cvt x9, c1, c1
        cvt x10, c1, c1
    b cvt_toptr_throughput_loop

    cvttpt_end:
        ret


cvt_tocap_latency_loop:
    cbz x0, cvttcl_end
    sub x0, x0, #1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
        cvt c1, c1, x1
    b cvt_tocap_latency_loop

    cvttcl_end:
        ret

cvt_tocap_throughput_loop:
    cbz x0, cvttct_end
    sub x0, x0, #1
        cvt c9, c1, x1
        cvt c10, c1, x1
        cvt c11, c1, x1
        cvt c12, c1, x1
        cvt c13, c1, x1
        cvt c14, c1, x1
        cvt c15, c1, x1
        cvt c9, c1, x1
        cvt c10, c1, x1
        cvt c11, c1, x1
        cvt c12, c1, x1
        cvt c13, c1, x1
        cvt c14, c1, x1
        cvt c15, c1, x1
        cvt c9, c1, x1
        cvt c10,c1, x1
    b cvt_tocap_throughput_loop

    cvttct_end:
        ret

cvtp_toptr_latency_loop:
    cbz x0, cvtptpl_end
    sub x0, x0, #1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
        cvtp x1, c1
    b cvtp_toptr_latency_loop

    cvtptpl_end:
        ret

cvtp_toptr_throughput_loop:
    cbz x0, cvtptpt_end
    sub x0, x0, #1
        cvtp x9, c1
        cvtp x10, c1
        cvtp x11, c1
        cvtp x12, c1
        cvtp x13, c1
        cvtp x14, c1
        cvtp x15, c1
        cvtp x9, c1
        cvtp x10, c1
        cvtp x11, c1
        cvtp x12, c1
        cvtp x13, c1
        cvtp x14, c1
        cvtp x15, c1
        cvtp x9, c1
        cvtp x10, c1
    b cvtp_toptr_throughput_loop

    cvtptpt_end:
        ret


cvtp_tocap_latency_loop:
    cbz x0, cvtptcl_end
    sub x0, x0, #1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
        cvtp c1, x1
    b cvtp_tocap_latency_loop

    cvtptcl_end:
        ret

cvtp_tocap_throughput_loop:
    cbz x0, cvtptct_end
    sub x0, x0, #1
        cvtp c9, x1
        cvtp c10, x1
        cvtp c11, x1
        cvtp c12, x1
        cvtp c13, x1
        cvtp c14, x1
        cvtp c15, x1
        cvtp c9, x1
        cvtp c10, x1
        cvtp c11, x1
        cvtp c12, x1
        cvtp c13, x1
        cvtp c14, x1
        cvtp c15, x1
        cvtp c9, x1
        cvtp c10, x1
    b cvtp_tocap_throughput_loop

    cvtptct_end:
        ret

cvtd_toptr_latency_loop:
    cbz x0, cvtdtpl_end
    sub x0, x0, #1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
        cvtd x1, c1
    b cvtd_toptr_latency_loop

    cvtdtpl_end:
        ret

cvtd_toptr_throughput_loop:
    cbz x0, cvtdtpt_end
    sub x0, x0, #1
        cvtd x9, c1
        cvtd x10, c1
        cvtd x11, c1
        cvtd x12, c1
        cvtd x13, c1
        cvtd x14, c1
        cvtd x15, c1
        cvtd x9, c1
        cvtd x10, c1
        cvtd x11, c1
        cvtd x12, c1
        cvtd x13, c1
        cvtd x14, c1
        cvtd x15, c1
        cvtd x9, c1
        cvtd x10, c1
    b cvtd_toptr_throughput_loop

    cvtdtpt_end:
        ret


cthi_latency_loop:
    cbz x0, cthil_end
    sub x0, x0, #1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
        cthi c1, c1, x1
    b cthi_latency_loop

    cthil_end:
        ret

cthi_throughput_loop:
    cbz x0, cthit_end
    sub x0, x0, #1
        cthi c1, c1, x1
        cthi c2, c2, x2
        cthi c3, c3, x3
        cthi c4, c4, x4
        cthi c5, c5, x5
        cthi c6, c6, x6
        cthi c7, c7, x7
        cthi c1, c1, x1
        cthi c2, c2, x2
        cthi c3, c3, x3
        cthi c4, c4, x4
        cthi c5, c5, x5
        cthi c6, c6, x6
        cthi c1, c1, x1
        cthi c2, c2, x2
        cthi c3, c3, x3
    b cthi_throughput_loop

    cthit_end:
        ret

cfhi_latency_loop:
    cbz x0, cfhil_end
    sub x0, x0, #1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
        cfhi x1, c1
    b cfhi_latency_loop

    cfhil_end:
        ret

cfhi_throughput_loop:
    cbz x0, cfhit_end
    sub x0, x0, #1
        cfhi x2, c2
        cfhi x3, c3
        cfhi x4, c4
        cfhi x5, c5
        cfhi x6, c6
        cfhi x7, c7
        cfhi x1, c1
        cfhi x2, c2
        cfhi x3, c3
        cfhi x4, c4
        cfhi x5, c5
        cfhi x6, c6
        cfhi x1, c1
        cfhi x2, c2
        cfhi x3, c3
        cfhi x3, c3
    b cfhi_throughput_loop

    cfhit_end:
        ret


cvtd_tocap_latency_loop:
    cbz x0, cvtdtcl_end
    sub x0, x0, #1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
        cvtd c1, x1
    b cvtd_tocap_latency_loop

    cvtdtcl_end:
        ret



cvtd_tocap_throughput_loop:
    cbz x0, cvtdtct_end
    sub x0, x0, #1
        cvtd c9, x1
        cvtd c10, x1
        cvtd c11, x1
        cvtd c12, x1
        cvtd c13, x1
        cvtd c14, x1
        cvtd c15, x1
        cvtd c9, x1
        cvtd c10, x1
        cvtd c11, x1
        cvtd c12, x1
        cvtd c13, x1
        cvtd c14, x1
        cvtd c15, x1
        cvtd c9, x1
        cvtd c10, x1
    b cvtd_tocap_throughput_loop

    cvtdtct_end:
        ret


// extern void ldr_cap_latency_loop(long iterations, int* tmp);
ldr_cap_latency_loop:
    ldrcl_loop:
    cbz x0, ldrcl_end
        sub x0, x0, #1
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
        ldr c1, [c1]
    b ldr_cap_latency_loop

    ldrcl_end:
        ret

// extern void ldp_cap_latency_loop(long iterations, intptr_t* tmp);
ldp_cap_latency_loop:
    cbz x0, ldpcl_end
        sub x0, x0, #1
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
        ldp c1, c2, [c1]
    b ldp_cap_latency_loop

    ldpcl_end:
        ret


stp_cap_throughput_loop:
    stpct_loop:
    cbz x0, stpct_end
        sub x0, x0, #1
        stp c1, c2, [c1, #0]
        stp c1, c2, [c1, #32]
        stp c1, c2, [c1, #64]
        stp c1, c2, [c1, #96]
        stp c1, c2, [c1, #0]
        stp c1, c2, [c1, #32]
        stp c1, c2, [c1, #64]
        stp c1, c2, [c1, #96]
        stp c1, c2, [c1, #0]
        stp c1, c2, [c1, #32]
        stp c1, c2, [c1, #64]
        stp c1, c2, [c1, #96]
        stp c1, c2, [c1, #0]
        stp c1, c2, [c1, #32]
        stp c1, c2, [c1, #64]
        stp c1, c2, [c1, #96]
    b stpct_loop

    stpct_end:
        ret

stp_throughput_loop:
    mov x9, #42
    mov x10, #31
    stpt_loop:
    cbz x0, stpt_end
        sub x0, x0, #1
        stp x9, x10, [c1, #0]
        stp x9, x10, [c1, #16]
        stp x9, x10, [c1, #32]
        stp x9, x10, [c1, #48]
        stp x9, x10, [c1, #64]
        stp x9, x10, [c1, #80]
        stp x9, x10, [c1, #96]
        stp x9, x10, [c1, #112]
        stp x9, x10, [c1, #0]
        stp x9, x10, [c1, #16]
        stp x9, x10, [c1, #32]
        stp x9, x10, [c1, #48]
        stp x9, x10, [c1, #64]
        stp x9, x10, [c1, #80]
        stp x9, x10, [c1, #96]
        stp x9, x10, [c1, #112]
    b stpt_loop

    stpt_end:
        ret

stp_cap_inplace_throughput_loop:
    stpcit_loop:
    cbz x0, stpcit_end
        sub x0, x0, #1
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
        stp c1, c2, [c1]
    b stpcit_loop

    stpcit_end:
        ret

stp_inplace_throughput_loop:
    mov x9, #42
    mov x10, #31
    stpit_loop:
    cbz x0, stpit_end
        sub x0, x0, #1
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
        stp x9, x10, [c1]
    b stpit_loop

    stpit_end:
        ret

// extern void str_cap_throughput_loop(long iterations, int* tmp);
ldr_cap_throughput_loop:
    cbz x0, ldrct_end
        sub x0, x0, #1
        ldr c10, [c1]
        ldr c13, [c1]
        ldr c14, [c1]
        ldr c9, [c1]
        ldr c12, [c1]
        ldr c11, [c1]
        ldr c15, [c1]
        ldr c9, [c1]
        ldr c12, [c1]
        ldr c11, [c1]
        ldr c13, [c1]
        ldr c10, [c1]
        ldr c14, [c1]
        ldr c15, [c1]
        ldr c9, [c1]
        ldr c10, [c1]
    b ldr_cap_throughput_loop

    ldrct_end:
        ret

// extern void ldp_throughput_loop(long iterations, int* tmp);
ldp_throughput_loop:
    cbz x0, ldpt_end
        sub x0, x0, #1
        ldp x10, x11, [c1]
        ldp x12, x13, [c1]
        ldp x14, x15, [c1]
        ldp x9, x10, [c1]
        ldp x11, x12, [c1]
        ldp x13, x14, [c1]
        ldp x15, x9, [c1]
        ldp x10, x11, [c1]
        ldp x12, x13, [c1]
        ldp x14, x15, [c1]
        ldp x9, x10, [c1]
        ldp x11, x12, [c1]
        ldp x13, x14, [c1]
        ldp x15, x9, [c1]
        ldp x10, x11, [c1]
        ldp x12, x13, [c1]
    b ldp_throughput_loop

    ldpt_end:
        ret

// extern void ldp_cap_throughput_loop(long iterations, int* tmp);
ldp_cap_throughput_loop:
    cbz x0, ldpcpt_end
        sub x0, x0, #1
        ldp c10, c11, [c1]
        ldp c12, c13, [c1]
        ldp c14, c15, [c1]
        ldp c9, c10, [c1]
        ldp c11, c12, [c1]
        ldp c13, c14, [c1]
        ldp c15, c9, [c1]
        ldp c10, c11, [c1]
        ldp c12, c13, [c1]
        ldp c14, c15, [c1]
        ldp c9, c10, [c1]
        ldp c11, c12, [c1]
        ldp c13, c14, [c1]
        ldp c15, c9, [c1]
        ldp c10, c11, [c1]
        ldp c12, c13, [c1]
    b ldp_cap_throughput_loop

    ldpcpt_end:
        ret

ldr_throughput_loop:
    cbz x0, ldrt_end
        sub x0, x0, #1
        ldr x10, [c1]
        ldr x13, [c1]
        ldr x14, [c1]
        ldr x9, [c1]
        ldr x12, [c1]
        ldr x11, [c1]
        ldr x15, [c1]
        ldr x9, [c1]
        ldr x12, [c1]
        ldr x11, [c1]
        ldr x13, [c1]
        ldr x10, [c1]
        ldr x14, [c1]
        ldr x15, [c1]
        ldr x9, [c1]
        ldr x10, [c1]
    b ldr_throughput_loop

    ldrt_end:
        ret

ldr_latency_loop:
    mov x9, #0
    ldrl_loop:
    cbz x0, ldrl_end
        sub x0, x0, #1
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
        ldr x9, [c1, x9]
    b ldrl_loop
    ldrl_end:
        ret

str_throughput_loop:
    mov x9, #42
    strt_loop:
    cbz x0, strt_end
        sub x0, x0, #1
        str x9, [c1, #0]
        str x9, [c1, #8]
        str x9, [c1, #16]
        str x9, [c1, #24]
        str x9, [c1, #32]
        str x9, [c1, #40]
        str x9, [c1, #48]
        str x9, [c1, #56]
        str x9, [c1, #64]
        str x9, [c1, #72]
        str x9, [c1, #80]
        str x9, [c1, #88]
        str x9, [c1, #96]
        str x9, [c1, #104]
        str x9, [c1, #112]
        str x9, [c1, #120]
            b strt_loop

    strt_end:
        ret

str_inplace_throughput_loop:
    mov x9, #42
    strit_loop:
    cbz x0, strit_end
        sub x0, x0, #1
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
        str x9, [c1]
            b strit_loop

    strit_end:
        ret

str_cap_throughput_loop:
    cbz x0, strct_end
        sub x0, x0, #1
        str c1, [c1, #0]
        str c1, [c1, #16]
        str c1, [c1, #32]
        str c1, [c1, #48]
        str c1, [c1, #64]
        str c1, [c1, #80]
        str c1, [c1, #96]
        str c1, [c1, #112]
        str c1, [c1, #0]
        str c1, [c1, #16]
        str c1, [c1, #32]
        str c1, [c1, #48]
        str c1, [c1, #64]
        str c1, [c1, #80]
        str c1, [c1, #96]
        str c1, [c1, #112]
            b str_cap_throughput_loop

    strct_end:
        ret

str_cap_inplace_throughput_loop:
    cbz x0, strcit_end
        sub x0, x0, #1
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
        str c1, [c1]
            b str_cap_inplace_throughput_loop

    strcit_end:
        ret


ldar_throughput_loop:
    cbz x0, ldart_end
        sub x0, x0, #1
        ldar x10, [c1]
        ldar x13, [c1]
        ldar x14, [c1]
        ldar x9, [c1]
        ldar x12, [c1]
        ldar x11, [c1]
        ldar x15, [c1]
        ldar x9, [c1]
        ldar x12, [c1]
        ldar x11, [c1]
        ldar x13, [c1]
        ldar x10, [c1]
        ldar x14, [c1]
        ldar x15, [c1]
        ldar x9, [c1]
        ldar x10, [c1]
    b ldar_throughput_loop

    ldart_end:
        ret

// extern void stlr_throughput_loop(long iterations, int* tmp);
stlr_throughput_loop:
    mov x9, #42
    stlrt_loop:
    cbz x0, stlrt_end
        sub x0, x0, #1
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
        stlr x9, [c1]
            b stlrt_loop

    stlrt_end:
        ret
