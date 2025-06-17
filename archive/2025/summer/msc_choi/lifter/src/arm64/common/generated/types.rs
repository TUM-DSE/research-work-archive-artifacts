#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{common, decode};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use strum_macros::EnumCount;
#[derive(Debug)]
pub enum Instruction {
    NOP,
    UNPRED,
    UNALLOC,
    UNDEF,
    ABS_Z_P_Z__(Box<ABS_Z_P_Z___operands>),
    ADDPL_R_RI__(Box<ADDPL_R_RI___operands>),
    ADDVL_R_RI__(Box<ADDVL_R_RI___operands>),
    ADD_Z_P_ZZ__(Box<ADD_Z_P_ZZ___operands>),
    ADD_Z_ZI__(Box<ADD_Z_ZI___operands>),
    ADD_Z_ZZ__(Box<ADD_Z_ZZ___operands>),
    ADR_Z_AZ_SD_same_scaled(Box<ADR_Z_AZ_SD_same_scaled_operands>),
    ANDV_R_P_Z__(Box<ANDV_R_P_Z___operands>),
    AND_P_P_PP_Z(Box<AND_P_P_PP_Z_operands>),
    AND_Z_P_ZZ__(Box<AND_Z_P_ZZ___operands>),
    AND_Z_ZI__(Box<AND_Z_ZI___operands>),
    AND_Z_ZZ__(Box<AND_Z_ZZ___operands>),
    ASRD_Z_P_ZI__(Box<ASRD_Z_P_ZI___operands>),
    ASRR_Z_P_ZZ__(Box<ASRR_Z_P_ZZ___operands>),
    ASR_Z_P_ZI__(Box<ASR_Z_P_ZI___operands>),
    ASR_Z_P_ZW__(Box<ASR_Z_P_ZW___operands>),
    ASR_Z_P_ZZ__(Box<ASR_Z_P_ZZ___operands>),
    ASR_Z_ZI__(Box<ASR_Z_ZI___operands>),
    ASR_Z_ZW__(Box<ASR_Z_ZW___operands>),
    BFCVTNT_Z_P_Z_S2BF(Box<BFCVTNT_Z_P_Z_S2BF_operands>),
    BFCVT_Z_P_Z_S2BF(Box<BFCVT_Z_P_Z_S2BF_operands>),
    BFDOT_Z_ZZZ__(Box<BFDOT_Z_ZZZ___operands>),
    BFDOT_Z_ZZZi__(Box<BFDOT_Z_ZZZi___operands>),
    BFMLALB_Z_ZZZ__(Box<BFMLALB_Z_ZZZ___operands>),
    BFMLALB_Z_ZZZi__(Box<BFMLALB_Z_ZZZi___operands>),
    BFMLALT_Z_ZZZ__(Box<BFMLALT_Z_ZZZ___operands>),
    BFMLALT_Z_ZZZi__(Box<BFMLALT_Z_ZZZi___operands>),
    BFMMLA_Z_ZZZ__(Box<BFMMLA_Z_ZZZ___operands>),
    BIC_P_P_PP_Z(Box<BIC_P_P_PP_Z_operands>),
    BIC_Z_P_ZZ__(Box<BIC_Z_P_ZZ___operands>),
    BIC_Z_ZZ__(Box<BIC_Z_ZZ___operands>),
    BRKA_P_P_P__(Box<BRKA_P_P_P___operands>),
    BRKB_P_P_P__(Box<BRKB_P_P_P___operands>),
    BRKN_P_P_PP__(Box<BRKN_P_P_PP___operands>),
    BRKPA_P_P_PP__(Box<BRKPA_P_P_PP___operands>),
    BRKPB_P_P_PP__(Box<BRKPB_P_P_PP___operands>),
    CLASTA_R_P_Z__(Box<CLASTA_R_P_Z___operands>),
    CLASTA_V_P_Z__(Box<CLASTA_V_P_Z___operands>),
    CLASTA_Z_P_ZZ__(Box<CLASTA_Z_P_ZZ___operands>),
    CLASTB_R_P_Z__(Box<CLASTB_R_P_Z___operands>),
    CLASTB_V_P_Z__(Box<CLASTB_V_P_Z___operands>),
    CLASTB_Z_P_ZZ__(Box<CLASTB_Z_P_ZZ___operands>),
    CLS_Z_P_Z__(Box<CLS_Z_P_Z___operands>),
    CLZ_Z_P_Z__(Box<CLZ_Z_P_Z___operands>),
    CMPEQ_P_P_ZI__(Box<CMPEQ_P_P_ZI___operands>),
    CMPEQ_P_P_ZW__(Box<CMPEQ_P_P_ZW___operands>),
    CMPEQ_P_P_ZZ__(Box<CMPEQ_P_P_ZZ___operands>),
    CNOT_Z_P_Z__(Box<CNOT_Z_P_Z___operands>),
    CNTB_R_S__(Box<CNTB_R_S___operands>),
    CNTP_R_P_P__(Box<CNTP_R_P_P___operands>),
    CNT_Z_P_Z__(Box<CNT_Z_P_Z___operands>),
    COMPACT_Z_P_Z__(Box<COMPACT_Z_P_Z___operands>),
    CPY_Z_O_I__(Box<CPY_Z_O_I___operands>),
    CPY_Z_P_I__(Box<CPY_Z_P_I___operands>),
    CPY_Z_P_R__(Box<CPY_Z_P_R___operands>),
    CPY_Z_P_V__(Box<CPY_Z_P_V___operands>),
    CTERMEQ_RR__(Box<CTERMEQ_RR___operands>),
    DECB_R_RS__(Box<DECB_R_RS___operands>),
    DECD_Z_ZS__(Box<DECD_Z_ZS___operands>),
    DECP_R_P_R__(Box<DECP_R_P_R___operands>),
    DECP_Z_P_Z__(Box<DECP_Z_P_Z___operands>),
    DUPM_Z_I__(Box<DUPM_Z_I___operands>),
    DUP_Z_I__(Box<DUP_Z_I___operands>),
    DUP_Z_R__(Box<DUP_Z_R___operands>),
    DUP_Z_Zi__(Box<DUP_Z_Zi___operands>),
    EORV_R_P_Z__(Box<EORV_R_P_Z___operands>),
    EOR_P_P_PP_Z(Box<EOR_P_P_PP_Z_operands>),
    EOR_Z_P_ZZ__(Box<EOR_Z_P_ZZ___operands>),
    EOR_Z_ZI__(Box<EOR_Z_ZI___operands>),
    EOR_Z_ZZ__(Box<EOR_Z_ZZ___operands>),
    EXT_Z_ZI_Des(Box<EXT_Z_ZI_Des_operands>),
    FABD_Z_P_ZZ__(Box<FABD_Z_P_ZZ___operands>),
    FABS_Z_P_Z__(Box<FABS_Z_P_Z___operands>),
    FACGT_P_P_ZZ__(Box<FACGT_P_P_ZZ___operands>),
    FADDA_V_P_Z__(Box<FADDA_V_P_Z___operands>),
    FADDV_V_P_Z__(Box<FADDV_V_P_Z___operands>),
    FADD_Z_P_ZS__(Box<FADD_Z_P_ZS___operands>),
    FADD_Z_P_ZZ__(Box<FADD_Z_P_ZZ___operands>),
    FADD_Z_ZZ__(Box<FADD_Z_ZZ___operands>),
    FCADD_Z_P_ZZ__(Box<FCADD_Z_P_ZZ___operands>),
    FCMEQ_P_P_Z0__(Box<FCMEQ_P_P_Z0___operands>),
    FCMEQ_P_P_ZZ__(Box<FCMEQ_P_P_ZZ___operands>),
    FCMLA_Z_P_ZZZ__(Box<FCMLA_Z_P_ZZZ___operands>),
    FCMLA_Z_ZZZi_H(Box<FCMLA_Z_ZZZi_H_operands>),
    FCPY_Z_P_I__(Box<FCPY_Z_P_I___operands>),
    FCVTZS_Z_P_Z_FP162H(Box<FCVTZS_Z_P_Z_FP162H_operands>),
    FCVTZU_Z_P_Z_FP162H(Box<FCVTZU_Z_P_Z_FP162H_operands>),
    FCVT_Z_P_Z_H2S(Box<FCVT_Z_P_Z_H2S_operands>),
    FDIVR_Z_P_ZZ__(Box<FDIVR_Z_P_ZZ___operands>),
    FDIV_Z_P_ZZ__(Box<FDIV_Z_P_ZZ___operands>),
    FDUP_Z_I__(Box<FDUP_Z_I___operands>),
    FEXPA_Z_Z__(Box<FEXPA_Z_Z___operands>),
    FMAD_Z_P_ZZZ__(Box<FMAD_Z_P_ZZZ___operands>),
    FMAXNMV_V_P_Z__(Box<FMAXNMV_V_P_Z___operands>),
    FMAXNM_Z_P_ZS__(Box<FMAXNM_Z_P_ZS___operands>),
    FMAXNM_Z_P_ZZ__(Box<FMAXNM_Z_P_ZZ___operands>),
    FMAXV_V_P_Z__(Box<FMAXV_V_P_Z___operands>),
    FMAX_Z_P_ZS__(Box<FMAX_Z_P_ZS___operands>),
    FMAX_Z_P_ZZ__(Box<FMAX_Z_P_ZZ___operands>),
    FMINNMV_V_P_Z__(Box<FMINNMV_V_P_Z___operands>),
    FMINNM_Z_P_ZS__(Box<FMINNM_Z_P_ZS___operands>),
    FMINNM_Z_P_ZZ__(Box<FMINNM_Z_P_ZZ___operands>),
    FMINV_V_P_Z__(Box<FMINV_V_P_Z___operands>),
    FMIN_Z_P_ZS__(Box<FMIN_Z_P_ZS___operands>),
    FMIN_Z_P_ZZ__(Box<FMIN_Z_P_ZZ___operands>),
    FMLA_Z_P_ZZZ__(Box<FMLA_Z_P_ZZZ___operands>),
    FMLA_Z_ZZZi_H(Box<FMLA_Z_ZZZi_H_operands>),
    FMLS_Z_P_ZZZ__(Box<FMLS_Z_P_ZZZ___operands>),
    FMLS_Z_ZZZi_H(Box<FMLS_Z_ZZZi_H_operands>),
    FMMLA_Z_ZZZ_S(Box<FMMLA_Z_ZZZ_S_operands>),
    FMSB_Z_P_ZZZ__(Box<FMSB_Z_P_ZZZ___operands>),
    FMULX_Z_P_ZZ__(Box<FMULX_Z_P_ZZ___operands>),
    FMUL_Z_P_ZS__(Box<FMUL_Z_P_ZS___operands>),
    FMUL_Z_P_ZZ__(Box<FMUL_Z_P_ZZ___operands>),
    FMUL_Z_ZZ__(Box<FMUL_Z_ZZ___operands>),
    FMUL_Z_ZZi_H(Box<FMUL_Z_ZZi_H_operands>),
    FNEG_Z_P_Z__(Box<FNEG_Z_P_Z___operands>),
    FNMAD_Z_P_ZZZ__(Box<FNMAD_Z_P_ZZZ___operands>),
    FNMLA_Z_P_ZZZ__(Box<FNMLA_Z_P_ZZZ___operands>),
    FNMLS_Z_P_ZZZ__(Box<FNMLS_Z_P_ZZZ___operands>),
    FNMSB_Z_P_ZZZ__(Box<FNMSB_Z_P_ZZZ___operands>),
    FRECPE_Z_Z__(Box<FRECPE_Z_Z___operands>),
    FRECPS_Z_ZZ__(Box<FRECPS_Z_ZZ___operands>),
    FRECPX_Z_P_Z__(Box<FRECPX_Z_P_Z___operands>),
    FRINTI_Z_P_Z__(Box<FRINTI_Z_P_Z___operands>),
    FRSQRTE_Z_Z__(Box<FRSQRTE_Z_Z___operands>),
    FRSQRTS_Z_ZZ__(Box<FRSQRTS_Z_ZZ___operands>),
    FSCALE_Z_P_ZZ__(Box<FSCALE_Z_P_ZZ___operands>),
    FSQRT_Z_P_Z__(Box<FSQRT_Z_P_Z___operands>),
    FSUBR_Z_P_ZS__(Box<FSUBR_Z_P_ZS___operands>),
    FSUBR_Z_P_ZZ__(Box<FSUBR_Z_P_ZZ___operands>),
    FSUB_Z_P_ZS__(Box<FSUB_Z_P_ZS___operands>),
    FSUB_Z_P_ZZ__(Box<FSUB_Z_P_ZZ___operands>),
    FSUB_Z_ZZ__(Box<FSUB_Z_ZZ___operands>),
    FTMAD_Z_ZZI__(Box<FTMAD_Z_ZZI___operands>),
    FTSMUL_Z_ZZ__(Box<FTSMUL_Z_ZZ___operands>),
    FTSSEL_Z_ZZ__(Box<FTSSEL_Z_ZZ___operands>),
    INCB_R_RS__(Box<INCB_R_RS___operands>),
    INCD_Z_ZS__(Box<INCD_Z_ZS___operands>),
    INCP_R_P_R__(Box<INCP_R_P_R___operands>),
    INCP_Z_P_Z__(Box<INCP_Z_P_Z___operands>),
    INDEX_Z_II__(Box<INDEX_Z_II___operands>),
    INDEX_Z_IR__(Box<INDEX_Z_IR___operands>),
    INDEX_Z_RI__(Box<INDEX_Z_RI___operands>),
    INDEX_Z_RR__(Box<INDEX_Z_RR___operands>),
    INSR_Z_R__(Box<INSR_Z_R___operands>),
    INSR_Z_V__(Box<INSR_Z_V___operands>),
    LASTA_R_P_Z__(Box<LASTA_R_P_Z___operands>),
    LASTA_V_P_Z__(Box<LASTA_V_P_Z___operands>),
    LASTB_R_P_Z__(Box<LASTB_R_P_Z___operands>),
    LASTB_V_P_Z__(Box<LASTB_V_P_Z___operands>),
    LD1B_Z_P_AI_S(Box<LD1B_Z_P_AI_S_operands>),
    LD1B_Z_P_BI_U8(Box<LD1B_Z_P_BI_U8_operands>),
    LD1B_Z_P_BR_U8(Box<LD1B_Z_P_BR_U8_operands>),
    LD1B_Z_P_BZ_D_x32_unscaled(Box<LD1B_Z_P_BZ_D_x32_unscaled_operands>),
    LD1D_Z_P_AI_D(Box<LD1D_Z_P_AI_D_operands>),
    LD1D_Z_P_BI_U64(Box<LD1D_Z_P_BI_U64_operands>),
    LD1D_Z_P_BR_U64(Box<LD1D_Z_P_BR_U64_operands>),
    LD1D_Z_P_BZ_D_x32_scaled(Box<LD1D_Z_P_BZ_D_x32_scaled_operands>),
    LD1H_Z_P_AI_S(Box<LD1H_Z_P_AI_S_operands>),
    LD1H_Z_P_BI_U16(Box<LD1H_Z_P_BI_U16_operands>),
    LD1H_Z_P_BR_U16(Box<LD1H_Z_P_BR_U16_operands>),
    LD1H_Z_P_BZ_S_x32_scaled(Box<LD1H_Z_P_BZ_S_x32_scaled_operands>),
    LD1RB_Z_P_BI_U8(Box<LD1RB_Z_P_BI_U8_operands>),
    LD1RD_Z_P_BI_U64(Box<LD1RD_Z_P_BI_U64_operands>),
    LD1RH_Z_P_BI_U16(Box<LD1RH_Z_P_BI_U16_operands>),
    LD1ROB_Z_P_BI_U8(Box<LD1ROB_Z_P_BI_U8_operands>),
    LD1ROB_Z_P_BR_Contiguous(Box<LD1ROB_Z_P_BR_Contiguous_operands>),
    LD1ROD_Z_P_BI_U64(Box<LD1ROD_Z_P_BI_U64_operands>),
    LD1ROD_Z_P_BR_Contiguous(Box<LD1ROD_Z_P_BR_Contiguous_operands>),
    LD1ROH_Z_P_BI_U16(Box<LD1ROH_Z_P_BI_U16_operands>),
    LD1ROH_Z_P_BR_Contiguous(Box<LD1ROH_Z_P_BR_Contiguous_operands>),
    LD1ROW_Z_P_BI_U32(Box<LD1ROW_Z_P_BI_U32_operands>),
    LD1ROW_Z_P_BR_Contiguous(Box<LD1ROW_Z_P_BR_Contiguous_operands>),
    LD1RQB_Z_P_BI_U8(Box<LD1RQB_Z_P_BI_U8_operands>),
    LD1RQB_Z_P_BR_Contiguous(Box<LD1RQB_Z_P_BR_Contiguous_operands>),
    LD1RQD_Z_P_BI_U64(Box<LD1RQD_Z_P_BI_U64_operands>),
    LD1RQD_Z_P_BR_Contiguous(Box<LD1RQD_Z_P_BR_Contiguous_operands>),
    LD1RQH_Z_P_BI_U16(Box<LD1RQH_Z_P_BI_U16_operands>),
    LD1RQH_Z_P_BR_Contiguous(Box<LD1RQH_Z_P_BR_Contiguous_operands>),
    LD1RQW_Z_P_BI_U32(Box<LD1RQW_Z_P_BI_U32_operands>),
    LD1RQW_Z_P_BR_Contiguous(Box<LD1RQW_Z_P_BR_Contiguous_operands>),
    LD1RSB_Z_P_BI_S16(Box<LD1RSB_Z_P_BI_S16_operands>),
    LD1RSH_Z_P_BI_S32(Box<LD1RSH_Z_P_BI_S32_operands>),
    LD1RSW_Z_P_BI_S64(Box<LD1RSW_Z_P_BI_S64_operands>),
    LD1RW_Z_P_BI_U32(Box<LD1RW_Z_P_BI_U32_operands>),
    LD1SB_Z_P_AI_S(Box<LD1SB_Z_P_AI_S_operands>),
    LD1SB_Z_P_BI_S16(Box<LD1SB_Z_P_BI_S16_operands>),
    LD1SB_Z_P_BR_S16(Box<LD1SB_Z_P_BR_S16_operands>),
    LD1SB_Z_P_BZ_D_x32_unscaled(Box<LD1SB_Z_P_BZ_D_x32_unscaled_operands>),
    LD1SH_Z_P_AI_S(Box<LD1SH_Z_P_AI_S_operands>),
    LD1SH_Z_P_BI_S32(Box<LD1SH_Z_P_BI_S32_operands>),
    LD1SH_Z_P_BR_S32(Box<LD1SH_Z_P_BR_S32_operands>),
    LD1SH_Z_P_BZ_S_x32_scaled(Box<LD1SH_Z_P_BZ_S_x32_scaled_operands>),
    LD1SW_Z_P_AI_D(Box<LD1SW_Z_P_AI_D_operands>),
    LD1SW_Z_P_BI_S64(Box<LD1SW_Z_P_BI_S64_operands>),
    LD1SW_Z_P_BR_S64(Box<LD1SW_Z_P_BR_S64_operands>),
    LD1SW_Z_P_BZ_D_x32_scaled(Box<LD1SW_Z_P_BZ_D_x32_scaled_operands>),
    LD1W_Z_P_AI_S(Box<LD1W_Z_P_AI_S_operands>),
    LD1W_Z_P_BI_U32(Box<LD1W_Z_P_BI_U32_operands>),
    LD1W_Z_P_BR_U32(Box<LD1W_Z_P_BR_U32_operands>),
    LD1W_Z_P_BZ_S_x32_scaled(Box<LD1W_Z_P_BZ_S_x32_scaled_operands>),
    LD2B_Z_P_BI_Contiguous(Box<LD2B_Z_P_BI_Contiguous_operands>),
    LD2B_Z_P_BR_Contiguous(Box<LD2B_Z_P_BR_Contiguous_operands>),
    LD2D_Z_P_BI_Contiguous(Box<LD2D_Z_P_BI_Contiguous_operands>),
    LD2D_Z_P_BR_Contiguous(Box<LD2D_Z_P_BR_Contiguous_operands>),
    LD2H_Z_P_BI_Contiguous(Box<LD2H_Z_P_BI_Contiguous_operands>),
    LD2H_Z_P_BR_Contiguous(Box<LD2H_Z_P_BR_Contiguous_operands>),
    LD2W_Z_P_BI_Contiguous(Box<LD2W_Z_P_BI_Contiguous_operands>),
    LD2W_Z_P_BR_Contiguous(Box<LD2W_Z_P_BR_Contiguous_operands>),
    LD3B_Z_P_BI_Contiguous(Box<LD3B_Z_P_BI_Contiguous_operands>),
    LD3B_Z_P_BR_Contiguous(Box<LD3B_Z_P_BR_Contiguous_operands>),
    LD3D_Z_P_BI_Contiguous(Box<LD3D_Z_P_BI_Contiguous_operands>),
    LD3D_Z_P_BR_Contiguous(Box<LD3D_Z_P_BR_Contiguous_operands>),
    LD3H_Z_P_BI_Contiguous(Box<LD3H_Z_P_BI_Contiguous_operands>),
    LD3H_Z_P_BR_Contiguous(Box<LD3H_Z_P_BR_Contiguous_operands>),
    LD3W_Z_P_BI_Contiguous(Box<LD3W_Z_P_BI_Contiguous_operands>),
    LD3W_Z_P_BR_Contiguous(Box<LD3W_Z_P_BR_Contiguous_operands>),
    LD4B_Z_P_BI_Contiguous(Box<LD4B_Z_P_BI_Contiguous_operands>),
    LD4B_Z_P_BR_Contiguous(Box<LD4B_Z_P_BR_Contiguous_operands>),
    LD4D_Z_P_BI_Contiguous(Box<LD4D_Z_P_BI_Contiguous_operands>),
    LD4D_Z_P_BR_Contiguous(Box<LD4D_Z_P_BR_Contiguous_operands>),
    LD4H_Z_P_BI_Contiguous(Box<LD4H_Z_P_BI_Contiguous_operands>),
    LD4H_Z_P_BR_Contiguous(Box<LD4H_Z_P_BR_Contiguous_operands>),
    LD4W_Z_P_BI_Contiguous(Box<LD4W_Z_P_BI_Contiguous_operands>),
    LD4W_Z_P_BR_Contiguous(Box<LD4W_Z_P_BR_Contiguous_operands>),
    LDFF1B_Z_P_AI_S(Box<LDFF1B_Z_P_AI_S_operands>),
    LDFF1B_Z_P_BR_U8(Box<LDFF1B_Z_P_BR_U8_operands>),
    LDFF1B_Z_P_BZ_D_x32_unscaled(Box<LDFF1B_Z_P_BZ_D_x32_unscaled_operands>),
    LDFF1D_Z_P_AI_D(Box<LDFF1D_Z_P_AI_D_operands>),
    LDFF1D_Z_P_BR_U64(Box<LDFF1D_Z_P_BR_U64_operands>),
    LDFF1D_Z_P_BZ_D_x32_scaled(Box<LDFF1D_Z_P_BZ_D_x32_scaled_operands>),
    LDFF1H_Z_P_AI_S(Box<LDFF1H_Z_P_AI_S_operands>),
    LDFF1H_Z_P_BR_U16(Box<LDFF1H_Z_P_BR_U16_operands>),
    LDFF1H_Z_P_BZ_S_x32_scaled(Box<LDFF1H_Z_P_BZ_S_x32_scaled_operands>),
    LDFF1SB_Z_P_AI_S(Box<LDFF1SB_Z_P_AI_S_operands>),
    LDFF1SB_Z_P_BR_S16(Box<LDFF1SB_Z_P_BR_S16_operands>),
    LDFF1SB_Z_P_BZ_D_x32_unscaled(Box<LDFF1SB_Z_P_BZ_D_x32_unscaled_operands>),
    LDFF1SH_Z_P_AI_S(Box<LDFF1SH_Z_P_AI_S_operands>),
    LDFF1SH_Z_P_BR_S32(Box<LDFF1SH_Z_P_BR_S32_operands>),
    LDFF1SH_Z_P_BZ_S_x32_scaled(Box<LDFF1SH_Z_P_BZ_S_x32_scaled_operands>),
    LDFF1SW_Z_P_AI_D(Box<LDFF1SW_Z_P_AI_D_operands>),
    LDFF1SW_Z_P_BR_S64(Box<LDFF1SW_Z_P_BR_S64_operands>),
    LDFF1SW_Z_P_BZ_D_x32_scaled(Box<LDFF1SW_Z_P_BZ_D_x32_scaled_operands>),
    LDFF1W_Z_P_AI_S(Box<LDFF1W_Z_P_AI_S_operands>),
    LDFF1W_Z_P_BR_U32(Box<LDFF1W_Z_P_BR_U32_operands>),
    LDFF1W_Z_P_BZ_S_x32_scaled(Box<LDFF1W_Z_P_BZ_S_x32_scaled_operands>),
    LDNF1B_Z_P_BI_U8(Box<LDNF1B_Z_P_BI_U8_operands>),
    LDNF1D_Z_P_BI_U64(Box<LDNF1D_Z_P_BI_U64_operands>),
    LDNF1H_Z_P_BI_U16(Box<LDNF1H_Z_P_BI_U16_operands>),
    LDNF1SB_Z_P_BI_S16(Box<LDNF1SB_Z_P_BI_S16_operands>),
    LDNF1SH_Z_P_BI_S32(Box<LDNF1SH_Z_P_BI_S32_operands>),
    LDNF1SW_Z_P_BI_S64(Box<LDNF1SW_Z_P_BI_S64_operands>),
    LDNF1W_Z_P_BI_U32(Box<LDNF1W_Z_P_BI_U32_operands>),
    LDNT1B_Z_P_BI_Contiguous(Box<LDNT1B_Z_P_BI_Contiguous_operands>),
    LDNT1B_Z_P_BR_Contiguous(Box<LDNT1B_Z_P_BR_Contiguous_operands>),
    LDNT1D_Z_P_BI_Contiguous(Box<LDNT1D_Z_P_BI_Contiguous_operands>),
    LDNT1D_Z_P_BR_Contiguous(Box<LDNT1D_Z_P_BR_Contiguous_operands>),
    LDNT1H_Z_P_BI_Contiguous(Box<LDNT1H_Z_P_BI_Contiguous_operands>),
    LDNT1H_Z_P_BR_Contiguous(Box<LDNT1H_Z_P_BR_Contiguous_operands>),
    LDNT1W_Z_P_BI_Contiguous(Box<LDNT1W_Z_P_BI_Contiguous_operands>),
    LDNT1W_Z_P_BR_Contiguous(Box<LDNT1W_Z_P_BR_Contiguous_operands>),
    LDR_P_BI__(Box<LDR_P_BI___operands>),
    LDR_Z_BI__(Box<LDR_Z_BI___operands>),
    LSLR_Z_P_ZZ__(Box<LSLR_Z_P_ZZ___operands>),
    LSL_Z_P_ZI__(Box<LSL_Z_P_ZI___operands>),
    LSL_Z_P_ZW__(Box<LSL_Z_P_ZW___operands>),
    LSL_Z_P_ZZ__(Box<LSL_Z_P_ZZ___operands>),
    LSL_Z_ZI__(Box<LSL_Z_ZI___operands>),
    LSL_Z_ZW__(Box<LSL_Z_ZW___operands>),
    LSRR_Z_P_ZZ__(Box<LSRR_Z_P_ZZ___operands>),
    LSR_Z_P_ZI__(Box<LSR_Z_P_ZI___operands>),
    LSR_Z_P_ZW__(Box<LSR_Z_P_ZW___operands>),
    LSR_Z_P_ZZ__(Box<LSR_Z_P_ZZ___operands>),
    LSR_Z_ZI__(Box<LSR_Z_ZI___operands>),
    LSR_Z_ZW__(Box<LSR_Z_ZW___operands>),
    MAD_Z_P_ZZZ__(Box<MAD_Z_P_ZZZ___operands>),
    MLA_Z_P_ZZZ__(Box<MLA_Z_P_ZZZ___operands>),
    MLS_Z_P_ZZZ__(Box<MLS_Z_P_ZZZ___operands>),
    MOVPRFX_Z_P_Z__(Box<MOVPRFX_Z_P_Z___operands>),
    MOVPRFX_Z_Z__(Box<MOVPRFX_Z_Z___operands>),
    MSB_Z_P_ZZZ__(Box<MSB_Z_P_ZZZ___operands>),
    MUL_Z_P_ZZ__(Box<MUL_Z_P_ZZ___operands>),
    MUL_Z_ZI__(Box<MUL_Z_ZI___operands>),
    NAND_P_P_PP_Z(Box<NAND_P_P_PP_Z_operands>),
    NEG_Z_P_Z__(Box<NEG_Z_P_Z___operands>),
    NOR_P_P_PP_Z(Box<NOR_P_P_PP_Z_operands>),
    NOT_Z_P_Z__(Box<NOT_Z_P_Z___operands>),
    ORN_P_P_PP_Z(Box<ORN_P_P_PP_Z_operands>),
    ORR_P_P_PP_Z(Box<ORR_P_P_PP_Z_operands>),
    ORR_Z_P_ZZ__(Box<ORR_Z_P_ZZ___operands>),
    ORR_Z_ZI__(Box<ORR_Z_ZI___operands>),
    ORR_Z_ZZ__(Box<ORR_Z_ZZ___operands>),
    ORV_R_P_Z__(Box<ORV_R_P_Z___operands>),
    PFALSE_P__(Box<PFALSE_P___operands>),
    PFIRST_P_P_P__(Box<PFIRST_P_P_P___operands>),
    PNEXT_P_P_P__(Box<PNEXT_P_P_P___operands>),
    PRFB_I_P_AI_S(Box<PRFB_I_P_AI_S_operands>),
    PRFB_I_P_BI_S(Box<PRFB_I_P_BI_S_operands>),
    PRFB_I_P_BR_S(Box<PRFB_I_P_BR_S_operands>),
    PRFB_I_P_BZ_S_x32_scaled(Box<PRFB_I_P_BZ_S_x32_scaled_operands>),
    PRFD_I_P_AI_S(Box<PRFD_I_P_AI_S_operands>),
    PRFD_I_P_BI_S(Box<PRFD_I_P_BI_S_operands>),
    PRFD_I_P_BR_S(Box<PRFD_I_P_BR_S_operands>),
    PRFD_I_P_BZ_S_x32_scaled(Box<PRFD_I_P_BZ_S_x32_scaled_operands>),
    PRFH_I_P_AI_S(Box<PRFH_I_P_AI_S_operands>),
    PRFH_I_P_BI_S(Box<PRFH_I_P_BI_S_operands>),
    PRFH_I_P_BR_S(Box<PRFH_I_P_BR_S_operands>),
    PRFH_I_P_BZ_S_x32_scaled(Box<PRFH_I_P_BZ_S_x32_scaled_operands>),
    PRFW_I_P_AI_S(Box<PRFW_I_P_AI_S_operands>),
    PRFW_I_P_BI_S(Box<PRFW_I_P_BI_S_operands>),
    PRFW_I_P_BR_S(Box<PRFW_I_P_BR_S_operands>),
    PRFW_I_P_BZ_S_x32_scaled(Box<PRFW_I_P_BZ_S_x32_scaled_operands>),
    PTEST__P_P__(Box<PTEST__P_P___operands>),
    PTRUE_P_S__(Box<PTRUE_P_S___operands>),
    PUNPKHI_P_P__(Box<PUNPKHI_P_P___operands>),
    RBIT_Z_P_Z__(Box<RBIT_Z_P_Z___operands>),
    RDFFR_P_F__(Box<RDFFR_P_F___operands>),
    RDFFR_P_P_F__(Box<RDFFR_P_P_F___operands>),
    RDVL_R_I__(Box<RDVL_R_I___operands>),
    REVB_Z_Z__(Box<REVB_Z_Z___operands>),
    REV_P_P__(Box<REV_P_P___operands>),
    REV_Z_Z__(Box<REV_Z_Z___operands>),
    SABD_Z_P_ZZ__(Box<SABD_Z_P_ZZ___operands>),
    SADDV_R_P_Z__(Box<SADDV_R_P_Z___operands>),
    SCVTF_Z_P_Z_H2FP16(Box<SCVTF_Z_P_Z_H2FP16_operands>),
    SDIVR_Z_P_ZZ__(Box<SDIVR_Z_P_ZZ___operands>),
    SDIV_Z_P_ZZ__(Box<SDIV_Z_P_ZZ___operands>),
    SDOT_Z_ZZZ__(Box<SDOT_Z_ZZZ___operands>),
    SDOT_Z_ZZZi_S(Box<SDOT_Z_ZZZi_S_operands>),
    SEL_P_P_PP__(Box<SEL_P_P_PP___operands>),
    SEL_Z_P_ZZ__(Box<SEL_Z_P_ZZ___operands>),
    SETFFR_F__(Box<SETFFR_F___operands>),
    SMAXV_R_P_Z__(Box<SMAXV_R_P_Z___operands>),
    SMAX_Z_P_ZZ__(Box<SMAX_Z_P_ZZ___operands>),
    SMAX_Z_ZI__(Box<SMAX_Z_ZI___operands>),
    SMINV_R_P_Z__(Box<SMINV_R_P_Z___operands>),
    SMIN_Z_P_ZZ__(Box<SMIN_Z_P_ZZ___operands>),
    SMIN_Z_ZI__(Box<SMIN_Z_ZI___operands>),
    SMMLA_Z_ZZZ__(Box<SMMLA_Z_ZZZ___operands>),
    SMULH_Z_P_ZZ__(Box<SMULH_Z_P_ZZ___operands>),
    SPLICE_Z_P_ZZ_Des(Box<SPLICE_Z_P_ZZ_Des_operands>),
    SQADD_Z_ZI__(Box<SQADD_Z_ZI___operands>),
    SQADD_Z_ZZ__(Box<SQADD_Z_ZZ___operands>),
    SQDECB_R_RS_SX(Box<SQDECB_R_RS_SX_operands>),
    SQDECD_R_RS_SX(Box<SQDECD_R_RS_SX_operands>),
    SQDECD_Z_ZS__(Box<SQDECD_Z_ZS___operands>),
    SQDECH_R_RS_SX(Box<SQDECH_R_RS_SX_operands>),
    SQDECH_Z_ZS__(Box<SQDECH_Z_ZS___operands>),
    SQDECP_R_P_R_SX(Box<SQDECP_R_P_R_SX_operands>),
    SQDECP_Z_P_Z__(Box<SQDECP_Z_P_Z___operands>),
    SQDECW_R_RS_SX(Box<SQDECW_R_RS_SX_operands>),
    SQDECW_Z_ZS__(Box<SQDECW_Z_ZS___operands>),
    SQINCB_R_RS_SX(Box<SQINCB_R_RS_SX_operands>),
    SQINCD_R_RS_SX(Box<SQINCD_R_RS_SX_operands>),
    SQINCD_Z_ZS__(Box<SQINCD_Z_ZS___operands>),
    SQINCH_R_RS_SX(Box<SQINCH_R_RS_SX_operands>),
    SQINCH_Z_ZS__(Box<SQINCH_Z_ZS___operands>),
    SQINCP_R_P_R_SX(Box<SQINCP_R_P_R_SX_operands>),
    SQINCP_Z_P_Z__(Box<SQINCP_Z_P_Z___operands>),
    SQINCW_R_RS_SX(Box<SQINCW_R_RS_SX_operands>),
    SQINCW_Z_ZS__(Box<SQINCW_Z_ZS___operands>),
    SQSUB_Z_ZI__(Box<SQSUB_Z_ZI___operands>),
    SQSUB_Z_ZZ__(Box<SQSUB_Z_ZZ___operands>),
    ST1B_Z_P_AI_S(Box<ST1B_Z_P_AI_S_operands>),
    ST1B_Z_P_BI__(Box<ST1B_Z_P_BI___operands>),
    ST1B_Z_P_BR__(Box<ST1B_Z_P_BR___operands>),
    ST1B_Z_P_BZ_D_x32_unscaled(Box<ST1B_Z_P_BZ_D_x32_unscaled_operands>),
    ST1D_Z_P_AI_D(Box<ST1D_Z_P_AI_D_operands>),
    ST1D_Z_P_BI__(Box<ST1D_Z_P_BI___operands>),
    ST1D_Z_P_BR__(Box<ST1D_Z_P_BR___operands>),
    ST1D_Z_P_BZ_D_x32_scaled(Box<ST1D_Z_P_BZ_D_x32_scaled_operands>),
    ST1H_Z_P_AI_S(Box<ST1H_Z_P_AI_S_operands>),
    ST1H_Z_P_BI__(Box<ST1H_Z_P_BI___operands>),
    ST1H_Z_P_BR__(Box<ST1H_Z_P_BR___operands>),
    ST1H_Z_P_BZ_S_x32_scaled(Box<ST1H_Z_P_BZ_S_x32_scaled_operands>),
    ST1W_Z_P_AI_S(Box<ST1W_Z_P_AI_S_operands>),
    ST1W_Z_P_BI__(Box<ST1W_Z_P_BI___operands>),
    ST1W_Z_P_BR__(Box<ST1W_Z_P_BR___operands>),
    ST1W_Z_P_BZ_S_x32_scaled(Box<ST1W_Z_P_BZ_S_x32_scaled_operands>),
    ST2B_Z_P_BI_Contiguous(Box<ST2B_Z_P_BI_Contiguous_operands>),
    ST2B_Z_P_BR_Contiguous(Box<ST2B_Z_P_BR_Contiguous_operands>),
    ST2D_Z_P_BI_Contiguous(Box<ST2D_Z_P_BI_Contiguous_operands>),
    ST2D_Z_P_BR_Contiguous(Box<ST2D_Z_P_BR_Contiguous_operands>),
    ST2H_Z_P_BI_Contiguous(Box<ST2H_Z_P_BI_Contiguous_operands>),
    ST2H_Z_P_BR_Contiguous(Box<ST2H_Z_P_BR_Contiguous_operands>),
    ST2W_Z_P_BI_Contiguous(Box<ST2W_Z_P_BI_Contiguous_operands>),
    ST2W_Z_P_BR_Contiguous(Box<ST2W_Z_P_BR_Contiguous_operands>),
    ST3B_Z_P_BI_Contiguous(Box<ST3B_Z_P_BI_Contiguous_operands>),
    ST3B_Z_P_BR_Contiguous(Box<ST3B_Z_P_BR_Contiguous_operands>),
    ST3D_Z_P_BI_Contiguous(Box<ST3D_Z_P_BI_Contiguous_operands>),
    ST3D_Z_P_BR_Contiguous(Box<ST3D_Z_P_BR_Contiguous_operands>),
    ST3H_Z_P_BI_Contiguous(Box<ST3H_Z_P_BI_Contiguous_operands>),
    ST3H_Z_P_BR_Contiguous(Box<ST3H_Z_P_BR_Contiguous_operands>),
    ST3W_Z_P_BI_Contiguous(Box<ST3W_Z_P_BI_Contiguous_operands>),
    ST3W_Z_P_BR_Contiguous(Box<ST3W_Z_P_BR_Contiguous_operands>),
    ST4B_Z_P_BI_Contiguous(Box<ST4B_Z_P_BI_Contiguous_operands>),
    ST4B_Z_P_BR_Contiguous(Box<ST4B_Z_P_BR_Contiguous_operands>),
    ST4D_Z_P_BI_Contiguous(Box<ST4D_Z_P_BI_Contiguous_operands>),
    ST4D_Z_P_BR_Contiguous(Box<ST4D_Z_P_BR_Contiguous_operands>),
    ST4H_Z_P_BI_Contiguous(Box<ST4H_Z_P_BI_Contiguous_operands>),
    ST4H_Z_P_BR_Contiguous(Box<ST4H_Z_P_BR_Contiguous_operands>),
    ST4W_Z_P_BI_Contiguous(Box<ST4W_Z_P_BI_Contiguous_operands>),
    ST4W_Z_P_BR_Contiguous(Box<ST4W_Z_P_BR_Contiguous_operands>),
    STNT1B_Z_P_BI_Contiguous(Box<STNT1B_Z_P_BI_Contiguous_operands>),
    STNT1B_Z_P_BR_Contiguous(Box<STNT1B_Z_P_BR_Contiguous_operands>),
    STNT1D_Z_P_BI_Contiguous(Box<STNT1D_Z_P_BI_Contiguous_operands>),
    STNT1D_Z_P_BR_Contiguous(Box<STNT1D_Z_P_BR_Contiguous_operands>),
    STNT1H_Z_P_BI_Contiguous(Box<STNT1H_Z_P_BI_Contiguous_operands>),
    STNT1H_Z_P_BR_Contiguous(Box<STNT1H_Z_P_BR_Contiguous_operands>),
    STNT1W_Z_P_BI_Contiguous(Box<STNT1W_Z_P_BI_Contiguous_operands>),
    STNT1W_Z_P_BR_Contiguous(Box<STNT1W_Z_P_BR_Contiguous_operands>),
    STR_P_BI__(Box<STR_P_BI___operands>),
    STR_Z_BI__(Box<STR_Z_BI___operands>),
    SUBR_Z_P_ZZ__(Box<SUBR_Z_P_ZZ___operands>),
    SUBR_Z_ZI__(Box<SUBR_Z_ZI___operands>),
    SUB_Z_P_ZZ__(Box<SUB_Z_P_ZZ___operands>),
    SUB_Z_ZI__(Box<SUB_Z_ZI___operands>),
    SUB_Z_ZZ__(Box<SUB_Z_ZZ___operands>),
    SUDOT_Z_ZZZi_S(Box<SUDOT_Z_ZZZi_S_operands>),
    SUNPKHI_Z_Z__(Box<SUNPKHI_Z_Z___operands>),
    SXTB_Z_P_Z__(Box<SXTB_Z_P_Z___operands>),
    TBL_Z_ZZ_1(Box<TBL_Z_ZZ_1_operands>),
    TRN1_P_PP__(Box<TRN1_P_PP___operands>),
    TRN1_Z_ZZ__(Box<TRN1_Z_ZZ___operands>),
    UABD_Z_P_ZZ__(Box<UABD_Z_P_ZZ___operands>),
    UADDV_R_P_Z__(Box<UADDV_R_P_Z___operands>),
    UCVTF_Z_P_Z_H2FP16(Box<UCVTF_Z_P_Z_H2FP16_operands>),
    UDIVR_Z_P_ZZ__(Box<UDIVR_Z_P_ZZ___operands>),
    UDIV_Z_P_ZZ__(Box<UDIV_Z_P_ZZ___operands>),
    UDOT_Z_ZZZ__(Box<UDOT_Z_ZZZ___operands>),
    UDOT_Z_ZZZi_S(Box<UDOT_Z_ZZZi_S_operands>),
    UMAXV_R_P_Z__(Box<UMAXV_R_P_Z___operands>),
    UMAX_Z_P_ZZ__(Box<UMAX_Z_P_ZZ___operands>),
    UMAX_Z_ZI__(Box<UMAX_Z_ZI___operands>),
    UMINV_R_P_Z__(Box<UMINV_R_P_Z___operands>),
    UMIN_Z_P_ZZ__(Box<UMIN_Z_P_ZZ___operands>),
    UMIN_Z_ZI__(Box<UMIN_Z_ZI___operands>),
    UMMLA_Z_ZZZ__(Box<UMMLA_Z_ZZZ___operands>),
    UMULH_Z_P_ZZ__(Box<UMULH_Z_P_ZZ___operands>),
    UQADD_Z_ZI__(Box<UQADD_Z_ZI___operands>),
    UQADD_Z_ZZ__(Box<UQADD_Z_ZZ___operands>),
    UQDECB_R_RS_UW(Box<UQDECB_R_RS_UW_operands>),
    UQDECD_R_RS_UW(Box<UQDECD_R_RS_UW_operands>),
    UQDECD_Z_ZS__(Box<UQDECD_Z_ZS___operands>),
    UQDECH_R_RS_UW(Box<UQDECH_R_RS_UW_operands>),
    UQDECH_Z_ZS__(Box<UQDECH_Z_ZS___operands>),
    UQDECP_R_P_R_UW(Box<UQDECP_R_P_R_UW_operands>),
    UQDECP_Z_P_Z__(Box<UQDECP_Z_P_Z___operands>),
    UQDECW_R_RS_UW(Box<UQDECW_R_RS_UW_operands>),
    UQDECW_Z_ZS__(Box<UQDECW_Z_ZS___operands>),
    UQINCB_R_RS_UW(Box<UQINCB_R_RS_UW_operands>),
    UQINCD_R_RS_UW(Box<UQINCD_R_RS_UW_operands>),
    UQINCD_Z_ZS__(Box<UQINCD_Z_ZS___operands>),
    UQINCH_R_RS_UW(Box<UQINCH_R_RS_UW_operands>),
    UQINCH_Z_ZS__(Box<UQINCH_Z_ZS___operands>),
    UQINCP_R_P_R_UW(Box<UQINCP_R_P_R_UW_operands>),
    UQINCP_Z_P_Z__(Box<UQINCP_Z_P_Z___operands>),
    UQINCW_R_RS_UW(Box<UQINCW_R_RS_UW_operands>),
    UQINCW_Z_ZS__(Box<UQINCW_Z_ZS___operands>),
    UQSUB_Z_ZI__(Box<UQSUB_Z_ZI___operands>),
    UQSUB_Z_ZZ__(Box<UQSUB_Z_ZZ___operands>),
    USDOT_Z_ZZZ_S(Box<USDOT_Z_ZZZ_S_operands>),
    USDOT_Z_ZZZi_S(Box<USDOT_Z_ZZZi_S_operands>),
    USMMLA_Z_ZZZ__(Box<USMMLA_Z_ZZZ___operands>),
    UUNPKHI_Z_Z__(Box<UUNPKHI_Z_Z___operands>),
    UXTB_Z_P_Z__(Box<UXTB_Z_P_Z___operands>),
    UZP1_P_PP__(Box<UZP1_P_PP___operands>),
    UZP1_Z_ZZ__(Box<UZP1_Z_ZZ___operands>),
    WHILELE_P_P_RR__(Box<WHILELE_P_P_RR___operands>),
    WHILELO_P_P_RR__(Box<WHILELO_P_P_RR___operands>),
    WHILELS_P_P_RR__(Box<WHILELS_P_P_RR___operands>),
    WHILELT_P_P_RR__(Box<WHILELT_P_P_RR___operands>),
    WRFFR_F_P__(Box<WRFFR_F_P___operands>),
    ZIP2_P_PP__(Box<ZIP2_P_PP___operands>),
    ZIP2_Z_ZZ__(Box<ZIP2_Z_ZZ___operands>),
    aarch64_branch_conditional_compare(Box<aarch64_branch_conditional_compare_operands>),
    aarch64_branch_conditional_cond(Box<aarch64_branch_conditional_cond_operands>),
    aarch64_branch_conditional_test(Box<aarch64_branch_conditional_test_operands>),
    aarch64_branch_unconditional_dret(Box<aarch64_branch_unconditional_dret_operands>),
    aarch64_branch_unconditional_eret(Box<aarch64_branch_unconditional_eret_operands>),
    aarch64_branch_unconditional_immediate(
        Box<aarch64_branch_unconditional_immediate_operands>,
    ),
    aarch64_branch_unconditional_register(
        Box<aarch64_branch_unconditional_register_operands>,
    ),
    aarch64_float_arithmetic_add_sub(Box<aarch64_float_arithmetic_add_sub_operands>),
    aarch64_float_arithmetic_div(Box<aarch64_float_arithmetic_div_operands>),
    aarch64_float_arithmetic_max_min(Box<aarch64_float_arithmetic_max_min_operands>),
    aarch64_float_arithmetic_mul_add_sub(
        Box<aarch64_float_arithmetic_mul_add_sub_operands>,
    ),
    aarch64_float_arithmetic_mul_product(
        Box<aarch64_float_arithmetic_mul_product_operands>,
    ),
    aarch64_float_arithmetic_round_frint(
        Box<aarch64_float_arithmetic_round_frint_operands>,
    ),
    aarch64_float_arithmetic_round_frint_32_64(
        Box<aarch64_float_arithmetic_round_frint_32_64_operands>,
    ),
    aarch64_float_arithmetic_unary(Box<aarch64_float_arithmetic_unary_operands>),
    aarch64_float_compare_cond(Box<aarch64_float_compare_cond_operands>),
    aarch64_float_compare_uncond(Box<aarch64_float_compare_uncond_operands>),
    aarch64_float_convert_fix(Box<aarch64_float_convert_fix_operands>),
    aarch64_float_convert_fp(Box<aarch64_float_convert_fp_operands>),
    aarch64_float_convert_int(Box<aarch64_float_convert_int_operands>),
    aarch64_float_move_fp_imm(Box<aarch64_float_move_fp_imm_operands>),
    aarch64_float_move_fp_select(Box<aarch64_float_move_fp_select_operands>),
    aarch64_integer_arithmetic_add_sub_carry(
        Box<aarch64_integer_arithmetic_add_sub_carry_operands>,
    ),
    aarch64_integer_arithmetic_add_sub_extendedreg(
        Box<aarch64_integer_arithmetic_add_sub_extendedreg_operands>,
    ),
    aarch64_integer_arithmetic_add_sub_immediate(
        Box<aarch64_integer_arithmetic_add_sub_immediate_operands>,
    ),
    aarch64_integer_arithmetic_add_sub_shiftedreg(
        Box<aarch64_integer_arithmetic_add_sub_shiftedreg_operands>,
    ),
    aarch64_integer_arithmetic_address_pc_rel(
        Box<aarch64_integer_arithmetic_address_pc_rel_operands>,
    ),
    aarch64_integer_arithmetic_cnt(Box<aarch64_integer_arithmetic_cnt_operands>),
    aarch64_integer_arithmetic_div(Box<aarch64_integer_arithmetic_div_operands>),
    aarch64_integer_arithmetic_mul_uniform_add_sub(
        Box<aarch64_integer_arithmetic_mul_uniform_add_sub_operands>,
    ),
    aarch64_integer_arithmetic_mul_widening_32_64(
        Box<aarch64_integer_arithmetic_mul_widening_32_64_operands>,
    ),
    aarch64_integer_arithmetic_mul_widening_64_128hi(
        Box<aarch64_integer_arithmetic_mul_widening_64_128hi_operands>,
    ),
    aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress(
        Box<aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_operands>,
    ),
    aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
        Box<aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_operands>,
    ),
    aarch64_integer_arithmetic_rbit(Box<aarch64_integer_arithmetic_rbit_operands>),
    aarch64_integer_arithmetic_rev(Box<aarch64_integer_arithmetic_rev_operands>),
    aarch64_integer_bitfield(Box<aarch64_integer_bitfield_operands>),
    aarch64_integer_conditional_compare_immediate(
        Box<aarch64_integer_conditional_compare_immediate_operands>,
    ),
    aarch64_integer_conditional_compare_register(
        Box<aarch64_integer_conditional_compare_register_operands>,
    ),
    aarch64_integer_conditional_select(Box<aarch64_integer_conditional_select_operands>),
    aarch64_integer_crc(Box<aarch64_integer_crc_operands>),
    aarch64_integer_flags_axflag(Box<aarch64_integer_flags_axflag_operands>),
    aarch64_integer_flags_cfinv(Box<aarch64_integer_flags_cfinv_operands>),
    aarch64_integer_flags_rmif(Box<aarch64_integer_flags_rmif_operands>),
    aarch64_integer_flags_setf(Box<aarch64_integer_flags_setf_operands>),
    aarch64_integer_flags_xaflag(Box<aarch64_integer_flags_xaflag_operands>),
    aarch64_integer_ins_ext_extract_immediate(
        Box<aarch64_integer_ins_ext_extract_immediate_operands>,
    ),
    aarch64_integer_ins_ext_insert_movewide(
        Box<aarch64_integer_ins_ext_insert_movewide_operands>,
    ),
    aarch64_integer_logical_immediate(Box<aarch64_integer_logical_immediate_operands>),
    aarch64_integer_logical_shiftedreg(Box<aarch64_integer_logical_shiftedreg_operands>),
    aarch64_integer_pac_autda_dp_1src(Box<aarch64_integer_pac_autda_dp_1src_operands>),
    aarch64_integer_pac_autdb_dp_1src(Box<aarch64_integer_pac_autdb_dp_1src_operands>),
    aarch64_integer_pac_autia_dp_1src(Box<aarch64_integer_pac_autia_dp_1src_operands>),
    aarch64_integer_pac_autib_dp_1src(Box<aarch64_integer_pac_autib_dp_1src_operands>),
    aarch64_integer_pac_pacda_dp_1src(Box<aarch64_integer_pac_pacda_dp_1src_operands>),
    aarch64_integer_pac_pacdb_dp_1src(Box<aarch64_integer_pac_pacdb_dp_1src_operands>),
    aarch64_integer_pac_pacga_dp_2src(Box<aarch64_integer_pac_pacga_dp_2src_operands>),
    aarch64_integer_pac_pacia_dp_1src(Box<aarch64_integer_pac_pacia_dp_1src_operands>),
    aarch64_integer_pac_pacib_dp_1src(Box<aarch64_integer_pac_pacib_dp_1src_operands>),
    aarch64_integer_pac_strip_dp_1src(Box<aarch64_integer_pac_strip_dp_1src_operands>),
    aarch64_integer_shift_variable(Box<aarch64_integer_shift_variable_operands>),
    aarch64_integer_tags_mcaddtag(Box<aarch64_integer_tags_mcaddtag_operands>),
    aarch64_integer_tags_mcgettag(Box<aarch64_integer_tags_mcgettag_operands>),
    aarch64_integer_tags_mcgettagarray(Box<aarch64_integer_tags_mcgettagarray_operands>),
    aarch64_integer_tags_mcinsertrandomtag(
        Box<aarch64_integer_tags_mcinsertrandomtag_operands>,
    ),
    aarch64_integer_tags_mcinserttagmask(
        Box<aarch64_integer_tags_mcinserttagmask_operands>,
    ),
    aarch64_integer_tags_mcsettaganddatapairpost(
        Box<aarch64_integer_tags_mcsettaganddatapairpost_operands>,
    ),
    aarch64_integer_tags_mcsettagandzeroarray(
        Box<aarch64_integer_tags_mcsettagandzeroarray_operands>,
    ),
    aarch64_integer_tags_mcsettagandzerodatapost(
        Box<aarch64_integer_tags_mcsettagandzerodatapost_operands>,
    ),
    aarch64_integer_tags_mcsettagarray(Box<aarch64_integer_tags_mcsettagarray_operands>),
    aarch64_integer_tags_mcsettagpairandzerodatapost(
        Box<aarch64_integer_tags_mcsettagpairandzerodatapost_operands>,
    ),
    aarch64_integer_tags_mcsettagpairpost(
        Box<aarch64_integer_tags_mcsettagpairpost_operands>,
    ),
    aarch64_integer_tags_mcsettagpost(Box<aarch64_integer_tags_mcsettagpost_operands>),
    aarch64_integer_tags_mcsubtag(Box<aarch64_integer_tags_mcsubtag_operands>),
    aarch64_memory_atomicops_cas_pair(Box<aarch64_memory_atomicops_cas_pair_operands>),
    aarch64_memory_atomicops_cas_single(
        Box<aarch64_memory_atomicops_cas_single_operands>,
    ),
    aarch64_memory_atomicops_ld(Box<aarch64_memory_atomicops_ld_operands>),
    aarch64_memory_atomicops_swp(Box<aarch64_memory_atomicops_swp_operands>),
    aarch64_memory_exclusive_pair(Box<aarch64_memory_exclusive_pair_operands>),
    aarch64_memory_exclusive_single(Box<aarch64_memory_exclusive_single_operands>),
    aarch64_memory_literal_general(Box<aarch64_memory_literal_general_operands>),
    aarch64_memory_literal_simdfp(Box<aarch64_memory_literal_simdfp_operands>),
    aarch64_memory_ordered(Box<aarch64_memory_ordered_operands>),
    aarch64_memory_ordered_rcpc(Box<aarch64_memory_ordered_rcpc_operands>),
    aarch64_memory_pair_general_no_alloc(
        Box<aarch64_memory_pair_general_no_alloc_operands>,
    ),
    aarch64_memory_pair_general_post_idx(
        Box<aarch64_memory_pair_general_post_idx_operands>,
    ),
    aarch64_memory_pair_simdfp_no_alloc(
        Box<aarch64_memory_pair_simdfp_no_alloc_operands>,
    ),
    aarch64_memory_pair_simdfp_post_idx(
        Box<aarch64_memory_pair_simdfp_post_idx_operands>,
    ),
    aarch64_memory_single_general_immediate_signed_offset_lda_stl(
        Box<aarch64_memory_single_general_immediate_signed_offset_lda_stl_operands>,
    ),
    aarch64_memory_single_general_immediate_signed_offset_normal(
        Box<aarch64_memory_single_general_immediate_signed_offset_normal_operands>,
    ),
    aarch64_memory_single_general_immediate_signed_offset_unpriv(
        Box<aarch64_memory_single_general_immediate_signed_offset_unpriv_operands>,
    ),
    aarch64_memory_single_general_immediate_signed_pac(
        Box<aarch64_memory_single_general_immediate_signed_pac_operands>,
    ),
    aarch64_memory_single_general_immediate_signed_post_idx(
        Box<aarch64_memory_single_general_immediate_signed_post_idx_operands>,
    ),
    aarch64_memory_single_general_immediate_unsigned(
        Box<aarch64_memory_single_general_immediate_unsigned_operands>,
    ),
    aarch64_memory_single_general_register(
        Box<aarch64_memory_single_general_register_operands>,
    ),
    aarch64_memory_single_simdfp_immediate_signed_offset_normal(
        Box<aarch64_memory_single_simdfp_immediate_signed_offset_normal_operands>,
    ),
    aarch64_memory_single_simdfp_immediate_signed_post_idx(
        Box<aarch64_memory_single_simdfp_immediate_signed_post_idx_operands>,
    ),
    aarch64_memory_single_simdfp_register(
        Box<aarch64_memory_single_simdfp_register_operands>,
    ),
    aarch64_memory_vector_multiple_no_wb(
        Box<aarch64_memory_vector_multiple_no_wb_operands>,
    ),
    aarch64_memory_vector_single_no_wb(Box<aarch64_memory_vector_single_no_wb_operands>),
    aarch64_system_barriers_dmb(Box<aarch64_system_barriers_dmb_operands>),
    aarch64_system_barriers_dsb(Box<aarch64_system_barriers_dsb_operands>),
    aarch64_system_barriers_isb(Box<aarch64_system_barriers_isb_operands>),
    aarch64_system_barriers_pssbb(Box<aarch64_system_barriers_pssbb_operands>),
    aarch64_system_barriers_sb(Box<aarch64_system_barriers_sb_operands>),
    aarch64_system_barriers_ssbb(Box<aarch64_system_barriers_ssbb_operands>),
    aarch64_system_exceptions_debug_breakpoint(
        Box<aarch64_system_exceptions_debug_breakpoint_operands>,
    ),
    aarch64_system_exceptions_debug_exception(
        Box<aarch64_system_exceptions_debug_exception_operands>,
    ),
    aarch64_system_exceptions_debug_halt(
        Box<aarch64_system_exceptions_debug_halt_operands>,
    ),
    aarch64_system_exceptions_runtime_hvc(
        Box<aarch64_system_exceptions_runtime_hvc_operands>,
    ),
    aarch64_system_exceptions_runtime_smc(
        Box<aarch64_system_exceptions_runtime_smc_operands>,
    ),
    aarch64_system_exceptions_runtime_svc(
        Box<aarch64_system_exceptions_runtime_svc_operands>,
    ),
    aarch64_system_hints(Box<aarch64_system_hints_operands>),
    aarch64_system_monitors(Box<aarch64_system_monitors_operands>),
    aarch64_system_register_cpsr(Box<aarch64_system_register_cpsr_operands>),
    aarch64_system_register_system(Box<aarch64_system_register_system_operands>),
    aarch64_system_sysops(Box<aarch64_system_sysops_operands>),
    aarch64_udf(Box<aarch64_udf_operands>),
    aarch64_vector_arithmetic_binary_disparate_add_sub_long(
        Box<aarch64_vector_arithmetic_binary_disparate_add_sub_long_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_add_sub_narrow(
        Box<aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_add_sub_wide(
        Box<aarch64_vector_arithmetic_binary_disparate_add_sub_wide_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_diff(
        Box<aarch64_vector_arithmetic_binary_disparate_diff_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_mul_accum(
        Box<aarch64_vector_arithmetic_binary_disparate_mul_accum_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd(
        Box<aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_mul_double_sisd(
        Box<aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_mul_poly(
        Box<aarch64_vector_arithmetic_binary_disparate_mul_poly_operands>,
    ),
    aarch64_vector_arithmetic_binary_disparate_mul_product(
        Box<aarch64_vector_arithmetic_binary_disparate_mul_product_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_bfdot(
        Box<aarch64_vector_arithmetic_binary_element_bfdot_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_dotp(
        Box<aarch64_vector_arithmetic_binary_element_dotp_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp(
        Box<aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_complex(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_complex_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_int(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_int_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_long(
        Box<aarch64_vector_arithmetic_binary_element_mul_acc_long_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
        Box<
            aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower_operands,
        >,
    ),
    aarch64_vector_arithmetic_binary_element_mul_double_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_double_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_high_sisd(
        Box<aarch64_vector_arithmetic_binary_element_mul_high_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_int(
        Box<aarch64_vector_arithmetic_binary_element_mul_int_operands>,
    ),
    aarch64_vector_arithmetic_binary_element_mul_long(
        Box<aarch64_vector_arithmetic_binary_element_mul_long_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_fp16(
        Box<aarch64_vector_arithmetic_binary_uniform_add_fp16_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_fp_complex(
        Box<aarch64_vector_arithmetic_binary_uniform_add_fp_complex_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_halving_rounding(
        Box<aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_halving_truncating(
        Box<aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair(
        Box<aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_diff(
        Box<aarch64_vector_arithmetic_binary_uniform_diff_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_div_fp16(
        Box<aarch64_vector_arithmetic_binary_uniform_div_fp16_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_logical_and_orr(
        Box<aarch64_vector_arithmetic_binary_uniform_logical_and_orr_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor(
        Box<aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
        Box<aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot(
        Box<aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985(
        Box<aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008(
        Box<aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_max_min_pair(
        Box<aarch64_vector_arithmetic_binary_uniform_max_min_pair_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_max_min_single(
        Box<aarch64_vector_arithmetic_binary_uniform_max_min_single_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_product(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_fp_complex(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower(
        Box<
            aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_operands,
        >,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_accum(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_int_accum_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_dotp(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_int_dotp_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
        Box<
            aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_operands,
        >,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_mul_int_product(
        Box<aarch64_vector_arithmetic_binary_uniform_mul_int_product_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_shift_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_shift_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd(
        Box<aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_sub_int(
        Box<aarch64_vector_arithmetic_binary_uniform_sub_int_operands>,
    ),
    aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd(
        Box<aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_add_pairwise(
        Box<aarch64_vector_arithmetic_unary_add_pairwise_operands>,
    ),
    aarch64_vector_arithmetic_unary_add_saturating_sisd(
        Box<aarch64_vector_arithmetic_unary_add_saturating_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_clsz(
        Box<aarch64_vector_arithmetic_unary_clsz_operands>,
    ),
    aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
        Box<aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
        Box<aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd(
        Box<aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd(
        Box<aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_cnt(
        Box<aarch64_vector_arithmetic_unary_cnt_operands>,
    ),
    aarch64_vector_arithmetic_unary_diff_neg_fp16(
        Box<aarch64_vector_arithmetic_unary_diff_neg_fp16_operands>,
    ),
    aarch64_vector_arithmetic_unary_diff_neg_int_sisd(
        Box<aarch64_vector_arithmetic_unary_diff_neg_int_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_diff_neg_sat_sisd(
        Box<aarch64_vector_arithmetic_unary_diff_neg_sat_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_extract_nosat(
        Box<aarch64_vector_arithmetic_unary_extract_nosat_operands>,
    ),
    aarch64_vector_arithmetic_unary_extract_sat_sisd(
        Box<aarch64_vector_arithmetic_unary_extract_sat_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_extract_sqxtun_sisd(
        Box<aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_float_narrow(
        Box<aarch64_vector_arithmetic_unary_float_narrow_operands>,
    ),
    aarch64_vector_arithmetic_unary_float_round_frint_32_64(
        Box<aarch64_vector_arithmetic_unary_float_round_frint_32_64_operands>,
    ),
    aarch64_vector_arithmetic_unary_float_widen(
        Box<aarch64_vector_arithmetic_unary_float_widen_operands>,
    ),
    aarch64_vector_arithmetic_unary_float_xtn_sisd(
        Box<aarch64_vector_arithmetic_unary_float_xtn_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
        Box<aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
        Box<aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_fp16_conv_int_sisd(
        Box<aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_fp16_round(
        Box<aarch64_vector_arithmetic_unary_fp16_round_operands>,
    ),
    aarch64_vector_arithmetic_unary_not(
        Box<aarch64_vector_arithmetic_unary_not_operands>,
    ),
    aarch64_vector_arithmetic_unary_rbit(
        Box<aarch64_vector_arithmetic_unary_rbit_operands>,
    ),
    aarch64_vector_arithmetic_unary_rev(
        Box<aarch64_vector_arithmetic_unary_rev_operands>,
    ),
    aarch64_vector_arithmetic_unary_shift(
        Box<aarch64_vector_arithmetic_unary_shift_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_frecpx_fp16(
        Box<aarch64_vector_arithmetic_unary_special_frecpx_fp16_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_recip_fp16_sisd(
        Box<aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_recip_int(
        Box<aarch64_vector_arithmetic_unary_special_recip_int_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd(
        Box<aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_sqrt_est_int(
        Box<aarch64_vector_arithmetic_unary_special_sqrt_est_int_operands>,
    ),
    aarch64_vector_arithmetic_unary_special_sqrt_fp16(
        Box<aarch64_vector_arithmetic_unary_special_sqrt_fp16_operands>,
    ),
    aarch64_vector_bfmmla(Box<aarch64_vector_bfmmla_operands>),
    aarch64_vector_crypto_aes_mix(Box<aarch64_vector_crypto_aes_mix_operands>),
    aarch64_vector_crypto_aes_round(Box<aarch64_vector_crypto_aes_round_operands>),
    aarch64_vector_crypto_sha2op_sha1_hash(
        Box<aarch64_vector_crypto_sha2op_sha1_hash_operands>,
    ),
    aarch64_vector_crypto_sha2op_sha1_sched1(
        Box<aarch64_vector_crypto_sha2op_sha1_sched1_operands>,
    ),
    aarch64_vector_crypto_sha2op_sha256_sched0(
        Box<aarch64_vector_crypto_sha2op_sha256_sched0_operands>,
    ),
    aarch64_vector_crypto_sha3_bcax(Box<aarch64_vector_crypto_sha3_bcax_operands>),
    aarch64_vector_crypto_sha3_eor3(Box<aarch64_vector_crypto_sha3_eor3_operands>),
    aarch64_vector_crypto_sha3_rax1(Box<aarch64_vector_crypto_sha3_rax1_operands>),
    aarch64_vector_crypto_sha3_xar(Box<aarch64_vector_crypto_sha3_xar_operands>),
    aarch64_vector_crypto_sha3op_sha1_hash_choose(
        Box<aarch64_vector_crypto_sha3op_sha1_hash_choose_operands>,
    ),
    aarch64_vector_crypto_sha3op_sha1_hash_majority(
        Box<aarch64_vector_crypto_sha3op_sha1_hash_majority_operands>,
    ),
    aarch64_vector_crypto_sha3op_sha1_hash_parity(
        Box<aarch64_vector_crypto_sha3op_sha1_hash_parity_operands>,
    ),
    aarch64_vector_crypto_sha3op_sha1_sched0(
        Box<aarch64_vector_crypto_sha3op_sha1_sched0_operands>,
    ),
    aarch64_vector_crypto_sha3op_sha256_hash(
        Box<aarch64_vector_crypto_sha3op_sha256_hash_operands>,
    ),
    aarch64_vector_crypto_sha3op_sha256_sched1(
        Box<aarch64_vector_crypto_sha3op_sha256_sched1_operands>,
    ),
    aarch64_vector_crypto_sha512_sha512h(
        Box<aarch64_vector_crypto_sha512_sha512h_operands>,
    ),
    aarch64_vector_crypto_sha512_sha512h2(
        Box<aarch64_vector_crypto_sha512_sha512h2_operands>,
    ),
    aarch64_vector_crypto_sha512_sha512su0(
        Box<aarch64_vector_crypto_sha512_sha512su0_operands>,
    ),
    aarch64_vector_crypto_sha512_sha512su1(
        Box<aarch64_vector_crypto_sha512_sha512su1_operands>,
    ),
    aarch64_vector_crypto_sm3_sm3partw1(
        Box<aarch64_vector_crypto_sm3_sm3partw1_operands>,
    ),
    aarch64_vector_crypto_sm3_sm3partw2(
        Box<aarch64_vector_crypto_sm3_sm3partw2_operands>,
    ),
    aarch64_vector_crypto_sm3_sm3ss1(Box<aarch64_vector_crypto_sm3_sm3ss1_operands>),
    aarch64_vector_crypto_sm3_sm3tt1a(Box<aarch64_vector_crypto_sm3_sm3tt1a_operands>),
    aarch64_vector_crypto_sm3_sm3tt1b(Box<aarch64_vector_crypto_sm3_sm3tt1b_operands>),
    aarch64_vector_crypto_sm3_sm3tt2a(Box<aarch64_vector_crypto_sm3_sm3tt2a_operands>),
    aarch64_vector_crypto_sm3_sm3tt2b(Box<aarch64_vector_crypto_sm3_sm3tt2b_operands>),
    aarch64_vector_crypto_sm4_sm4enc(Box<aarch64_vector_crypto_sm4_sm4enc_operands>),
    aarch64_vector_crypto_sm4_sm4enckey(
        Box<aarch64_vector_crypto_sm4_sm4enckey_operands>,
    ),
    aarch64_vector_cvt_bf16_scalar(Box<aarch64_vector_cvt_bf16_scalar_operands>),
    aarch64_vector_cvt_bf16_vector(Box<aarch64_vector_cvt_bf16_vector_operands>),
    aarch64_vector_fp16_movi(Box<aarch64_vector_fp16_movi_operands>),
    aarch64_vector_logical(Box<aarch64_vector_logical_operands>),
    aarch64_vector_reduce_add_long(Box<aarch64_vector_reduce_add_long_operands>),
    aarch64_vector_reduce_add_simd(Box<aarch64_vector_reduce_add_simd_operands>),
    aarch64_vector_reduce_add_sisd(Box<aarch64_vector_reduce_add_sisd_operands>),
    aarch64_vector_reduce_fp16_add_sisd(
        Box<aarch64_vector_reduce_fp16_add_sisd_operands>,
    ),
    aarch64_vector_reduce_fp16_max_simd(
        Box<aarch64_vector_reduce_fp16_max_simd_operands>,
    ),
    aarch64_vector_reduce_fp16_max_sisd(
        Box<aarch64_vector_reduce_fp16_max_sisd_operands>,
    ),
    aarch64_vector_reduce_fp16_maxnm_simd(
        Box<aarch64_vector_reduce_fp16_maxnm_simd_operands>,
    ),
    aarch64_vector_reduce_fp16_maxnm_sisd(
        Box<aarch64_vector_reduce_fp16_maxnm_sisd_operands>,
    ),
    aarch64_vector_reduce_int_max(Box<aarch64_vector_reduce_int_max_operands>),
    aarch64_vector_shift_conv_float_sisd(
        Box<aarch64_vector_shift_conv_float_sisd_operands>,
    ),
    aarch64_vector_shift_conv_int_sisd(Box<aarch64_vector_shift_conv_int_sisd_operands>),
    aarch64_vector_shift_left_insert_sisd(
        Box<aarch64_vector_shift_left_insert_sisd_operands>,
    ),
    aarch64_vector_shift_left_long(Box<aarch64_vector_shift_left_long_operands>),
    aarch64_vector_shift_left_sat_sisd(Box<aarch64_vector_shift_left_sat_sisd_operands>),
    aarch64_vector_shift_left_sisd(Box<aarch64_vector_shift_left_sisd_operands>),
    aarch64_vector_shift_right_insert_sisd(
        Box<aarch64_vector_shift_right_insert_sisd_operands>,
    ),
    aarch64_vector_shift_right_narrow_logical(
        Box<aarch64_vector_shift_right_narrow_logical_operands>,
    ),
    aarch64_vector_shift_right_narrow_nonuniform_sisd(
        Box<aarch64_vector_shift_right_narrow_nonuniform_sisd_operands>,
    ),
    aarch64_vector_shift_right_narrow_uniform_sisd(
        Box<aarch64_vector_shift_right_narrow_uniform_sisd_operands>,
    ),
    aarch64_vector_shift_right_sisd(Box<aarch64_vector_shift_right_sisd_operands>),
    aarch64_vector_transfer_integer_dup(
        Box<aarch64_vector_transfer_integer_dup_operands>,
    ),
    aarch64_vector_transfer_integer_insert(
        Box<aarch64_vector_transfer_integer_insert_operands>,
    ),
    aarch64_vector_transfer_integer_move_signed(
        Box<aarch64_vector_transfer_integer_move_signed_operands>,
    ),
    aarch64_vector_transfer_integer_move_unsigned(
        Box<aarch64_vector_transfer_integer_move_unsigned_operands>,
    ),
    aarch64_vector_transfer_vector_cpy_dup_sisd(
        Box<aarch64_vector_transfer_vector_cpy_dup_sisd_operands>,
    ),
    aarch64_vector_transfer_vector_extract(
        Box<aarch64_vector_transfer_vector_extract_operands>,
    ),
    aarch64_vector_transfer_vector_insert(
        Box<aarch64_vector_transfer_vector_insert_operands>,
    ),
    aarch64_vector_transfer_vector_permute_transpose(
        Box<aarch64_vector_transfer_vector_permute_transpose_operands>,
    ),
    aarch64_vector_transfer_vector_permute_unzip(
        Box<aarch64_vector_transfer_vector_permute_unzip_operands>,
    ),
    aarch64_vector_transfer_vector_permute_zip(
        Box<aarch64_vector_transfer_vector_permute_zip_operands>,
    ),
    aarch64_vector_transfer_vector_table(
        Box<aarch64_vector_transfer_vector_table_operands>,
    ),
}
#[derive(Debug)]
pub struct ABS_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ADDPL_R_RI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ADDVL_R_RI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ADD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ADD_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ADD_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ADR_Z_AZ_SD_same_scaled_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ANDV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct AND_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct AND_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct AND_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct AND_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ASRD_Z_P_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ASRR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ASR_Z_P_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ASR_Z_P_ZW___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ASR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ASR_Z_ZI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ASR_Z_ZW___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFCVTNT_Z_P_Z_S2BF_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFCVT_Z_P_Z_S2BF_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFDOT_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFDOT_Z_ZZZi___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFMLALB_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFMLALB_Z_ZZZi___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFMLALT_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFMLALT_Z_ZZZi___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BFMMLA_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BIC_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BIC_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct BIC_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BRKA_P_P_P___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BRKB_P_P_P___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BRKN_P_P_PP___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BRKPA_P_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct BRKPB_P_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTA_R_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTA_V_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTA_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTB_R_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTB_V_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLASTB_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct CLS_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CLZ_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CMPEQ_P_P_ZI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CMPEQ_P_P_ZW___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CMPEQ_P_P_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CNOT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CNTB_R_S___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct CNTP_R_P_P___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CNT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct COMPACT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CPY_Z_O_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct CPY_Z_P_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct CPY_Z_P_R___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CPY_Z_P_V___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct CTERMEQ_RR___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct DECB_R_RS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct DECD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct DECP_R_P_R___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct DECP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct DUPM_Z_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct DUP_Z_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct DUP_Z_R___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct DUP_Z_Zi___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct EORV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct EOR_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct EOR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct EOR_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct EOR_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct EXT_Z_ZI_Des_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FABD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FABS_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FACGT_P_P_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FADDA_V_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FADDV_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FADD_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FADD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FADD_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCADD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FCMEQ_P_P_Z0___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCMEQ_P_P_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCMLA_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCMLA_Z_ZZZi_H_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCPY_Z_P_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct FCVTZS_Z_P_Z_FP162H_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCVTZU_Z_P_Z_FP162H_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FCVT_Z_P_Z_H2S_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FDIVR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FDIV_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FDUP_Z_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct FEXPA_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMAD_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMAXNMV_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMAXNM_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMAXNM_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMAXV_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMAX_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMAX_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMINNMV_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMINNM_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMINNM_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMINV_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMIN_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMIN_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMLA_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMLA_Z_ZZZi_H_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMLS_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMLS_Z_ZZZi_H_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMMLA_Z_ZZZ_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMSB_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMULX_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMUL_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMUL_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FMUL_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FMUL_Z_ZZi_H_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FNEG_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FNMAD_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FNMLA_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FNMLS_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FNMSB_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FRECPE_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FRECPS_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FRECPX_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FRINTI_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FRSQRTE_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FRSQRTS_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FSCALE_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FSQRT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FSUBR_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FSUBR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FSUB_Z_P_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FSUB_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FSUB_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FTMAD_Z_ZZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct FTSMUL_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct FTSSEL_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct INCB_R_RS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct INCD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct INCP_R_P_R___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct INCP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct INDEX_Z_II___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct INDEX_Z_IR___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct INDEX_Z_RI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct INDEX_Z_RR___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct INSR_Z_R___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct INSR_Z_V___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LASTA_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LASTA_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LASTB_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LASTB_V_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LD1B_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1B_Z_P_BI_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1B_Z_P_BR_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1B_Z_P_BZ_D_x32_unscaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1D_Z_P_AI_D_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1D_Z_P_BI_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1D_Z_P_BR_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1D_Z_P_BZ_D_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1H_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1H_Z_P_BI_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1H_Z_P_BR_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1H_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RB_Z_P_BI_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RD_Z_P_BI_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RH_Z_P_BI_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROB_Z_P_BI_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROB_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROD_Z_P_BI_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROD_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROH_Z_P_BI_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROH_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROW_Z_P_BI_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1ROW_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQB_Z_P_BI_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQB_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQD_Z_P_BI_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQD_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQH_Z_P_BI_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQH_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQW_Z_P_BI_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RQW_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RSB_Z_P_BI_S16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RSH_Z_P_BI_S32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RSW_Z_P_BI_S64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1RW_Z_P_BI_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SB_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SB_Z_P_BI_S16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SB_Z_P_BR_S16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SB_Z_P_BZ_D_x32_unscaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SH_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SH_Z_P_BI_S32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SH_Z_P_BR_S32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SH_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SW_Z_P_AI_D_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SW_Z_P_BI_S64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SW_Z_P_BR_S64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1SW_Z_P_BZ_D_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1W_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1W_Z_P_BI_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1W_Z_P_BR_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD1W_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD2W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD3W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LD4W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1B_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1B_Z_P_BR_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1B_Z_P_BZ_D_x32_unscaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1D_Z_P_AI_D_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1D_Z_P_BR_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1D_Z_P_BZ_D_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1H_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1H_Z_P_BR_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1H_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SB_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SB_Z_P_BR_S16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SB_Z_P_BZ_D_x32_unscaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SH_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SH_Z_P_BR_S32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SH_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SW_Z_P_AI_D_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SW_Z_P_BR_S64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1SW_Z_P_BZ_D_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1W_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1W_Z_P_BR_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDFF1W_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1B_Z_P_BI_U8_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1D_Z_P_BI_U64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1H_Z_P_BI_U16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1SB_Z_P_BI_S16_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1SH_Z_P_BI_S32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1SW_Z_P_BI_S64_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNF1W_Z_P_BI_U32_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDNT1W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDR_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LDR_Z_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct LSLR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSL_Z_P_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSL_Z_P_ZW___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSL_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSL_Z_ZI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LSL_Z_ZW___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LSRR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSR_Z_P_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSR_Z_P_ZW___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct LSR_Z_ZI___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct LSR_Z_ZW___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct MAD_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct MLA_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct MLS_Z_P_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct MOVPRFX_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct MOVPRFX_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct MSB_Z_P_ZZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct MUL_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct MUL_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct NAND_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct NEG_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct NOR_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct NOT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ORN_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ORR_P_P_PP_Z_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ORR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ORR_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct ORR_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ORV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PFALSE_P___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct PFIRST_P_P_P___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct PNEXT_P_P_P___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct PRFB_I_P_AI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFB_I_P_BI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFB_I_P_BR_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFB_I_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFD_I_P_AI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFD_I_P_BI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFD_I_P_BR_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFD_I_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFH_I_P_AI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFH_I_P_BI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFH_I_P_BR_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFH_I_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFW_I_P_AI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFW_I_P_BI_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFW_I_P_BR_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PRFW_I_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PTEST__P_P___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct PTRUE_P_S___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct PUNPKHI_P_P___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct RBIT_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct RDFFR_P_F___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct RDFFR_P_P_F___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct RDVL_R_I___operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct REVB_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct REV_P_P___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct REV_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SABD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SADDV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SCVTF_Z_P_Z_H2FP16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SDIVR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SDIV_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SDOT_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SDOT_Z_ZZZi_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SEL_P_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SEL_Z_P_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SETFFR_F___operands {}
#[derive(Debug)]
pub struct SMAXV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SMAX_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SMAX_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SMINV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SMIN_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SMIN_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SMMLA_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SMULH_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SPLICE_Z_P_ZZ_Des_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQADD_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQADD_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECB_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECD_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECH_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECH_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECP_R_P_R_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECW_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQDECW_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCB_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCD_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCH_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCH_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCP_R_P_R_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCW_R_RS_SX_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQINCW_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQSUB_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SQSUB_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ST1B_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1B_Z_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1B_Z_P_BR___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1B_Z_P_BZ_D_x32_unscaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1D_Z_P_AI_D_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1D_Z_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1D_Z_P_BR___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1D_Z_P_BZ_D_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1H_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1H_Z_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1H_Z_P_BR___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1H_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1W_Z_P_AI_S_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1W_Z_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1W_Z_P_BR___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST1W_Z_P_BZ_S_x32_scaled_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST2W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST3W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct ST4W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1B_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1B_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1D_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1D_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1H_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1H_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1W_Z_P_BI_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STNT1W_Z_P_BR_Contiguous_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STR_P_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct STR_Z_BI___operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct SUBR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SUBR_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SUB_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SUB_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct SUB_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SUDOT_Z_ZZZi_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SUNPKHI_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct SXTB_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct TBL_Z_ZZ_1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct TRN1_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct TRN1_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UABD_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UADDV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UCVTF_Z_P_Z_H2FP16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UDIVR_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UDIV_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UDOT_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UDOT_Z_ZZZi_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UMAXV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UMAX_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UMAX_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UMINV_R_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UMIN_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UMIN_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UMMLA_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UMULH_Z_P_ZZ___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQADD_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQADD_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECB_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECD_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECH_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECH_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECP_R_P_R_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECW_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQDECW_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCB_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCD_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCD_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCH_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCH_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCP_R_P_R_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCP_Z_P_Z___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCW_R_RS_UW_operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQINCW_Z_ZS___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQSUB_Z_ZI___operands {
    pub dn: common::types::integer,
}
#[derive(Debug)]
pub struct UQSUB_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct USDOT_Z_ZZZ_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct USDOT_Z_ZZZi_S_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct USMMLA_Z_ZZZ___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UUNPKHI_Z_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UXTB_Z_P_Z___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UZP1_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct UZP1_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct WHILELE_P_P_RR___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct WHILELO_P_P_RR___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct WHILELS_P_P_RR___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct WHILELT_P_P_RR___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct WRFFR_F_P___operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ZIP2_P_PP___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct ZIP2_Z_ZZ___operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_branch_conditional_compare_operands {
    pub datasize: common::types::integer,
    pub iszero: common::types::boolean,
    pub offset: common::types::bits,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_branch_conditional_cond_operands {
    pub condition: common::types::bits,
    pub offset: common::types::bits,
}
#[derive(Debug)]
pub struct aarch64_branch_conditional_test_operands {
    pub bit_pos: common::types::integer,
    pub bit_val: common::types::bits,
    pub datasize: common::types::integer,
    pub offset: common::types::bits,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_branch_unconditional_dret_operands {}
#[derive(Debug)]
pub struct aarch64_branch_unconditional_eret_operands {}
#[derive(Debug)]
pub struct aarch64_branch_unconditional_immediate_operands {
    pub branch_type: common::types::BranchType,
    pub offset: common::types::bits,
}
#[derive(Debug)]
pub struct aarch64_branch_unconditional_register_operands {
    pub branch_type: common::types::BranchType,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub pac: common::types::boolean,
    pub source_is_sp: common::types::boolean,
    pub use_key_a: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_add_sub_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_div_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_max_min_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_mul_add_sub_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_mul_product_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_round_frint_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_round_frint_32_64_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_arithmetic_unary_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_compare_cond_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_compare_uncond_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_convert_fix_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_convert_fp_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_convert_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_move_fp_imm_operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_float_move_fp_select_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_add_sub_carry_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub setflags: common::types::boolean,
    pub sub_op: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_add_sub_extendedreg_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub extend_type: common::types::ExtendType,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub setflags: common::types::boolean,
    pub shift: common::types::integer,
    pub sub_op: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_add_sub_immediate_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_add_sub_shiftedreg_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub setflags: common::types::boolean,
    pub shift_amount: common::types::integer,
    pub shift_type: common::types::ShiftType,
    pub sub_op: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_address_pc_rel_operands {
    pub d: common::types::integer,
    pub imm: common::types::bits,
    pub page: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_cnt_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub n: common::types::integer,
    pub opcode: common::types::CountOp,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_div_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub unsigned: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_mul_uniform_add_sub_operands {
    pub a: common::types::integer,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub destsize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub sub_op: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_mul_widening_32_64_operands {
    pub a: common::types::integer,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub destsize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub sub_op: common::types::boolean,
    pub unsigned: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_mul_widening_64_128hi_operands {
    pub a: common::types::integer,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub destsize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub unsigned: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_rbit_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_arithmetic_rev_operands {
    pub container_size: common::types::integer,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_bitfield_operands {
    pub R: common::types::integer,
    pub S: common::types::integer,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub extend: common::types::boolean,
    pub inzero: common::types::boolean,
    pub n: common::types::integer,
    pub tmask: common::types::bits,
    pub wmask: common::types::bits,
}
#[derive(Debug)]
pub struct aarch64_integer_conditional_compare_immediate_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_conditional_compare_register_operands {
    pub condition: common::types::bits,
    pub datasize: common::types::integer,
    pub flags: common::types::bits,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub sub_op: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_conditional_select_operands {
    pub condition: common::types::bits,
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub else_inc: common::types::boolean,
    pub else_inv: common::types::boolean,
    pub m: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_crc_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_flags_axflag_operands {}
#[derive(Debug)]
pub struct aarch64_integer_flags_cfinv_operands {}
#[derive(Debug)]
pub struct aarch64_integer_flags_rmif_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_flags_setf_operands {
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_flags_xaflag_operands {}
#[derive(Debug)]
pub struct aarch64_integer_ins_ext_extract_immediate_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub lsb: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_ins_ext_insert_movewide_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub imm: common::types::bits,
    pub opcode: common::types::MoveWideOp,
    pub pos: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_logical_immediate_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub imm: common::types::bits,
    pub n: common::types::integer,
    pub op: common::types::LogicalOp,
    pub setflags: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_integer_logical_shiftedreg_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub invert: common::types::boolean,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub op: common::types::LogicalOp,
    pub setflags: common::types::boolean,
    pub shift_amount: common::types::integer,
    pub shift_type: common::types::ShiftType,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_autda_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_autdb_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_autia_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_autib_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_pacda_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_pacdb_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_pacga_dp_2src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_pacia_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_pacib_dp_1src_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_pac_strip_dp_1src_operands {
    pub d: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_shift_variable_operands {
    pub d: common::types::integer,
    pub datasize: common::types::integer,
    pub m: common::types::integer,
    pub n: common::types::integer,
    pub shift_type: common::types::ShiftType,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcaddtag_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcgettag_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcgettagarray_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcinsertrandomtag_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcinserttagmask_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettaganddatapairpost_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagandzeroarray_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagandzerodatapost_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagarray_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagpairandzerodatapost_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagpairpost_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsettagpost_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_integer_tags_mcsubtag_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_atomicops_cas_pair_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_atomicops_cas_single_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_atomicops_ld_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_atomicops_swp_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_exclusive_pair_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_exclusive_single_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_literal_general_operands {
    pub memop: common::types::MemOp,
    pub offset: common::types::bits,
    pub signed: common::types::boolean,
    pub size: common::types::integer,
    pub t: common::types::integer,
    pub tag_checked: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_literal_simdfp_operands {
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_ordered_operands {
    pub acctype: common::types::AccType,
    pub datasize: common::types::integer,
    pub elsize: common::types::integer,
    pub memop: common::types::MemOp,
    pub n: common::types::integer,
    pub regsize: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
    pub tag_checked: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_ordered_rcpc_operands {
    pub n: common::types::integer,
    pub s: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_pair_general_no_alloc_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_pair_general_post_idx_operands {
    pub acctype: common::types::AccType,
    pub datasize: common::types::integer,
    pub memop: common::types::MemOp,
    pub n: common::types::integer,
    pub offset: common::types::bits,
    pub postindex: common::types::boolean,
    pub scale: common::types::integer,
    pub signed: common::types::boolean,
    pub t: common::types::integer,
    pub t2: common::types::integer,
    pub tag_checked: common::types::boolean,
    pub wback: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_pair_simdfp_no_alloc_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_pair_simdfp_post_idx_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
    pub t2: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_signed_offset_lda_stl_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_signed_offset_normal_operands {
    pub acctype: common::types::AccType,
    pub datasize: common::types::integer,
    pub memop: common::types::MemOp,
    pub n: common::types::integer,
    pub offset: common::types::bits,
    pub postindex: common::types::boolean,
    pub regsize: common::types::integer,
    pub scale: common::types::integer,
    pub signed: common::types::boolean,
    pub t: common::types::integer,
    pub tag_checked: common::types::boolean,
    pub wback: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_signed_offset_unpriv_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_signed_pac_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_signed_post_idx_operands {
    pub acctype: common::types::AccType,
    pub datasize: common::types::integer,
    pub memop: common::types::MemOp,
    pub n: common::types::integer,
    pub offset: common::types::bits,
    pub postindex: common::types::boolean,
    pub regsize: common::types::integer,
    pub scale: common::types::integer,
    pub signed: common::types::boolean,
    pub t: common::types::integer,
    pub tag_checked: common::types::boolean,
    pub wback: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_immediate_unsigned_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_general_register_operands {
    pub acctype: common::types::AccType,
    pub datasize: common::types::integer,
    pub extend_type: common::types::ExtendType,
    pub m: common::types::integer,
    pub memop: common::types::MemOp,
    pub n: common::types::integer,
    pub postindex: common::types::boolean,
    pub regsize: common::types::integer,
    pub scale: common::types::integer,
    pub shift: common::types::integer,
    pub signed: common::types::boolean,
    pub t: common::types::integer,
    pub tag_checked: common::types::boolean,
    pub wback: common::types::boolean,
}
#[derive(Debug)]
pub struct aarch64_memory_single_simdfp_immediate_signed_offset_normal_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_simdfp_immediate_signed_post_idx_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_single_simdfp_register_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_vector_multiple_no_wb_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_memory_vector_single_no_wb_operands {
    pub n: common::types::integer,
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_system_barriers_dmb_operands {}
#[derive(Debug)]
pub struct aarch64_system_barriers_dsb_operands {}
#[derive(Debug)]
pub struct aarch64_system_barriers_isb_operands {}
#[derive(Debug)]
pub struct aarch64_system_barriers_pssbb_operands {}
#[derive(Debug)]
pub struct aarch64_system_barriers_sb_operands {}
#[derive(Debug)]
pub struct aarch64_system_barriers_ssbb_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_debug_breakpoint_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_debug_exception_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_debug_halt_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_runtime_hvc_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_runtime_smc_operands {}
#[derive(Debug)]
pub struct aarch64_system_exceptions_runtime_svc_operands {}
#[derive(Debug)]
pub struct aarch64_system_hints_operands {}
#[derive(Debug)]
pub struct aarch64_system_monitors_operands {}
#[derive(Debug)]
pub struct aarch64_system_register_cpsr_operands {}
#[derive(Debug)]
pub struct aarch64_system_register_system_operands {
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_system_sysops_operands {
    pub t: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_udf_operands {}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_add_sub_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_add_sub_wide_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_diff_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_mul_accum_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_mul_poly_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_disparate_mul_product_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_bfdot_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_dotp_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_complex_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_double_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_high_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_element_mul_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_fp16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_fp_complex_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_diff_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_div_fp16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_logical_and_orr_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_max_min_pair_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_max_min_single_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_accum_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_dotp_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_mul_int_product_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_shift_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_sub_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_add_pairwise_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_add_saturating_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_clsz_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_cnt_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_diff_neg_fp16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_diff_neg_int_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_diff_neg_sat_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_extract_nosat_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_extract_sat_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_float_narrow_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_float_round_frint_32_64_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_float_widen_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_float_xtn_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_fp16_round_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_not_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_rbit_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_rev_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_shift_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_frecpx_fp16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_recip_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_sqrt_est_int_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_arithmetic_unary_special_sqrt_fp16_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_bfmmla_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_aes_mix_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_aes_round_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha2op_sha1_hash_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha2op_sha1_sched1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha2op_sha256_sched0_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3_bcax_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3_eor3_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3_rax1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3_xar_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha1_hash_choose_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha1_hash_majority_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha1_hash_parity_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha1_sched0_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha256_hash_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha3op_sha256_sched1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha512_sha512h_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha512_sha512h2_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha512_sha512su0_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sha512_sha512su1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3partw1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3partw2_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3ss1_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3tt1a_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3tt1b_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3tt2a_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm3_sm3tt2b_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm4_sm4enc_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_crypto_sm4_sm4enckey_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_cvt_bf16_scalar_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_cvt_bf16_vector_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_fp16_movi_operands {}
#[derive(Debug)]
pub struct aarch64_vector_logical_operands {}
#[derive(Debug)]
pub struct aarch64_vector_reduce_add_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_add_simd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_add_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_fp16_add_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_fp16_max_simd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_fp16_max_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_fp16_maxnm_simd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_fp16_maxnm_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_reduce_int_max_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_conv_float_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_conv_int_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_left_insert_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_left_long_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_left_sat_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_left_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_right_insert_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_right_narrow_logical_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_right_narrow_nonuniform_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_right_narrow_uniform_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_shift_right_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_integer_dup_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_integer_insert_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_integer_move_signed_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_integer_move_unsigned_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_cpy_dup_sisd_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_extract_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_insert_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_permute_transpose_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_permute_unzip_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_permute_zip_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug)]
pub struct aarch64_vector_transfer_vector_table_operands {
    pub d: common::types::integer,
    pub n: common::types::integer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum AccType {
    AccType_NORMAL,
    AccType_VEC,
    AccType_STREAM,
    AccType_VECSTREAM,
    AccType_ATOMIC,
    AccType_ATOMICRW,
    AccType_ORDERED,
    AccType_ORDEREDRW,
    AccType_ORDEREDATOMIC,
    AccType_ORDEREDATOMICRW,
    AccType_LIMITEDORDERED,
    AccType_UNPRIV,
    AccType_IFETCH,
    AccType_PTW,
    AccType_NONFAULT,
    AccType_CNOTFIRST,
    AccType_NV2REGISTER,
    AccType_DC,
    AccType_DC_UNPRIV,
    AccType_IC,
    AccType_DCZVA,
    AccType_AT,
}
impl Default for AccType {
    fn default() -> Self {
        AccType::AccType_NORMAL
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum ArchVersion {
    ARMv8p0,
    ARMv8p1,
    ARMv8p2,
    ARMv8p3,
    ARMv8p4,
    ARMv8p5,
    ARMv8p6,
}
impl Default for ArchVersion {
    fn default() -> Self {
        ArchVersion::ARMv8p0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum BranchType {
    BranchType_DIRCALL,
    BranchType_INDCALL,
    BranchType_ERET,
    BranchType_DBGEXIT,
    BranchType_RET,
    BranchType_DIR,
    BranchType_INDIR,
    BranchType_EXCEPTION,
    BranchType_RESET,
    BranchType_UNKNOWN,
}
impl Default for BranchType {
    fn default() -> Self {
        BranchType::BranchType_DIRCALL
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum CompareOp {
    CompareOp_GT,
    CompareOp_GE,
    CompareOp_EQ,
    CompareOp_LE,
    CompareOp_LT,
}
impl Default for CompareOp {
    fn default() -> Self {
        CompareOp::CompareOp_GT
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum Constraint {
    Constraint_NONE,
    Constraint_UNKNOWN,
    Constraint_UNDEF,
    Constraint_UNDEFEL0,
    Constraint_NOP,
    Constraint_TRUE,
    Constraint_FALSE,
    Constraint_DISABLED,
    Constraint_UNCOND,
    Constraint_COND,
    Constraint_ADDITIONAL_DECODE,
    Constraint_WBSUPPRESS,
    Constraint_FAULT,
    Constraint_FORCE,
    Constraint_FORCENOSLCHECK,
}
impl Default for Constraint {
    fn default() -> Self {
        Constraint::Constraint_NONE
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum CountOp {
    CountOp_CLZ,
    CountOp_CLS,
    CountOp_CNT,
}
impl Default for CountOp {
    fn default() -> Self {
        CountOp::CountOp_CLZ
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum CrossTriggerIn {
    CrossTriggerIn_CrossHalt,
    CrossTriggerIn_PMUOverflow,
    CrossTriggerIn_RSVD2,
    CrossTriggerIn_RSVD3,
    CrossTriggerIn_TraceExtOut0,
    CrossTriggerIn_TraceExtOut1,
    CrossTriggerIn_TraceExtOut2,
    CrossTriggerIn_TraceExtOut3,
}
impl Default for CrossTriggerIn {
    fn default() -> Self {
        CrossTriggerIn::CrossTriggerIn_CrossHalt
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum CrossTriggerOut {
    CrossTriggerOut_DebugRequest,
    CrossTriggerOut_RestartRequest,
    CrossTriggerOut_IRQ,
    CrossTriggerOut_RSVD3,
    CrossTriggerOut_TraceExtIn0,
    CrossTriggerOut_TraceExtIn1,
    CrossTriggerOut_TraceExtIn2,
    CrossTriggerOut_TraceExtIn3,
}
impl Default for CrossTriggerOut {
    fn default() -> Self {
        CrossTriggerOut::CrossTriggerOut_DebugRequest
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum DeviceType {
    DeviceType_GRE,
    DeviceType_nGRE,
    DeviceType_nGnRE,
    DeviceType_nGnRnE,
}
impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::DeviceType_GRE
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum Exception {
    Exception_Uncategorized,
    Exception_WFxTrap,
    Exception_CP15RTTrap,
    Exception_CP15RRTTrap,
    Exception_CP14RTTrap,
    Exception_CP14DTTrap,
    Exception_AdvSIMDFPAccessTrap,
    Exception_FPIDTrap,
    Exception_PACTrap,
    Exception_CP14RRTTrap,
    Exception_IllegalState,
    Exception_SupervisorCall,
    Exception_HypervisorCall,
    Exception_MonitorCall,
    Exception_SystemRegisterTrap,
    Exception_ERetTrap,
    Exception_InstructionAbort,
    Exception_PCAlignment,
    Exception_DataAbort,
    Exception_NV2DataAbort,
    Exception_PACFail,
    Exception_SPAlignment,
    Exception_FPTrappedException,
    Exception_SError,
    Exception_Breakpoint,
    Exception_SoftwareStep,
    Exception_Watchpoint,
    Exception_NV2Watchpoint,
    Exception_SoftwareBreakpoint,
    Exception_VectorCatch,
    Exception_IRQ,
    Exception_SVEAccessTrap,
    Exception_BranchTarget,
    Exception_FIQ,
}
impl Default for Exception {
    fn default() -> Self {
        Exception::Exception_Uncategorized
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum ExtendType {
    ExtendType_SXTB,
    ExtendType_SXTH,
    ExtendType_SXTW,
    ExtendType_SXTX,
    ExtendType_UXTB,
    ExtendType_UXTH,
    ExtendType_UXTW,
    ExtendType_UXTX,
}
impl Default for ExtendType {
    fn default() -> Self {
        ExtendType::ExtendType_SXTB
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPConvOp {
    FPConvOp_CVT_FtoI,
    FPConvOp_CVT_ItoF,
    FPConvOp_MOV_FtoI,
    FPConvOp_MOV_ItoF,
    FPConvOp_CVT_FtoI_JS,
}
impl Default for FPConvOp {
    fn default() -> Self {
        FPConvOp::FPConvOp_CVT_FtoI
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPExc {
    FPExc_InvalidOp,
    FPExc_DivideByZero,
    FPExc_Overflow,
    FPExc_Underflow,
    FPExc_Inexact,
    FPExc_InputDenorm,
}
impl Default for FPExc {
    fn default() -> Self {
        FPExc::FPExc_InvalidOp
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPMaxMinOp {
    FPMaxMinOp_MAX,
    FPMaxMinOp_MIN,
    FPMaxMinOp_MAXNUM,
    FPMaxMinOp_MINNUM,
}
impl Default for FPMaxMinOp {
    fn default() -> Self {
        FPMaxMinOp::FPMaxMinOp_MAX
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPRounding {
    FPRounding_TIEEVEN,
    FPRounding_POSINF,
    FPRounding_NEGINF,
    FPRounding_ZERO,
    FPRounding_TIEAWAY,
    FPRounding_ODD,
}
impl Default for FPRounding {
    fn default() -> Self {
        FPRounding::FPRounding_TIEEVEN
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPType {
    FPType_Nonzero,
    FPType_Zero,
    FPType_Infinity,
    FPType_QNaN,
    FPType_SNaN,
}
impl Default for FPType {
    fn default() -> Self {
        FPType::FPType_Nonzero
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum FPUnaryOp {
    FPUnaryOp_ABS,
    FPUnaryOp_MOV,
    FPUnaryOp_NEG,
    FPUnaryOp_SQRT,
}
impl Default for FPUnaryOp {
    fn default() -> Self {
        FPUnaryOp::FPUnaryOp_ABS
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum Fault {
    Fault_None,
    Fault_AccessFlag,
    Fault_Alignment,
    Fault_Background,
    Fault_Domain,
    Fault_Permission,
    Fault_Translation,
    Fault_AddressSize,
    Fault_SyncExternal,
    Fault_SyncExternalOnWalk,
    Fault_SyncParity,
    Fault_SyncParityOnWalk,
    Fault_AsyncParity,
    Fault_AsyncExternal,
    Fault_Debug,
    Fault_TLBConflict,
    Fault_BranchTarget,
    Fault_HWUpdateAccessFlag,
    Fault_Lockdown,
    Fault_Exclusive,
    Fault_ICacheMaint,
}
impl Default for Fault {
    fn default() -> Self {
        Fault::Fault_None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum ImmediateOp {
    ImmediateOp_MOVI,
    ImmediateOp_MVNI,
    ImmediateOp_ORR,
    ImmediateOp_BIC,
}
impl Default for ImmediateOp {
    fn default() -> Self {
        ImmediateOp::ImmediateOp_MOVI
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum InstrSet {
    InstrSet_A64,
    InstrSet_A32,
    InstrSet_T32,
}
impl Default for InstrSet {
    fn default() -> Self {
        InstrSet::InstrSet_A64
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum InterruptID {
    InterruptID_PMUIRQ,
    InterruptID_COMMIRQ,
    InterruptID_CTIIRQ,
    InterruptID_COMMRX,
    InterruptID_COMMTX,
}
impl Default for InterruptID {
    fn default() -> Self {
        InterruptID::InterruptID_PMUIRQ
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum LogicalOp {
    LogicalOp_AND,
    LogicalOp_EOR,
    LogicalOp_ORR,
}
impl Default for LogicalOp {
    fn default() -> Self {
        LogicalOp::LogicalOp_AND
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MBReqDomain {
    MBReqDomain_Nonshareable,
    MBReqDomain_InnerShareable,
    MBReqDomain_OuterShareable,
    MBReqDomain_FullSystem,
}
impl Default for MBReqDomain {
    fn default() -> Self {
        MBReqDomain::MBReqDomain_Nonshareable
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MBReqTypes {
    MBReqTypes_Reads,
    MBReqTypes_Writes,
    MBReqTypes_All,
}
impl Default for MBReqTypes {
    fn default() -> Self {
        MBReqTypes::MBReqTypes_Reads
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MemAtomicOp {
    MemAtomicOp_ADD,
    MemAtomicOp_BIC,
    MemAtomicOp_EOR,
    MemAtomicOp_ORR,
    MemAtomicOp_SMAX,
    MemAtomicOp_SMIN,
    MemAtomicOp_UMAX,
    MemAtomicOp_UMIN,
    MemAtomicOp_SWP,
}
impl Default for MemAtomicOp {
    fn default() -> Self {
        MemAtomicOp::MemAtomicOp_ADD
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MemBarrierOp {
    MemBarrierOp_DSB,
    MemBarrierOp_DMB,
    MemBarrierOp_ISB,
    MemBarrierOp_SSBB,
    MemBarrierOp_PSSBB,
    MemBarrierOp_SB,
}
impl Default for MemBarrierOp {
    fn default() -> Self {
        MemBarrierOp::MemBarrierOp_DSB
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MemOp {
    MemOp_LOAD,
    MemOp_STORE,
    MemOp_PREFETCH,
}
impl Default for MemOp {
    fn default() -> Self {
        MemOp::MemOp_LOAD
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MemType {
    MemType_Normal,
    MemType_Device,
}
impl Default for MemType {
    fn default() -> Self {
        MemType::MemType_Normal
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum MoveWideOp {
    MoveWideOp_N,
    MoveWideOp_Z,
    MoveWideOp_K,
}
impl Default for MoveWideOp {
    fn default() -> Self {
        MoveWideOp::MoveWideOp_N
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum OpType {
    OpType_Load,
    OpType_Store,
    OpType_LoadAtomic,
    OpType_Branch,
    OpType_Other,
}
impl Default for OpType {
    fn default() -> Self {
        OpType::OpType_Load
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum PSTATEField {
    PSTATEField_DAIFSet,
    PSTATEField_DAIFClr,
    PSTATEField_PAN,
    PSTATEField_UAO,
    PSTATEField_DIT,
    PSTATEField_SSBS,
    PSTATEField_TCO,
    PSTATEField_SP,
}
impl Default for PSTATEField {
    fn default() -> Self {
        PSTATEField::PSTATEField_DAIFSet
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum PrefetchHint {
    Prefetch_READ,
    Prefetch_WRITE,
    Prefetch_EXEC,
}
impl Default for PrefetchHint {
    fn default() -> Self {
        PrefetchHint::Prefetch_READ
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum PrivilegeLevel {
    PL3,
    PL2,
    PL1,
    PL0,
}
impl Default for PrivilegeLevel {
    fn default() -> Self {
        PrivilegeLevel::PL3
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum ReduceOp {
    ReduceOp_FMINNUM,
    ReduceOp_FMAXNUM,
    ReduceOp_FMIN,
    ReduceOp_FMAX,
    ReduceOp_FADD,
    ReduceOp_ADD,
}
impl Default for ReduceOp {
    fn default() -> Self {
        ReduceOp::ReduceOp_FMINNUM
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum SRType {
    SRType_LSL,
    SRType_LSR,
    SRType_ASR,
    SRType_ROR,
    SRType_RRX,
}
impl Default for SRType {
    fn default() -> Self {
        SRType::SRType_LSL
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum SVECmp {
    Cmp_EQ,
    Cmp_NE,
    Cmp_GE,
    Cmp_GT,
    Cmp_LT,
    Cmp_LE,
    Cmp_UN,
}
impl Default for SVECmp {
    fn default() -> Self {
        SVECmp::Cmp_EQ
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum ShiftType {
    ShiftType_LSL,
    ShiftType_LSR,
    ShiftType_ASR,
    ShiftType_ROR,
}
impl Default for ShiftType {
    fn default() -> Self {
        ShiftType::ShiftType_LSL
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum SysRegAccess {
    SysRegAccess_OK,
    SysRegAccess_UNDEFINED,
    SysRegAccess_TrapToEL1,
    SysRegAccess_TrapToEL2,
    SysRegAccess_TrapToEL3,
}
impl Default for SysRegAccess {
    fn default() -> Self {
        SysRegAccess::SysRegAccess_OK
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum SystemHintOp {
    SystemHintOp_NOP,
    SystemHintOp_YIELD,
    SystemHintOp_WFE,
    SystemHintOp_WFI,
    SystemHintOp_SEV,
    SystemHintOp_SEVL,
    SystemHintOp_DGH,
    SystemHintOp_ESB,
    SystemHintOp_PSB,
    SystemHintOp_TSB,
    SystemHintOp_BTI,
    SystemHintOp_CSDB,
}
impl Default for SystemHintOp {
    fn default() -> Self {
        SystemHintOp::SystemHintOp_NOP
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum SystemOp {
    Sys_AT,
    Sys_DC,
    Sys_IC,
    Sys_TLBI,
    Sys_SYS,
}
impl Default for SystemOp {
    fn default() -> Self {
        SystemOp::Sys_AT
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum TimeStamp {
    TimeStamp_None,
    TimeStamp_CoreSight,
    TimeStamp_Virtual,
    TimeStamp_Physical,
}
impl Default for TimeStamp {
    fn default() -> Self {
        TimeStamp::TimeStamp_None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum Unpredictable {
    Unpredictable_WBOVERLAPLD,
    Unpredictable_WBOVERLAPST,
    Unpredictable_LDPOVERLAP,
    Unpredictable_BASEOVERLAP,
    Unpredictable_DATAOVERLAP,
    Unpredictable_DEVPAGE2,
    Unpredictable_INSTRDEVICE,
    Unpredictable_RESCPACR,
    Unpredictable_RESMAIR,
    Unpredictable_RESTEXCB,
    Unpredictable_RESPRRR,
    Unpredictable_RESDACR,
    Unpredictable_RESVTCRS,
    Unpredictable_RESTnSZ,
    Unpredictable_RESTCF,
    Unpredictable_OORTnSZ,
    Unpredictable_LARGEIPA,
    Unpredictable_ESRCONDPASS,
    Unpredictable_ILZEROIT,
    Unpredictable_ILZEROT,
    Unpredictable_BPVECTORCATCHPRI,
    Unpredictable_VCMATCHHALF,
    Unpredictable_VCMATCHDAPA,
    Unpredictable_WPMASKANDBAS,
    Unpredictable_WPBASCONTIGUOUS,
    Unpredictable_RESWPMASK,
    Unpredictable_WPMASKEDBITS,
    Unpredictable_RESBPWPCTRL,
    Unpredictable_BPNOTIMPL,
    Unpredictable_RESBPTYPE,
    Unpredictable_BPNOTCTXCMP,
    Unpredictable_BPMATCHHALF,
    Unpredictable_BPMISMATCHHALF,
    Unpredictable_RESTARTALIGNPC,
    Unpredictable_RESTARTZEROUPPERPC,
    Unpredictable_ZEROUPPER,
    Unpredictable_ERETZEROUPPERPC,
    Unpredictable_A32FORCEALIGNPC,
    Unpredictable_SMD,
    Unpredictable_NONFAULT,
    Unpredictable_SVEZEROUPPER,
    Unpredictable_SVELDNFDATA,
    Unpredictable_SVELDNFZERO,
    Unpredictable_AFUPDATE,
    Unpredictable_IESBinDebug,
    Unpredictable_BADPMSFCR,
    Unpredictable_ZEROBTYPE,
    Unpredictable_EL2TIMESTAMP,
    Unpredictable_EL1TIMESTAMP,
    Unpredictable_CLEARERRITEZERO,
}
impl Default for Unpredictable {
    fn default() -> Self {
        Unpredictable::Unpredictable_WBOVERLAPLD
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum VBitOp {
    VBitOp_VBIF,
    VBitOp_VBIT,
    VBitOp_VBSL,
    VBitOp_VEOR,
}
impl Default for VBitOp {
    fn default() -> Self {
        VBitOp::VBitOp_VBIF
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum VBitOps {
    VBitOps_VBIF,
    VBitOps_VBIT,
    VBitOps_VBSL,
}
impl Default for VBitOps {
    fn default() -> Self {
        VBitOps::VBitOps_VBIF
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum VCGEtype {
    VCGEtype_signed,
    VCGEtype_unsigned,
    VCGEtype_fp,
}
impl Default for VCGEtype {
    fn default() -> Self {
        VCGEtype::VCGEtype_signed
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum VCGTtype {
    VCGTtype_signed,
    VCGTtype_unsigned,
    VCGTtype_fp,
}
impl Default for VCGTtype {
    fn default() -> Self {
        VCGTtype::VCGTtype_signed
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum VFPNegMul {
    VFPNegMul_VNMLA,
    VFPNegMul_VNMLS,
    VFPNegMul_VNMUL,
}
impl Default for VFPNegMul {
    fn default() -> Self {
        VFPNegMul::VFPNegMul_VNMLA
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum __InstrEnc {
    __A64,
    __A32,
    __T16,
    __T32,
}
impl Default for __InstrEnc {
    fn default() -> Self {
        __InstrEnc::__A64
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum boolean {
    FALSE,
    TRUE,
}
impl Default for boolean {
    fn default() -> Self {
        boolean::FALSE
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
#[repr(u8)]
pub enum signal {
    LOW,
    HIGH,
}
impl Default for signal {
    fn default() -> Self {
        signal::LOW
    }
}
pub static DebugException_BKPT: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("0011")?) });
pub static DebugException_Breakpoint: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("0001")?) });
pub static DebugException_VectorCatch: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("0101")?) });
pub static DebugException_Watchpoint: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("1010")?) });
pub static DebugHalt_Breakpoint: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("000111")?) });
pub static DebugHalt_EDBGRQ: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("010011")?) });
pub static DebugHalt_ExceptionCatch: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("110111")?) });
pub static DebugHalt_HaltInstruction: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("101111")?) });
pub static DebugHalt_OSUnlockCatch: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("100011")?) });
pub static DebugHalt_ResetCatch: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("100111")?) });
pub static DebugHalt_SoftwareAccess: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("110011")?) });
pub static DebugHalt_Step_Exclusive: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("011111")?) });
pub static DebugHalt_Step_NoSyndrome: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("111011")?) });
pub static DebugHalt_Step_Normal: Lazy<
    Result<common::types::bits, AArch64LifterError>,
> = Lazy::new(|| { Ok(common::types::bits::from_bits_literal("011011")?) });
pub static DebugHalt_Watchpoint: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("101011")?) });
pub static DefaultPARTID: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{
    Ok(
        common::types::integer::from(0)
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(common::types::integer::from(15)) + 1
                    - integer_to_usize!(common::types::integer::from(0)),
            )?,
    )
});
pub static DefaultPMG: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{
    Ok(
        common::types::integer::from(0)
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(common::types::integer::from(7)) + 1
                    - integer_to_usize!(common::types::integer::from(0)),
            )?,
    )
});
pub static EL0: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("00")?)
});
pub static EL1: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("01")?)
});
pub static EL2: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("10")?)
});
pub static EL3: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("11")?)
});
pub static LOG2_TAG_GRANULE: Lazy<Result<common::types::integer, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::integer::from(4)) });
pub static M32_Abort: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("10111")?) });
pub static M32_FIQ: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("10001")?)
});
pub static M32_Hyp: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("11010")?)
});
pub static M32_IRQ: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("10010")?)
});
pub static M32_Monitor: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("10110")?) });
pub static M32_Svc: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(|| {
    Ok(common::types::bits::from_bits_literal("10011")?)
});
pub static M32_System: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("11111")?) });
pub static M32_Undef: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("11011")?) });
pub static M32_User: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("10000")?) });
pub static MAX_PL: Lazy<Result<common::types::integer, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::integer::from(256)) });
pub static MAX_VL: Lazy<Result<common::types::integer, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::integer::from(2048)) });
pub static MemAttr_NC: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("00")?) });
pub static MemAttr_WB: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("11")?) });
pub static MemAttr_WT: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("10")?) });
pub static MemHint_No: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("00")?) });
pub static MemHint_RA: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("10")?) });
pub static MemHint_RWA: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("11")?) });
pub static MemHint_WA: Lazy<Result<common::types::bits, AArch64LifterError>> = Lazy::new(||
{ Ok(common::types::bits::from_bits_literal("01")?) });
pub static TAG_GRANULE: Lazy<Result<common::types::integer, AArch64LifterError>> = Lazy::new(||
{
    Ok(
        decode::helpers::pow_int_int_0(
            common::types::integer::from(2),
            common::types::LOG2_TAG_GRANULE.clone()?,
        )?,
    )
});
