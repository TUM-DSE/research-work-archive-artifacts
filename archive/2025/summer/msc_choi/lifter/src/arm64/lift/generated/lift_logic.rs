#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{lift, common};
use crate::arm64::lift::types::BlockSequencer;
use tnj::air::instructions::builder::InstructionBuilder;
pub fn generated_lift_logic(
    instruction: common::types::Instruction,
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: u64,
) -> Result<(), AArch64LifterError> {
    match instruction {
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ABS_Z_P_Z__(operands) => {
            let common::types::ABS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ABS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ADDPL_R_RI__(operands) => {
            let common::types::ADDPL_R_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADDPL_R_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ADDVL_R_RI__(operands) => {
            let common::types::ADDVL_R_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADDVL_R_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ADD_Z_P_ZZ__(operands) => {
            let common::types::ADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ADD_Z_ZI__(operands) => {
            let common::types::ADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ADD_Z_ZZ__(operands) => {
            let common::types::ADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ADR_Z_AZ_SD_same_scaled(operands) => {
            let common::types::ADR_Z_AZ_SD_same_scaled_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADR_Z_AZ_SD_same_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ANDV_R_P_Z__(operands) => {
            let common::types::ANDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ANDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::AND_P_P_PP_Z(operands) => {
            let common::types::AND_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_AND_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::AND_Z_P_ZZ__(operands) => {
            let common::types::AND_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::AND_Z_ZI__(operands) => {
            let common::types::AND_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::AND_Z_ZZ__(operands) => {
            let common::types::AND_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ASRD_Z_P_ZI__(operands) => {
            let common::types::ASRD_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASRD_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ASRR_Z_P_ZZ__(operands) => {
            let common::types::ASRR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASRR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ASR_Z_P_ZI__(operands) => {
            let common::types::ASR_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ASR_Z_P_ZW__(operands) => {
            let common::types::ASR_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ASR_Z_P_ZZ__(operands) => {
            let common::types::ASR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ASR_Z_ZI__(operands) => {
            let common::types::ASR_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ASR_Z_ZW__(operands) => {
            let common::types::ASR_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BFCVTNT_Z_P_Z_S2BF(operands) => {
            let common::types::BFCVTNT_Z_P_Z_S2BF_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BFCVTNT_Z_P_Z_S2BF(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BFCVT_Z_P_Z_S2BF(operands) => {
            let common::types::BFCVT_Z_P_Z_S2BF_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BFCVT_Z_P_Z_S2BF(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BFDOT_Z_ZZZ__(operands) => {
            let common::types::BFDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFDOT_Z_ZZZi__(operands) => {
            let common::types::BFDOT_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFDOT_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFMLALB_Z_ZZZ__(operands) => {
            let common::types::BFMLALB_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALB_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFMLALB_Z_ZZZi__(operands) => {
            let common::types::BFMLALB_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALB_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFMLALT_Z_ZZZ__(operands) => {
            let common::types::BFMLALT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFMLALT_Z_ZZZi__(operands) => {
            let common::types::BFMLALT_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALT_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BFMMLA_Z_ZZZ__(operands) => {
            let common::types::BFMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BIC_P_P_PP_Z(operands) => {
            let common::types::BIC_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BIC_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BIC_Z_P_ZZ__(operands) => {
            let common::types::BIC_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_BIC_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::BIC_Z_ZZ__(operands) => {
            let common::types::BIC_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BIC_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BRKA_P_P_P__(operands) => {
            let common::types::BRKA_P_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKA_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BRKB_P_P_P__(operands) => {
            let common::types::BRKB_P_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKB_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BRKN_P_P_PP__(operands) => {
            let common::types::BRKN_P_P_PP___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BRKN_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::BRKPA_P_P_PP__(operands) => {
            let common::types::BRKPA_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKPA_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::BRKPB_P_P_PP__(operands) => {
            let common::types::BRKPB_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKPB_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CLASTA_R_P_Z__(operands) => {
            let common::types::CLASTA_R_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLASTA_V_P_Z__(operands) => {
            let common::types::CLASTA_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLASTA_Z_P_ZZ__(operands) => {
            let common::types::CLASTA_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLASTB_R_P_Z__(operands) => {
            let common::types::CLASTB_R_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLASTB_V_P_Z__(operands) => {
            let common::types::CLASTB_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLASTB_Z_P_ZZ__(operands) => {
            let common::types::CLASTB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::CLS_Z_P_Z__(operands) => {
            let common::types::CLS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CLS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CLZ_Z_P_Z__(operands) => {
            let common::types::CLZ_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CLZ_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CMPEQ_P_P_ZI__(operands) => {
            let common::types::CMPEQ_P_P_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CMPEQ_P_P_ZW__(operands) => {
            let common::types::CMPEQ_P_P_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CMPEQ_P_P_ZZ__(operands) => {
            let common::types::CMPEQ_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CNOT_Z_P_Z__(operands) => {
            let common::types::CNOT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNOT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CNTB_R_S__(operands) => {
            let common::types::CNTB_R_S___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CNTB_R_S__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::CNTP_R_P_P__(operands) => {
            let common::types::CNTP_R_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNTP_R_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CNT_Z_P_Z__(operands) => {
            let common::types::CNT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::COMPACT_Z_P_Z__(operands) => {
            let common::types::COMPACT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_COMPACT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CPY_Z_O_I__(operands) => {
            let common::types::CPY_Z_O_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_O_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::CPY_Z_P_I__(operands) => {
            let common::types::CPY_Z_P_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::CPY_Z_P_R__(operands) => {
            let common::types::CPY_Z_P_R___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CPY_Z_P_V__(operands) => {
            let common::types::CPY_Z_P_V___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_V__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::CTERMEQ_RR__(operands) => {
            let common::types::CTERMEQ_RR___operands { n } = *operands;
            lift::generated::lift_blocks::lift_CTERMEQ_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::DECB_R_RS__(operands) => {
            let common::types::DECB_R_RS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECB_R_RS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::DECD_Z_ZS__(operands) => {
            let common::types::DECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::DECP_R_P_R__(operands) => {
            let common::types::DECP_R_P_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECP_R_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::DECP_Z_P_Z__(operands) => {
            let common::types::DECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::DUPM_Z_I__(operands) => {
            let common::types::DUPM_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_DUPM_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::DUP_Z_I__(operands) => {
            let common::types::DUP_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::DUP_Z_R__(operands) => {
            let common::types::DUP_Z_R___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_R__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::DUP_Z_Zi__(operands) => {
            let common::types::DUP_Z_Zi___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_Zi__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::EORV_R_P_Z__(operands) => {
            let common::types::EORV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EORV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::EOR_P_P_PP_Z(operands) => {
            let common::types::EOR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EOR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::EOR_Z_P_ZZ__(operands) => {
            let common::types::EOR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::EOR_Z_ZI__(operands) => {
            let common::types::EOR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::EOR_Z_ZZ__(operands) => {
            let common::types::EOR_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::EXT_Z_ZI_Des(operands) => {
            let common::types::EXT_Z_ZI_Des_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EXT_Z_ZI_Des(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FABD_Z_P_ZZ__(operands) => {
            let common::types::FABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FABS_Z_P_Z__(operands) => {
            let common::types::FABS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FABS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FACGT_P_P_ZZ__(operands) => {
            let common::types::FACGT_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FACGT_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FADDA_V_P_Z__(operands) => {
            let common::types::FADDA_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADDA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FADDV_V_P_Z__(operands) => {
            let common::types::FADDV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FADDV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FADD_Z_P_ZS__(operands) => {
            let common::types::FADD_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FADD_Z_P_ZZ__(operands) => {
            let common::types::FADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FADD_Z_ZZ__(operands) => {
            let common::types::FADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FCADD_Z_P_ZZ__(operands) => {
            let common::types::FCADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FCADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FCMEQ_P_P_Z0__(operands) => {
            let common::types::FCMEQ_P_P_Z0___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCMEQ_P_P_Z0__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FCMEQ_P_P_ZZ__(operands) => {
            let common::types::FCMEQ_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCMEQ_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FCMLA_Z_P_ZZZ__(operands) => {
            let common::types::FCMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FCMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FCMLA_Z_ZZZi_H(operands) => {
            let common::types::FCMLA_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FCMLA_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FCPY_Z_P_I__(operands) => {
            let common::types::FCPY_Z_P_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_FCPY_Z_P_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::FCVTZS_Z_P_Z_FP162H(operands) => {
            let common::types::FCVTZS_Z_P_Z_FP162H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVTZS_Z_P_Z_FP162H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FCVTZU_Z_P_Z_FP162H(operands) => {
            let common::types::FCVTZU_Z_P_Z_FP162H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVTZU_Z_P_Z_FP162H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FCVT_Z_P_Z_H2S(operands) => {
            let common::types::FCVT_Z_P_Z_H2S_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVT_Z_P_Z_H2S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FDIVR_Z_P_ZZ__(operands) => {
            let common::types::FDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FDIV_Z_P_ZZ__(operands) => {
            let common::types::FDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FDUP_Z_I__(operands) => {
            let common::types::FDUP_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_FDUP_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::FEXPA_Z_Z__(operands) => {
            let common::types::FEXPA_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FEXPA_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMAD_Z_P_ZZZ__(operands) => {
            let common::types::FMAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMAXNMV_V_P_Z__(operands) => {
            let common::types::FMAXNMV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMAXNMV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMAXNM_Z_P_ZS__(operands) => {
            let common::types::FMAXNM_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAXNM_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMAXNM_Z_P_ZZ__(operands) => {
            let common::types::FMAXNM_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAXNM_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMAXV_V_P_Z__(operands) => {
            let common::types::FMAXV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMAXV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMAX_Z_P_ZS__(operands) => {
            let common::types::FMAX_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAX_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMAX_Z_P_ZZ__(operands) => {
            let common::types::FMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMINNMV_V_P_Z__(operands) => {
            let common::types::FMINNMV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMINNMV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMINNM_Z_P_ZS__(operands) => {
            let common::types::FMINNM_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMINNM_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMINNM_Z_P_ZZ__(operands) => {
            let common::types::FMINNM_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMINNM_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMINV_V_P_Z__(operands) => {
            let common::types::FMINV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMINV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMIN_Z_P_ZS__(operands) => {
            let common::types::FMIN_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMIN_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMIN_Z_P_ZZ__(operands) => {
            let common::types::FMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMLA_Z_P_ZZZ__(operands) => {
            let common::types::FMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FMLA_Z_ZZZi_H(operands) => {
            let common::types::FMLA_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLA_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FMLS_Z_P_ZZZ__(operands) => {
            let common::types::FMLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FMLS_Z_ZZZi_H(operands) => {
            let common::types::FMLS_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLS_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FMMLA_Z_ZZZ_S(operands) => {
            let common::types::FMMLA_Z_ZZZ_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMMLA_Z_ZZZ_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FMSB_Z_P_ZZZ__(operands) => {
            let common::types::FMSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMULX_Z_P_ZZ__(operands) => {
            let common::types::FMULX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMULX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMUL_Z_P_ZS__(operands) => {
            let common::types::FMUL_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMUL_Z_P_ZZ__(operands) => {
            let common::types::FMUL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FMUL_Z_ZZ__(operands) => {
            let common::types::FMUL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FMUL_Z_ZZi_H(operands) => {
            let common::types::FMUL_Z_ZZi_H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_ZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FNEG_Z_P_Z__(operands) => {
            let common::types::FNEG_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FNEG_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FNMAD_Z_P_ZZZ__(operands) => {
            let common::types::FNMAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FNMAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FNMLA_Z_P_ZZZ__(operands) => {
            let common::types::FNMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FNMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FNMLS_Z_P_ZZZ__(operands) => {
            let common::types::FNMLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FNMLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::FNMSB_Z_P_ZZZ__(operands) => {
            let common::types::FNMSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FNMSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FRECPE_Z_Z__(operands) => {
            let common::types::FRECPE_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPE_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FRECPS_Z_ZZ__(operands) => {
            let common::types::FRECPS_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPS_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FRECPX_Z_P_Z__(operands) => {
            let common::types::FRECPX_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPX_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FRINTI_Z_P_Z__(operands) => {
            let common::types::FRINTI_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRINTI_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FRSQRTE_Z_Z__(operands) => {
            let common::types::FRSQRTE_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRSQRTE_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FRSQRTS_Z_ZZ__(operands) => {
            let common::types::FRSQRTS_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRSQRTS_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FSCALE_Z_P_ZZ__(operands) => {
            let common::types::FSCALE_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSCALE_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FSQRT_Z_P_Z__(operands) => {
            let common::types::FSQRT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FSQRT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FSUBR_Z_P_ZS__(operands) => {
            let common::types::FSUBR_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUBR_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FSUBR_Z_P_ZZ__(operands) => {
            let common::types::FSUBR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUBR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FSUB_Z_P_ZS__(operands) => {
            let common::types::FSUB_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FSUB_Z_P_ZZ__(operands) => {
            let common::types::FSUB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FSUB_Z_ZZ__(operands) => {
            let common::types::FSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FTMAD_Z_ZZI__(operands) => {
            let common::types::FTMAD_Z_ZZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FTMAD_Z_ZZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::FTSMUL_Z_ZZ__(operands) => {
            let common::types::FTSMUL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FTSMUL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::FTSSEL_Z_ZZ__(operands) => {
            let common::types::FTSSEL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FTSSEL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::INCB_R_RS__(operands) => {
            let common::types::INCB_R_RS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCB_R_RS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::INCD_Z_ZS__(operands) => {
            let common::types::INCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::INCP_R_P_R__(operands) => {
            let common::types::INCP_R_P_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCP_R_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::INCP_Z_P_Z__(operands) => {
            let common::types::INCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::INDEX_Z_II__(operands) => {
            let common::types::INDEX_Z_II___operands { d } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_II__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::INDEX_Z_IR__(operands) => {
            let common::types::INDEX_Z_IR___operands { d } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_IR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::INDEX_Z_RI__(operands) => {
            let common::types::INDEX_Z_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::INDEX_Z_RR__(operands) => {
            let common::types::INDEX_Z_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::INSR_Z_R__(operands) => {
            let common::types::INSR_Z_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INSR_Z_R__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::INSR_Z_V__(operands) => {
            let common::types::INSR_Z_V___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INSR_Z_V__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LASTA_R_P_Z__(operands) => {
            let common::types::LASTA_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTA_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LASTA_V_P_Z__(operands) => {
            let common::types::LASTA_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LASTB_R_P_Z__(operands) => {
            let common::types::LASTB_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTB_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LASTB_V_P_Z__(operands) => {
            let common::types::LASTB_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTB_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LD1B_Z_P_AI_S(operands) => {
            let common::types::LD1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1B_Z_P_BI_U8(operands) => {
            let common::types::LD1B_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1B_Z_P_BR_U8(operands) => {
            let common::types::LD1B_Z_P_BR_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BR_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LD1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1D_Z_P_AI_D(operands) => {
            let common::types::LD1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1D_Z_P_BI_U64(operands) => {
            let common::types::LD1D_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1D_Z_P_BR_U64(operands) => {
            let common::types::LD1D_Z_P_BR_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BR_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LD1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1H_Z_P_AI_S(operands) => {
            let common::types::LD1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1H_Z_P_BI_U16(operands) => {
            let common::types::LD1H_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1H_Z_P_BR_U16(operands) => {
            let common::types::LD1H_Z_P_BR_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BR_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RB_Z_P_BI_U8(operands) => {
            let common::types::LD1RB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RD_Z_P_BI_U64(operands) => {
            let common::types::LD1RD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RH_Z_P_BI_U16(operands) => {
            let common::types::LD1RH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROB_Z_P_BI_U8(operands) => {
            let common::types::LD1ROB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROB_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROB_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROB_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROD_Z_P_BI_U64(operands) => {
            let common::types::LD1ROD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROD_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROD_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROD_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROH_Z_P_BI_U16(operands) => {
            let common::types::LD1ROH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROH_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROH_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROH_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROW_Z_P_BI_U32(operands) => {
            let common::types::LD1ROW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1ROW_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROW_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROW_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQB_Z_P_BI_U8(operands) => {
            let common::types::LD1RQB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQB_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQB_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQB_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQD_Z_P_BI_U64(operands) => {
            let common::types::LD1RQD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQD_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQD_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQD_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQH_Z_P_BI_U16(operands) => {
            let common::types::LD1RQH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQH_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQH_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQH_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQW_Z_P_BI_U32(operands) => {
            let common::types::LD1RQW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RQW_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQW_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQW_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RSB_Z_P_BI_S16(operands) => {
            let common::types::LD1RSB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RSH_Z_P_BI_S32(operands) => {
            let common::types::LD1RSH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RSW_Z_P_BI_S64(operands) => {
            let common::types::LD1RSW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1RW_Z_P_BI_U32(operands) => {
            let common::types::LD1RW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SB_Z_P_AI_S(operands) => {
            let common::types::LD1SB_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SB_Z_P_BI_S16(operands) => {
            let common::types::LD1SB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SB_Z_P_BR_S16(operands) => {
            let common::types::LD1SB_Z_P_BR_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BR_S16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SB_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LD1SB_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SH_Z_P_AI_S(operands) => {
            let common::types::LD1SH_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SH_Z_P_BI_S32(operands) => {
            let common::types::LD1SH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SH_Z_P_BR_S32(operands) => {
            let common::types::LD1SH_Z_P_BR_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BR_S32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SH_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1SH_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SW_Z_P_AI_D(operands) => {
            let common::types::LD1SW_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SW_Z_P_BI_S64(operands) => {
            let common::types::LD1SW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SW_Z_P_BR_S64(operands) => {
            let common::types::LD1SW_Z_P_BR_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BR_S64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1SW_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LD1SW_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1W_Z_P_AI_S(operands) => {
            let common::types::LD1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1W_Z_P_BI_U32(operands) => {
            let common::types::LD1W_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1W_Z_P_BR_U32(operands) => {
            let common::types::LD1W_Z_P_BR_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BR_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD2W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD3W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LD4W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1B_Z_P_AI_S(operands) => {
            let common::types::LDFF1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1B_Z_P_BR_U8(operands) => {
            let common::types::LDFF1B_Z_P_BR_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_BR_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LDFF1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1D_Z_P_AI_D(operands) => {
            let common::types::LDFF1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1D_Z_P_BR_U64(operands) => {
            let common::types::LDFF1D_Z_P_BR_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_BR_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LDFF1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1H_Z_P_AI_S(operands) => {
            let common::types::LDFF1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1H_Z_P_BR_U16(operands) => {
            let common::types::LDFF1H_Z_P_BR_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_BR_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SB_Z_P_AI_S(operands) => {
            let common::types::LDFF1SB_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SB_Z_P_BR_S16(operands) => {
            let common::types::LDFF1SB_Z_P_BR_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_BR_S16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SB_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LDFF1SB_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SH_Z_P_AI_S(operands) => {
            let common::types::LDFF1SH_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SH_Z_P_BR_S32(operands) => {
            let common::types::LDFF1SH_Z_P_BR_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_BR_S32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SH_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1SH_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SW_Z_P_AI_D(operands) => {
            let common::types::LDFF1SW_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SW_Z_P_BR_S64(operands) => {
            let common::types::LDFF1SW_Z_P_BR_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_BR_S64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1SW_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LDFF1SW_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1W_Z_P_AI_S(operands) => {
            let common::types::LDFF1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1W_Z_P_BR_U32(operands) => {
            let common::types::LDFF1W_Z_P_BR_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_BR_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDFF1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1B_Z_P_BI_U8(operands) => {
            let common::types::LDNF1B_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1B_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1D_Z_P_BI_U64(operands) => {
            let common::types::LDNF1D_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1D_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1H_Z_P_BI_U16(operands) => {
            let common::types::LDNF1H_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1H_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1SB_Z_P_BI_S16(operands) => {
            let common::types::LDNF1SB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1SH_Z_P_BI_S32(operands) => {
            let common::types::LDNF1SH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1SW_Z_P_BI_S64(operands) => {
            let common::types::LDNF1SW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNF1W_Z_P_BI_U32(operands) => {
            let common::types::LDNF1W_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1W_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1B_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1B_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1D_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1D_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1H_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1H_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1W_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDNT1W_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDR_P_BI__(operands) => {
            let common::types::LDR_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDR_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LDR_Z_BI__(operands) => {
            let common::types::LDR_Z_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDR_Z_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::LSLR_Z_P_ZZ__(operands) => {
            let common::types::LSLR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSLR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSL_Z_P_ZI__(operands) => {
            let common::types::LSL_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSL_Z_P_ZW__(operands) => {
            let common::types::LSL_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSL_Z_P_ZZ__(operands) => {
            let common::types::LSL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSL_Z_ZI__(operands) => {
            let common::types::LSL_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LSL_Z_ZW__(operands) => {
            let common::types::LSL_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LSRR_Z_P_ZZ__(operands) => {
            let common::types::LSRR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSRR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSR_Z_P_ZI__(operands) => {
            let common::types::LSR_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSR_Z_P_ZW__(operands) => {
            let common::types::LSR_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSR_Z_P_ZZ__(operands) => {
            let common::types::LSR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::LSR_Z_ZI__(operands) => {
            let common::types::LSR_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::LSR_Z_ZW__(operands) => {
            let common::types::LSR_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::MAD_Z_P_ZZZ__(operands) => {
            let common::types::MAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::MLA_Z_P_ZZZ__(operands) => {
            let common::types::MLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_MLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::MLS_Z_P_ZZZ__(operands) => {
            let common::types::MLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_MLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::MOVPRFX_Z_P_Z__(operands) => {
            let common::types::MOVPRFX_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_MOVPRFX_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::MOVPRFX_Z_Z__(operands) => {
            let common::types::MOVPRFX_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_MOVPRFX_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::MSB_Z_P_ZZZ__(operands) => {
            let common::types::MSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::MUL_Z_P_ZZ__(operands) => {
            let common::types::MUL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MUL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::MUL_Z_ZI__(operands) => {
            let common::types::MUL_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MUL_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::NAND_P_P_PP_Z(operands) => {
            let common::types::NAND_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NAND_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::NEG_Z_P_Z__(operands) => {
            let common::types::NEG_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NEG_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOR_P_P_PP_Z(operands) => {
            let common::types::NOR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NOR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOT_Z_P_Z__(operands) => {
            let common::types::NOT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NOT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ORN_P_P_PP_Z(operands) => {
            let common::types::ORN_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORN_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ORR_P_P_PP_Z(operands) => {
            let common::types::ORR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ORR_Z_P_ZZ__(operands) => {
            let common::types::ORR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ORR_Z_ZI__(operands) => {
            let common::types::ORR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::ORR_Z_ZZ__(operands) => {
            let common::types::ORR_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ORV_R_P_Z__(operands) => {
            let common::types::ORV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::PFALSE_P__(operands) => {
            let common::types::PFALSE_P___operands { d } = *operands;
            lift::generated::lift_blocks::lift_PFALSE_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::PFIRST_P_P_P__(operands) => {
            let common::types::PFIRST_P_P_P___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_PFIRST_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::PNEXT_P_P_P__(operands) => {
            let common::types::PNEXT_P_P_P___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_PNEXT_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::PRFB_I_P_AI_S(operands) => {
            let common::types::PRFB_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFB_I_P_BI_S(operands) => {
            let common::types::PRFB_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFB_I_P_BR_S(operands) => {
            let common::types::PRFB_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFB_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFB_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFD_I_P_AI_S(operands) => {
            let common::types::PRFD_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFD_I_P_BI_S(operands) => {
            let common::types::PRFD_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFD_I_P_BR_S(operands) => {
            let common::types::PRFD_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFD_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFD_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFH_I_P_AI_S(operands) => {
            let common::types::PRFH_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFH_I_P_BI_S(operands) => {
            let common::types::PRFH_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFH_I_P_BR_S(operands) => {
            let common::types::PRFH_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFH_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFH_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFW_I_P_AI_S(operands) => {
            let common::types::PRFW_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFW_I_P_BI_S(operands) => {
            let common::types::PRFW_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFW_I_P_BR_S(operands) => {
            let common::types::PRFW_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PRFW_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFW_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PTEST__P_P__(operands) => {
            let common::types::PTEST__P_P___operands { n } = *operands;
            lift::generated::lift_blocks::lift_PTEST__P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::PTRUE_P_S__(operands) => {
            let common::types::PTRUE_P_S___operands { d } = *operands;
            lift::generated::lift_blocks::lift_PTRUE_P_S__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::PUNPKHI_P_P__(operands) => {
            let common::types::PUNPKHI_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_PUNPKHI_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::RBIT_Z_P_Z__(operands) => {
            let common::types::RBIT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_RBIT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::RDFFR_P_F__(operands) => {
            let common::types::RDFFR_P_F___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDFFR_P_F__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::RDFFR_P_P_F__(operands) => {
            let common::types::RDFFR_P_P_F___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDFFR_P_P_F__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::RDVL_R_I__(operands) => {
            let common::types::RDVL_R_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDVL_R_I__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::REVB_Z_Z__(operands) => {
            let common::types::REVB_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REVB_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::REV_P_P__(operands) => {
            let common::types::REV_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REV_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::REV_Z_Z__(operands) => {
            let common::types::REV_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REV_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SABD_Z_P_ZZ__(operands) => {
            let common::types::SABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SADDV_R_P_Z__(operands) => {
            let common::types::SADDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SADDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SCVTF_Z_P_Z_H2FP16(operands) => {
            let common::types::SCVTF_Z_P_Z_H2FP16_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SCVTF_Z_P_Z_H2FP16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SDIVR_Z_P_ZZ__(operands) => {
            let common::types::SDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SDIV_Z_P_ZZ__(operands) => {
            let common::types::SDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SDOT_Z_ZZZ__(operands) => {
            let common::types::SDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_SDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::SDOT_Z_ZZZi_S(operands) => {
            let common::types::SDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_SDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::SEL_P_P_PP__(operands) => {
            let common::types::SEL_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SEL_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SEL_Z_P_ZZ__(operands) => {
            let common::types::SEL_Z_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SEL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SETFFR_F__(operands) => {
            let common::types::SETFFR_F___operands {} = *operands;
            lift::generated::lift_blocks::lift_SETFFR_F__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::SMAXV_R_P_Z__(operands) => {
            let common::types::SMAXV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SMAXV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SMAX_Z_P_ZZ__(operands) => {
            let common::types::SMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SMAX_Z_ZI__(operands) => {
            let common::types::SMAX_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMAX_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SMINV_R_P_Z__(operands) => {
            let common::types::SMINV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SMINV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SMIN_Z_P_ZZ__(operands) => {
            let common::types::SMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SMIN_Z_ZI__(operands) => {
            let common::types::SMIN_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMIN_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SMMLA_Z_ZZZ__(operands) => {
            let common::types::SMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_SMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::SMULH_Z_P_ZZ__(operands) => {
            let common::types::SMULH_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMULH_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SPLICE_Z_P_ZZ_Des(operands) => {
            let common::types::SPLICE_Z_P_ZZ_Des_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SPLICE_Z_P_ZZ_Des(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQADD_Z_ZI__(operands) => {
            let common::types::SQADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQADD_Z_ZZ__(operands) => {
            let common::types::SQADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SQADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SQDECB_R_RS_SX(operands) => {
            let common::types::SQDECB_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECB_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECD_R_RS_SX(operands) => {
            let common::types::SQDECD_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECD_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECD_Z_ZS__(operands) => {
            let common::types::SQDECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECH_R_RS_SX(operands) => {
            let common::types::SQDECH_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECH_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECH_Z_ZS__(operands) => {
            let common::types::SQDECH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECP_R_P_R_SX(operands) => {
            let common::types::SQDECP_R_P_R_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECP_R_P_R_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECP_Z_P_Z__(operands) => {
            let common::types::SQDECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECW_R_RS_SX(operands) => {
            let common::types::SQDECW_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECW_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQDECW_Z_ZS__(operands) => {
            let common::types::SQDECW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCB_R_RS_SX(operands) => {
            let common::types::SQINCB_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCB_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCD_R_RS_SX(operands) => {
            let common::types::SQINCD_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCD_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCD_Z_ZS__(operands) => {
            let common::types::SQINCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCH_R_RS_SX(operands) => {
            let common::types::SQINCH_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCH_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCH_Z_ZS__(operands) => {
            let common::types::SQINCH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCP_R_P_R_SX(operands) => {
            let common::types::SQINCP_R_P_R_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCP_R_P_R_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCP_Z_P_Z__(operands) => {
            let common::types::SQINCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCW_R_RS_SX(operands) => {
            let common::types::SQINCW_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCW_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQINCW_Z_ZS__(operands) => {
            let common::types::SQINCW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQSUB_Z_ZI__(operands) => {
            let common::types::SQSUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQSUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SQSUB_Z_ZZ__(operands) => {
            let common::types::SQSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SQSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ST1B_Z_P_AI_S(operands) => {
            let common::types::ST1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1B_Z_P_BI__(operands) => {
            let common::types::ST1B_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1B_Z_P_BR__(operands) => {
            let common::types::ST1B_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::ST1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1D_Z_P_AI_D(operands) => {
            let common::types::ST1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1D_Z_P_BI__(operands) => {
            let common::types::ST1D_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1D_Z_P_BR__(operands) => {
            let common::types::ST1D_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::ST1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1H_Z_P_AI_S(operands) => {
            let common::types::ST1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1H_Z_P_BI__(operands) => {
            let common::types::ST1H_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1H_Z_P_BR__(operands) => {
            let common::types::ST1H_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::ST1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1W_Z_P_AI_S(operands) => {
            let common::types::ST1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1W_Z_P_BI__(operands) => {
            let common::types::ST1W_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1W_Z_P_BR__(operands) => {
            let common::types::ST1W_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::ST1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST2W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST3W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::ST4W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1B_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1B_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1D_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1D_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1H_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1H_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1W_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STNT1W_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STR_P_BI__(operands) => {
            let common::types::STR_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STR_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::STR_Z_BI__(operands) => {
            let common::types::STR_Z_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STR_Z_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::SUBR_Z_P_ZZ__(operands) => {
            let common::types::SUBR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUBR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SUBR_Z_ZI__(operands) => {
            let common::types::SUBR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUBR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SUB_Z_P_ZZ__(operands) => {
            let common::types::SUB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SUB_Z_ZI__(operands) => {
            let common::types::SUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::SUB_Z_ZZ__(operands) => {
            let common::types::SUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SUDOT_Z_ZZZi_S(operands) => {
            let common::types::SUDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_SUDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::SUNPKHI_Z_Z__(operands) => {
            let common::types::SUNPKHI_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SUNPKHI_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::SXTB_Z_P_Z__(operands) => {
            let common::types::SXTB_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SXTB_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::TBL_Z_ZZ_1(operands) => {
            let common::types::TBL_Z_ZZ_1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TBL_Z_ZZ_1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::TRN1_P_PP__(operands) => {
            let common::types::TRN1_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TRN1_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::TRN1_Z_ZZ__(operands) => {
            let common::types::TRN1_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TRN1_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UABD_Z_P_ZZ__(operands) => {
            let common::types::UABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UADDV_R_P_Z__(operands) => {
            let common::types::UADDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UADDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UCVTF_Z_P_Z_H2FP16(operands) => {
            let common::types::UCVTF_Z_P_Z_H2FP16_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UCVTF_Z_P_Z_H2FP16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UDIVR_Z_P_ZZ__(operands) => {
            let common::types::UDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UDIV_Z_P_ZZ__(operands) => {
            let common::types::UDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UDOT_Z_ZZZ__(operands) => {
            let common::types::UDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_UDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::UDOT_Z_ZZZi_S(operands) => {
            let common::types::UDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_UDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::UMAXV_R_P_Z__(operands) => {
            let common::types::UMAXV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UMAXV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UMAX_Z_P_ZZ__(operands) => {
            let common::types::UMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UMAX_Z_ZI__(operands) => {
            let common::types::UMAX_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMAX_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UMINV_R_P_Z__(operands) => {
            let common::types::UMINV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UMINV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UMIN_Z_P_ZZ__(operands) => {
            let common::types::UMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UMIN_Z_ZI__(operands) => {
            let common::types::UMIN_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMIN_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UMMLA_Z_ZZZ__(operands) => {
            let common::types::UMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_UMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::UMULH_Z_P_ZZ__(operands) => {
            let common::types::UMULH_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMULH_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQADD_Z_ZI__(operands) => {
            let common::types::UQADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQADD_Z_ZZ__(operands) => {
            let common::types::UQADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UQADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UQDECB_R_RS_UW(operands) => {
            let common::types::UQDECB_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECB_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECD_R_RS_UW(operands) => {
            let common::types::UQDECD_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECD_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECD_Z_ZS__(operands) => {
            let common::types::UQDECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECH_R_RS_UW(operands) => {
            let common::types::UQDECH_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECH_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECH_Z_ZS__(operands) => {
            let common::types::UQDECH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECP_R_P_R_UW(operands) => {
            let common::types::UQDECP_R_P_R_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECP_R_P_R_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECP_Z_P_Z__(operands) => {
            let common::types::UQDECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECW_R_RS_UW(operands) => {
            let common::types::UQDECW_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECW_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQDECW_Z_ZS__(operands) => {
            let common::types::UQDECW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCB_R_RS_UW(operands) => {
            let common::types::UQINCB_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCB_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCD_R_RS_UW(operands) => {
            let common::types::UQINCD_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCD_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCD_Z_ZS__(operands) => {
            let common::types::UQINCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCH_R_RS_UW(operands) => {
            let common::types::UQINCH_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCH_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCH_Z_ZS__(operands) => {
            let common::types::UQINCH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCP_R_P_R_UW(operands) => {
            let common::types::UQINCP_R_P_R_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCP_R_P_R_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCP_Z_P_Z__(operands) => {
            let common::types::UQINCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCW_R_RS_UW(operands) => {
            let common::types::UQINCW_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCW_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQINCW_Z_ZS__(operands) => {
            let common::types::UQINCW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQSUB_Z_ZI__(operands) => {
            let common::types::UQSUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQSUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                dn,
            )?;
        }
        common::types::Instruction::UQSUB_Z_ZZ__(operands) => {
            let common::types::UQSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UQSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::USDOT_Z_ZZZ_S(operands) => {
            let common::types::USDOT_Z_ZZZ_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_USDOT_Z_ZZZ_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::USDOT_Z_ZZZi_S(operands) => {
            let common::types::USDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_USDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::USMMLA_Z_ZZZ__(operands) => {
            let common::types::USMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_USMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::UUNPKHI_Z_Z__(operands) => {
            let common::types::UUNPKHI_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UUNPKHI_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UXTB_Z_P_Z__(operands) => {
            let common::types::UXTB_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UXTB_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UZP1_P_PP__(operands) => {
            let common::types::UZP1_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UZP1_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::UZP1_Z_ZZ__(operands) => {
            let common::types::UZP1_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UZP1_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::WHILELE_P_P_RR__(operands) => {
            let common::types::WHILELE_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELE_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::WHILELO_P_P_RR__(operands) => {
            let common::types::WHILELO_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELO_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::WHILELS_P_P_RR__(operands) => {
            let common::types::WHILELS_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELS_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::WHILELT_P_P_RR__(operands) => {
            let common::types::WHILELT_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELT_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::WRFFR_F_P__(operands) => {
            let common::types::WRFFR_F_P___operands { n } = *operands;
            lift::generated::lift_blocks::lift_WRFFR_F_P__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::ZIP2_P_PP__(operands) => {
            let common::types::ZIP2_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ZIP2_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::ZIP2_Z_ZZ__(operands) => {
            let common::types::ZIP2_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ZIP2_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_branch_conditional_compare(operands) => {
            let common::types::aarch64_branch_conditional_compare_operands {
                datasize,
                iszero,
                offset,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_compare(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                datasize,
                iszero,
                offset,
                t,
            )?;
        }
        common::types::Instruction::aarch64_branch_conditional_cond(operands) => {
            let common::types::aarch64_branch_conditional_cond_operands {
                condition,
                offset,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_cond(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                condition,
                offset,
            )?;
        }
        common::types::Instruction::aarch64_branch_conditional_test(operands) => {
            let common::types::aarch64_branch_conditional_test_operands {
                bit_pos,
                bit_val,
                datasize,
                offset,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_test(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                bit_pos,
                bit_val,
                datasize,
                offset,
                t,
            )?;
        }
        common::types::Instruction::aarch64_branch_unconditional_dret(operands) => {
            let common::types::aarch64_branch_unconditional_dret_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_dret(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_branch_unconditional_eret(operands) => {
            let common::types::aarch64_branch_unconditional_eret_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_eret(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_branch_unconditional_immediate(operands) => {
            let common::types::aarch64_branch_unconditional_immediate_operands {
                branch_type,
                offset,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                branch_type,
                offset,
            )?;
        }
        common::types::Instruction::aarch64_branch_unconditional_register(operands) => {
            let common::types::aarch64_branch_unconditional_register_operands {
                branch_type,
                m,
                n,
                pac,
                source_is_sp,
                use_key_a,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_register(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                branch_type,
                m,
                n,
                pac,
                source_is_sp,
                use_key_a,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_add_sub(operands) => {
            let common::types::aarch64_float_arithmetic_add_sub_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_div(operands) => {
            let common::types::aarch64_float_arithmetic_div_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_div(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_max_min(operands) => {
            let common::types::aarch64_float_arithmetic_max_min_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_max_min(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_mul_add_sub(operands) => {
            let common::types::aarch64_float_arithmetic_mul_add_sub_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_mul_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_mul_product(operands) => {
            let common::types::aarch64_float_arithmetic_mul_product_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_mul_product(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_round_frint(operands) => {
            let common::types::aarch64_float_arithmetic_round_frint_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_round_frint(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_round_frint_32_64(
            operands,
        ) => {
            let common::types::aarch64_float_arithmetic_round_frint_32_64_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_round_frint_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_arithmetic_unary(operands) => {
            let common::types::aarch64_float_arithmetic_unary_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_unary(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_compare_cond(operands) => {
            let common::types::aarch64_float_compare_cond_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_compare_cond(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_compare_uncond(operands) => {
            let common::types::aarch64_float_compare_uncond_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_compare_uncond(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_convert_fix(operands) => {
            let common::types::aarch64_float_convert_fix_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_fix(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_convert_fp(operands) => {
            let common::types::aarch64_float_convert_fp_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_fp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_convert_int(operands) => {
            let common::types::aarch64_float_convert_int_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_float_move_fp_imm(operands) => {
            let common::types::aarch64_float_move_fp_imm_operands { d } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_move_fp_imm(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::aarch64_float_move_fp_select(operands) => {
            let common::types::aarch64_float_move_fp_select_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_move_fp_select(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_carry(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_carry_operands {
                d,
                datasize,
                m,
                n,
                setflags,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_carry(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                m,
                n,
                setflags,
                sub_op,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_extendedreg(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_extendedreg_operands {
                d,
                datasize,
                extend_type,
                m,
                n,
                setflags,
                shift,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_extendedreg(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                extend_type,
                m,
                n,
                setflags,
                shift,
                sub_op,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_immediate_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_shiftedreg(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_shiftedreg_operands {
                d,
                datasize,
                m,
                n,
                setflags,
                shift_amount,
                shift_type,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_shiftedreg(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                m,
                n,
                setflags,
                shift_amount,
                shift_type,
                sub_op,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_address_pc_rel(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_address_pc_rel_operands {
                d,
                imm,
                page,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_address_pc_rel(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                imm,
                page,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_cnt(operands) => {
            let common::types::aarch64_integer_arithmetic_cnt_operands {
                d,
                datasize,
                n,
                opcode,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_cnt(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                n,
                opcode,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_div(operands) => {
            let common::types::aarch64_integer_arithmetic_div_operands {
                d,
                datasize,
                m,
                n,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_div(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                m,
                n,
                unsigned,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_uniform_add_sub(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_uniform_add_sub_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_uniform_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_widening_32_64(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_widening_32_64_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_widening_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
                unsigned,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_widening_64_128hi(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_widening_64_128hi_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_widening_64_128hi(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                unsigned,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_rbit(operands) => {
            let common::types::aarch64_integer_arithmetic_rbit_operands {
                d,
                datasize,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_rbit(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_arithmetic_rev(operands) => {
            let common::types::aarch64_integer_arithmetic_rev_operands {
                container_size,
                d,
                datasize,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_rev(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                container_size,
                d,
                datasize,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_bitfield(operands) => {
            let common::types::aarch64_integer_bitfield_operands {
                R,
                S,
                d,
                datasize,
                extend,
                inzero,
                n,
                tmask,
                wmask,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_bitfield(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                R,
                S,
                d,
                datasize,
                extend,
                inzero,
                n,
                tmask,
                wmask,
            )?;
        }
        common::types::Instruction::aarch64_integer_conditional_compare_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_conditional_compare_immediate_operands {
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_compare_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_conditional_compare_register(
            operands,
        ) => {
            let common::types::aarch64_integer_conditional_compare_register_operands {
                condition,
                datasize,
                flags,
                m,
                n,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_compare_register(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                condition,
                datasize,
                flags,
                m,
                n,
                sub_op,
            )?;
        }
        common::types::Instruction::aarch64_integer_conditional_select(operands) => {
            let common::types::aarch64_integer_conditional_select_operands {
                condition,
                d,
                datasize,
                else_inc,
                else_inv,
                m,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_select(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                condition,
                d,
                datasize,
                else_inc,
                else_inv,
                m,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_crc(operands) => {
            let common::types::aarch64_integer_crc_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_crc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_flags_axflag(operands) => {
            let common::types::aarch64_integer_flags_axflag_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_axflag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_integer_flags_cfinv(operands) => {
            let common::types::aarch64_integer_flags_cfinv_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_cfinv(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_integer_flags_rmif(operands) => {
            let common::types::aarch64_integer_flags_rmif_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_rmif(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_flags_setf(operands) => {
            let common::types::aarch64_integer_flags_setf_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_setf(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_flags_xaflag(operands) => {
            let common::types::aarch64_integer_flags_xaflag_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_xaflag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_integer_ins_ext_extract_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_ins_ext_extract_immediate_operands {
                d,
                datasize,
                lsb,
                m,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_ins_ext_extract_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                lsb,
                m,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_ins_ext_insert_movewide(
            operands,
        ) => {
            let common::types::aarch64_integer_ins_ext_insert_movewide_operands {
                d,
                datasize,
                imm,
                opcode,
                pos,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_ins_ext_insert_movewide(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                imm,
                opcode,
                pos,
            )?;
        }
        common::types::Instruction::aarch64_integer_logical_immediate(operands) => {
            let common::types::aarch64_integer_logical_immediate_operands {
                d,
                datasize,
                imm,
                n,
                op,
                setflags,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_logical_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                imm,
                n,
                op,
                setflags,
            )?;
        }
        common::types::Instruction::aarch64_integer_logical_shiftedreg(operands) => {
            let common::types::aarch64_integer_logical_shiftedreg_operands {
                d,
                datasize,
                invert,
                m,
                n,
                op,
                setflags,
                shift_amount,
                shift_type,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_logical_shiftedreg(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                invert,
                m,
                n,
                op,
                setflags,
                shift_amount,
                shift_type,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_autda_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autda_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autda_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_autdb_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autdb_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autdb_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_autia_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autia_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autia_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_autib_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autib_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autib_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_pacda_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacda_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacda_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_pacdb_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacdb_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacdb_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_pacga_dp_2src(operands) => {
            let common::types::aarch64_integer_pac_pacga_dp_2src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacga_dp_2src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_pacia_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacia_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacia_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_pacib_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacib_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacib_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_pac_strip_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_strip_dp_1src_operands { d } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_strip_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
            )?;
        }
        common::types::Instruction::aarch64_integer_shift_variable(operands) => {
            let common::types::aarch64_integer_shift_variable_operands {
                d,
                datasize,
                m,
                n,
                shift_type,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_shift_variable(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                datasize,
                m,
                n,
                shift_type,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcaddtag(operands) => {
            let common::types::aarch64_integer_tags_mcaddtag_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcaddtag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcgettag(operands) => {
            let common::types::aarch64_integer_tags_mcgettag_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcgettag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcgettagarray(operands) => {
            let common::types::aarch64_integer_tags_mcgettagarray_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcgettagarray(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcinsertrandomtag(operands) => {
            let common::types::aarch64_integer_tags_mcinsertrandomtag_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcinsertrandomtag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcinserttagmask(operands) => {
            let common::types::aarch64_integer_tags_mcinserttagmask_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcinserttagmask(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettaganddatapairpost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettaganddatapairpost_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettaganddatapairpost(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagandzeroarray(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagandzeroarray_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagandzeroarray(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagandzerodatapost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagandzerodatapost_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagandzerodatapost(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagarray(operands) => {
            let common::types::aarch64_integer_tags_mcsettagarray_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagarray(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpairandzerodatapost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagpairandzerodatapost_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpairandzerodatapost(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpairpost(operands) => {
            let common::types::aarch64_integer_tags_mcsettagpairpost_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpairpost(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpost(operands) => {
            let common::types::aarch64_integer_tags_mcsettagpost_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpost(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_integer_tags_mcsubtag(operands) => {
            let common::types::aarch64_integer_tags_mcsubtag_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsubtag(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_memory_atomicops_cas_pair(operands) => {
            let common::types::aarch64_memory_atomicops_cas_pair_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_cas_pair(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_atomicops_cas_single(operands) => {
            let common::types::aarch64_memory_atomicops_cas_single_operands {
                n,
                s,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_cas_single(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_atomicops_ld(operands) => {
            let common::types::aarch64_memory_atomicops_ld_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_ld(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_atomicops_swp(operands) => {
            let common::types::aarch64_memory_atomicops_swp_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_swp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_exclusive_pair(operands) => {
            let common::types::aarch64_memory_exclusive_pair_operands { n, s, t, t2 } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_exclusive_pair(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_memory_exclusive_single(operands) => {
            let common::types::aarch64_memory_exclusive_single_operands {
                n,
                s,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_exclusive_single(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_memory_literal_general(operands) => {
            let common::types::aarch64_memory_literal_general_operands {
                memop,
                offset,
                signed,
                size,
                t,
                tag_checked,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_literal_general(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                memop,
                offset,
                signed,
                size,
                t,
                tag_checked,
            )?;
        }
        common::types::Instruction::aarch64_memory_literal_simdfp(operands) => {
            let common::types::aarch64_memory_literal_simdfp_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_literal_simdfp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_ordered(operands) => {
            let common::types::aarch64_memory_ordered_operands {
                acctype,
                datasize,
                elsize,
                memop,
                n,
                regsize,
                s,
                t,
                t2,
                tag_checked,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_ordered(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                acctype,
                datasize,
                elsize,
                memop,
                n,
                regsize,
                s,
                t,
                t2,
                tag_checked,
            )?;
        }
        common::types::Instruction::aarch64_memory_ordered_rcpc(operands) => {
            let common::types::aarch64_memory_ordered_rcpc_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_ordered_rcpc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_pair_general_no_alloc(operands) => {
            let common::types::aarch64_memory_pair_general_no_alloc_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_general_no_alloc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_memory_pair_general_post_idx(operands) => {
            let common::types::aarch64_memory_pair_general_post_idx_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                scale,
                signed,
                t,
                t2,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_general_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                scale,
                signed,
                t,
                t2,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::aarch64_memory_pair_simdfp_no_alloc(operands) => {
            let common::types::aarch64_memory_pair_simdfp_no_alloc_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_simdfp_no_alloc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_memory_pair_simdfp_post_idx(operands) => {
            let common::types::aarch64_memory_pair_simdfp_post_idx_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_simdfp_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_lda_stl(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_lda_stl_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_lda_stl(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_normal(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_normal_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_normal(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_unpriv(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_unpriv_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_unpriv(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_pac(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_pac_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_pac(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_post_idx(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_post_idx_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_unsigned(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_unsigned_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_unsigned(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_general_register(operands) => {
            let common::types::aarch64_memory_single_general_register_operands {
                acctype,
                datasize,
                extend_type,
                m,
                memop,
                n,
                postindex,
                regsize,
                scale,
                shift,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_register(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                acctype,
                datasize,
                extend_type,
                m,
                memop,
                n,
                postindex,
                regsize,
                scale,
                shift,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_simdfp_immediate_signed_offset_normal(
            operands,
        ) => {
            let common::types::aarch64_memory_single_simdfp_immediate_signed_offset_normal_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_immediate_signed_offset_normal(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_simdfp_immediate_signed_post_idx(
            operands,
        ) => {
            let common::types::aarch64_memory_single_simdfp_immediate_signed_post_idx_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_immediate_signed_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_single_simdfp_register(operands) => {
            let common::types::aarch64_memory_single_simdfp_register_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_register(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_vector_multiple_no_wb(operands) => {
            let common::types::aarch64_memory_vector_multiple_no_wb_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_vector_multiple_no_wb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_memory_vector_single_no_wb(operands) => {
            let common::types::aarch64_memory_vector_single_no_wb_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_vector_single_no_wb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                n,
                t,
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_dmb(operands) => {
            let common::types::aarch64_system_barriers_dmb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_dmb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_dsb(operands) => {
            let common::types::aarch64_system_barriers_dsb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_dsb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_isb(operands) => {
            let common::types::aarch64_system_barriers_isb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_isb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_pssbb(operands) => {
            let common::types::aarch64_system_barriers_pssbb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_pssbb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_sb(operands) => {
            let common::types::aarch64_system_barriers_sb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_sb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_barriers_ssbb(operands) => {
            let common::types::aarch64_system_barriers_ssbb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_ssbb(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_debug_breakpoint(
            operands,
        ) => {
            let common::types::aarch64_system_exceptions_debug_breakpoint_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_breakpoint(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_debug_exception(
            operands,
        ) => {
            let common::types::aarch64_system_exceptions_debug_exception_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_exception(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_debug_halt(operands) => {
            let common::types::aarch64_system_exceptions_debug_halt_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_halt(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_hvc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_hvc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_hvc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_smc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_smc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_smc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_svc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_svc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_svc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_hints(operands) => {
            let common::types::aarch64_system_hints_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_hints(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_monitors(operands) => {
            let common::types::aarch64_system_monitors_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_monitors(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_register_cpsr(operands) => {
            let common::types::aarch64_system_register_cpsr_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_register_cpsr(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_system_register_system(operands) => {
            let common::types::aarch64_system_register_system_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_register_system(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                t,
            )?;
        }
        common::types::Instruction::aarch64_system_sysops(operands) => {
            let common::types::aarch64_system_sysops_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_sysops(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                t,
            )?;
        }
        common::types::Instruction::aarch64_udf(operands) => {
            let common::types::aarch64_udf_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_udf(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_wide(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_wide_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_wide(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_diff(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_diff_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_diff(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_accum(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_accum_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_accum(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_poly(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_poly_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_poly(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_product(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_bfdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_bfdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_bfdot(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_complex(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_high_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_high_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_high_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_fp_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_fp_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_fp_complex(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_diff(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_diff_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_diff(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_div_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_div_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_div_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_logical_and_orr(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_logical_and_orr_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_logical_and_orr(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_pair(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_pair_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_pair(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_single(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_single_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_single(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_product(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp_complex(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_accum(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_accum_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_accum(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_product(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_shift_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_shift_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_shift_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_add_pairwise(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_add_pairwise_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_add_pairwise(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_add_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_add_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_add_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_clsz(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_clsz_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_clsz(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cnt(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_cnt_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cnt(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_nosat(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_nosat_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_nosat(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_sat_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_sat_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_sqxtun_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_sqxtun_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_narrow(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_narrow_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_narrow(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_round_frint_32_64(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_round_frint_32_64_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_round_frint_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_widen(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_widen_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_widen(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_xtn_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_xtn_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_xtn_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_round(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_round_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_round(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_not(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_not_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_not(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_rbit(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_rbit_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_rbit(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_rev(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_rev_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_rev(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_shift(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_shift_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_shift(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_frecpx_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_frecpx_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_frecpx_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_recip_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_recip_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_recip_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_recip_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_recip_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_est_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_est_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_est_int(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_bfmmla(operands) => {
            let common::types::aarch64_vector_bfmmla_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_bfmmla(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_aes_mix(operands) => {
            let common::types::aarch64_vector_crypto_aes_mix_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_aes_mix(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_aes_round(operands) => {
            let common::types::aarch64_vector_crypto_aes_round_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_aes_round(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha1_hash(operands) => {
            let common::types::aarch64_vector_crypto_sha2op_sha1_hash_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha1_hash(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha1_sched1(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha2op_sha1_sched1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha1_sched1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha256_sched0(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha2op_sha256_sched0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha256_sched0(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_bcax(operands) => {
            let common::types::aarch64_vector_crypto_sha3_bcax_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_bcax(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_eor3(operands) => {
            let common::types::aarch64_vector_crypto_sha3_eor3_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_eor3(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_rax1(operands) => {
            let common::types::aarch64_vector_crypto_sha3_rax1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_rax1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_xar(operands) => {
            let common::types::aarch64_vector_crypto_sha3_xar_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_xar(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_choose(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_choose_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_choose(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_majority(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_majority_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_majority(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_parity(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_parity_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_parity(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_sched0(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_sched0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_sched0(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha256_hash(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha256_hash_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha256_hash(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha256_sched1(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha256_sched1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha256_sched1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512h(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512h_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512h(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512h2(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512h2_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512h2(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512su0(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512su0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512su0(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512su1(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512su1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512su1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3partw1(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3partw1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3partw1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3partw2(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3partw2_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3partw2(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3ss1(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3ss1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3ss1(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt1a(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt1a_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt1a(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt1b(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt1b_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt1b(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt2a(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt2a_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt2a(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt2b(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt2b_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt2b(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm4_sm4enc(operands) => {
            let common::types::aarch64_vector_crypto_sm4_sm4enc_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm4_sm4enc(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_crypto_sm4_sm4enckey(operands) => {
            let common::types::aarch64_vector_crypto_sm4_sm4enckey_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm4_sm4enckey(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_cvt_bf16_scalar(operands) => {
            let common::types::aarch64_vector_cvt_bf16_scalar_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_cvt_bf16_scalar(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_cvt_bf16_vector(operands) => {
            let common::types::aarch64_vector_cvt_bf16_vector_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_cvt_bf16_vector(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_fp16_movi(operands) => {
            let common::types::aarch64_vector_fp16_movi_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_fp16_movi(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_vector_logical(operands) => {
            let common::types::aarch64_vector_logical_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_logical(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_add_long(operands) => {
            let common::types::aarch64_vector_reduce_add_long_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_add_simd(operands) => {
            let common::types::aarch64_vector_reduce_add_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_simd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_add_sisd(operands) => {
            let common::types::aarch64_vector_reduce_add_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_add_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_add_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_add_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_max_simd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_max_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_max_simd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_max_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_max_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_max_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_maxnm_simd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_maxnm_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_maxnm_simd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_maxnm_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_maxnm_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_maxnm_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_reduce_int_max(operands) => {
            let common::types::aarch64_vector_reduce_int_max_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_int_max(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_conv_float_sisd(operands) => {
            let common::types::aarch64_vector_shift_conv_float_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_conv_float_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_conv_int_sisd(operands) => {
            let common::types::aarch64_vector_shift_conv_int_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_conv_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_left_insert_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_insert_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_insert_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_left_long(operands) => {
            let common::types::aarch64_vector_shift_left_long_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_long(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_left_sat_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_sat_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_left_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_right_insert_sisd(operands) => {
            let common::types::aarch64_vector_shift_right_insert_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_insert_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_logical(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_logical_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_logical(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_nonuniform_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_nonuniform_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_nonuniform_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_uniform_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_uniform_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_uniform_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_shift_right_sisd(operands) => {
            let common::types::aarch64_vector_shift_right_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_integer_dup(operands) => {
            let common::types::aarch64_vector_transfer_integer_dup_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_dup(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_integer_insert(operands) => {
            let common::types::aarch64_vector_transfer_integer_insert_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_insert(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_integer_move_signed(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_integer_move_signed_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_move_signed(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_integer_move_unsigned(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_integer_move_unsigned_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_move_unsigned(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_cpy_dup_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_cpy_dup_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_cpy_dup_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_extract(operands) => {
            let common::types::aarch64_vector_transfer_vector_extract_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_extract(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_insert(operands) => {
            let common::types::aarch64_vector_transfer_vector_insert_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_insert(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_transpose(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_transpose_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_transpose(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_unzip(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_unzip_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_unzip(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_zip(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_zip_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_zip(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        common::types::Instruction::aarch64_vector_transfer_vector_table(operands) => {
            let common::types::aarch64_vector_transfer_vector_table_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_table(
                builder,
                sequencer,
                lift::types::Variable::from(common::types::bits::new(pc as u128, 64)),
                d,
                n,
            )?;
        }
        _ => return Err(AArch64LifterError::UnspecifiedInstruction),
    };
    Ok(())
}
