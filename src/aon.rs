#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aon: Aon,
    aon_common: AonCommon,
    aon_misc: AonMisc,
    _reserved3: [u8; 0x04],
    bg_sys_top: BgSysTop,
    dcdc18_top_0: Dcdc18Top0,
    dcdc18_top_1: Dcdc18Top1,
    ldo11soc_and_dctest: Ldo11socAndDctest,
    psw_irrcv: PswIrrcv,
    _reserved8: [u8; 0x5c],
    rf_top_aon: RfTopAon,
    xtal_cfg: XtalCfg,
    tsen: Tsen,
    _reserved11: [u8; 0x74],
    acomp0_ctrl: Acomp0Ctrl,
    acomp1_ctrl: Acomp1Ctrl,
    acomp_ctrl: AcompCtrl,
    gpadc_reg_cmd: GpadcRegCmd,
    gpadc_reg_config1: GpadcRegConfig1,
    gpadc_reg_config2: GpadcRegConfig2,
    gpadc_reg_scn_pos1: GpadcRegScnPos1,
    gpadc_reg_scn_pos2: GpadcRegScnPos2,
    gpadc_reg_scn_neg1: GpadcRegScnNeg1,
    gpadc_reg_scn_neg2: GpadcRegScnNeg2,
    gpadc_reg_status: GpadcRegStatus,
    gpadc_reg_isr: GpadcRegIsr,
    gpadc_reg_result: GpadcRegResult,
    gpadc_reg_raw_result: GpadcRegRawResult,
    gpadc_reg_define: GpadcRegDefine,
    hbncore_resv0: HbncoreResv0,
    hbncore_resv1: HbncoreResv1,
}
impl RegisterBlock {
    #[doc = "0x00 - aon."]
    #[inline(always)]
    pub const fn aon(&self) -> &Aon {
        &self.aon
    }
    #[doc = "0x04 - aon_common."]
    #[inline(always)]
    pub const fn aon_common(&self) -> &AonCommon {
        &self.aon_common
    }
    #[doc = "0x08 - aon_misc."]
    #[inline(always)]
    pub const fn aon_misc(&self) -> &AonMisc {
        &self.aon_misc
    }
    #[doc = "0x10 - bg_sys_top."]
    #[inline(always)]
    pub const fn bg_sys_top(&self) -> &BgSysTop {
        &self.bg_sys_top
    }
    #[doc = "0x14 - dcdc18_top_0."]
    #[inline(always)]
    pub const fn dcdc18_top_0(&self) -> &Dcdc18Top0 {
        &self.dcdc18_top_0
    }
    #[doc = "0x18 - dcdc18_top_1."]
    #[inline(always)]
    pub const fn dcdc18_top_1(&self) -> &Dcdc18Top1 {
        &self.dcdc18_top_1
    }
    #[doc = "0x1c - ldo11soc_and_dctest."]
    #[inline(always)]
    pub const fn ldo11soc_and_dctest(&self) -> &Ldo11socAndDctest {
        &self.ldo11soc_and_dctest
    }
    #[doc = "0x20 - psw_irrcv."]
    #[inline(always)]
    pub const fn psw_irrcv(&self) -> &PswIrrcv {
        &self.psw_irrcv
    }
    #[doc = "0x80 - rf_top_aon."]
    #[inline(always)]
    pub const fn rf_top_aon(&self) -> &RfTopAon {
        &self.rf_top_aon
    }
    #[doc = "0x84 - xtal_cfg."]
    #[inline(always)]
    pub const fn xtal_cfg(&self) -> &XtalCfg {
        &self.xtal_cfg
    }
    #[doc = "0x88 - tsen."]
    #[inline(always)]
    pub const fn tsen(&self) -> &Tsen {
        &self.tsen
    }
    #[doc = "0x100 - acomp0_ctrl."]
    #[inline(always)]
    pub const fn acomp0_ctrl(&self) -> &Acomp0Ctrl {
        &self.acomp0_ctrl
    }
    #[doc = "0x104 - acomp1_ctrl."]
    #[inline(always)]
    pub const fn acomp1_ctrl(&self) -> &Acomp1Ctrl {
        &self.acomp1_ctrl
    }
    #[doc = "0x108 - acomp_ctrl."]
    #[inline(always)]
    pub const fn acomp_ctrl(&self) -> &AcompCtrl {
        &self.acomp_ctrl
    }
    #[doc = "0x10c - gpadc_reg_cmd."]
    #[inline(always)]
    pub const fn gpadc_reg_cmd(&self) -> &GpadcRegCmd {
        &self.gpadc_reg_cmd
    }
    #[doc = "0x110 - gpadc_reg_config1."]
    #[inline(always)]
    pub const fn gpadc_reg_config1(&self) -> &GpadcRegConfig1 {
        &self.gpadc_reg_config1
    }
    #[doc = "0x114 - gpadc_reg_config2."]
    #[inline(always)]
    pub const fn gpadc_reg_config2(&self) -> &GpadcRegConfig2 {
        &self.gpadc_reg_config2
    }
    #[doc = "0x118 - adc converation sequence 1"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_pos1(&self) -> &GpadcRegScnPos1 {
        &self.gpadc_reg_scn_pos1
    }
    #[doc = "0x11c - adc converation sequence 2"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_pos2(&self) -> &GpadcRegScnPos2 {
        &self.gpadc_reg_scn_pos2
    }
    #[doc = "0x120 - adc converation sequence 3"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_neg1(&self) -> &GpadcRegScnNeg1 {
        &self.gpadc_reg_scn_neg1
    }
    #[doc = "0x124 - adc converation sequence 4"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_neg2(&self) -> &GpadcRegScnNeg2 {
        &self.gpadc_reg_scn_neg2
    }
    #[doc = "0x128 - gpadc_reg_status."]
    #[inline(always)]
    pub const fn gpadc_reg_status(&self) -> &GpadcRegStatus {
        &self.gpadc_reg_status
    }
    #[doc = "0x12c - gpadc_reg_isr."]
    #[inline(always)]
    pub const fn gpadc_reg_isr(&self) -> &GpadcRegIsr {
        &self.gpadc_reg_isr
    }
    #[doc = "0x130 - gpadc_reg_result."]
    #[inline(always)]
    pub const fn gpadc_reg_result(&self) -> &GpadcRegResult {
        &self.gpadc_reg_result
    }
    #[doc = "0x134 - gpadc_reg_raw_result."]
    #[inline(always)]
    pub const fn gpadc_reg_raw_result(&self) -> &GpadcRegRawResult {
        &self.gpadc_reg_raw_result
    }
    #[doc = "0x138 - gpadc_reg_define."]
    #[inline(always)]
    pub const fn gpadc_reg_define(&self) -> &GpadcRegDefine {
        &self.gpadc_reg_define
    }
    #[doc = "0x13c - hbncore_resv0."]
    #[inline(always)]
    pub const fn hbncore_resv0(&self) -> &HbncoreResv0 {
        &self.hbncore_resv0
    }
    #[doc = "0x140 - hbncore_resv1."]
    #[inline(always)]
    pub const fn hbncore_resv1(&self) -> &HbncoreResv1 {
        &self.hbncore_resv1
    }
}
#[doc = "aon (rw) register accessor: aon.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon`] module"]
#[doc(alias = "aon")]
pub type Aon = crate::Reg<aon::AonSpec>;
#[doc = "aon."]
pub mod aon;
#[doc = "aon_common (rw) register accessor: aon_common.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_common::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_common::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_common`] module"]
#[doc(alias = "aon_common")]
pub type AonCommon = crate::Reg<aon_common::AonCommonSpec>;
#[doc = "aon_common."]
pub mod aon_common;
#[doc = "aon_misc (rw) register accessor: aon_misc.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_misc`] module"]
#[doc(alias = "aon_misc")]
pub type AonMisc = crate::Reg<aon_misc::AonMiscSpec>;
#[doc = "aon_misc."]
pub mod aon_misc;
#[doc = "bg_sys_top (rw) register accessor: bg_sys_top.\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_sys_top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sys_top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bg_sys_top`] module"]
#[doc(alias = "bg_sys_top")]
pub type BgSysTop = crate::Reg<bg_sys_top::BgSysTopSpec>;
#[doc = "bg_sys_top."]
pub mod bg_sys_top;
#[doc = "dcdc18_top_0 (rw) register accessor: dcdc18_top_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc18_top_0`] module"]
#[doc(alias = "dcdc18_top_0")]
pub type Dcdc18Top0 = crate::Reg<dcdc18_top_0::Dcdc18Top0Spec>;
#[doc = "dcdc18_top_0."]
pub mod dcdc18_top_0;
#[doc = "dcdc18_top_1 (rw) register accessor: dcdc18_top_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc18_top_1`] module"]
#[doc(alias = "dcdc18_top_1")]
pub type Dcdc18Top1 = crate::Reg<dcdc18_top_1::Dcdc18Top1Spec>;
#[doc = "dcdc18_top_1."]
pub mod dcdc18_top_1;
#[doc = "ldo11soc_and_dctest (rw) register accessor: ldo11soc_and_dctest.\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo11soc_and_dctest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo11soc_and_dctest`] module"]
#[doc(alias = "ldo11soc_and_dctest")]
pub type Ldo11socAndDctest = crate::Reg<ldo11soc_and_dctest::Ldo11socAndDctestSpec>;
#[doc = "ldo11soc_and_dctest."]
pub mod ldo11soc_and_dctest;
#[doc = "psw_irrcv (rw) register accessor: psw_irrcv.\n\nYou can [`read`](crate::Reg::read) this register and get [`psw_irrcv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psw_irrcv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psw_irrcv`] module"]
#[doc(alias = "psw_irrcv")]
pub type PswIrrcv = crate::Reg<psw_irrcv::PswIrrcvSpec>;
#[doc = "psw_irrcv."]
pub mod psw_irrcv;
#[doc = "rf_top_aon (rw) register accessor: rf_top_aon.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_top_aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_top_aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_top_aon`] module"]
#[doc(alias = "rf_top_aon")]
pub type RfTopAon = crate::Reg<rf_top_aon::RfTopAonSpec>;
#[doc = "rf_top_aon."]
pub mod rf_top_aon;
#[doc = "xtal_cfg (rw) register accessor: xtal_cfg.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_cfg`] module"]
#[doc(alias = "xtal_cfg")]
pub type XtalCfg = crate::Reg<xtal_cfg::XtalCfgSpec>;
#[doc = "xtal_cfg."]
pub mod xtal_cfg;
#[doc = "tsen (rw) register accessor: tsen.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsen`] module"]
#[doc(alias = "tsen")]
pub type Tsen = crate::Reg<tsen::TsenSpec>;
#[doc = "tsen."]
pub mod tsen;
#[doc = "acomp0_ctrl (rw) register accessor: acomp0_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp0_ctrl`] module"]
#[doc(alias = "acomp0_ctrl")]
pub type Acomp0Ctrl = crate::Reg<acomp0_ctrl::Acomp0CtrlSpec>;
#[doc = "acomp0_ctrl."]
pub mod acomp0_ctrl;
#[doc = "acomp1_ctrl (rw) register accessor: acomp1_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp1_ctrl`] module"]
#[doc(alias = "acomp1_ctrl")]
pub type Acomp1Ctrl = crate::Reg<acomp1_ctrl::Acomp1CtrlSpec>;
#[doc = "acomp1_ctrl."]
pub mod acomp1_ctrl;
#[doc = "acomp_ctrl (rw) register accessor: acomp_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp_ctrl`] module"]
#[doc(alias = "acomp_ctrl")]
pub type AcompCtrl = crate::Reg<acomp_ctrl::AcompCtrlSpec>;
#[doc = "acomp_ctrl."]
pub mod acomp_ctrl;
#[doc = "gpadc_reg_cmd (rw) register accessor: gpadc_reg_cmd.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_cmd`] module"]
#[doc(alias = "gpadc_reg_cmd")]
pub type GpadcRegCmd = crate::Reg<gpadc_reg_cmd::GpadcRegCmdSpec>;
#[doc = "gpadc_reg_cmd."]
pub mod gpadc_reg_cmd;
#[doc = "gpadc_reg_config1 (rw) register accessor: gpadc_reg_config1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config1`] module"]
#[doc(alias = "gpadc_reg_config1")]
pub type GpadcRegConfig1 = crate::Reg<gpadc_reg_config1::GpadcRegConfig1Spec>;
#[doc = "gpadc_reg_config1."]
pub mod gpadc_reg_config1;
#[doc = "gpadc_reg_config2 (rw) register accessor: gpadc_reg_config2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config2`] module"]
#[doc(alias = "gpadc_reg_config2")]
pub type GpadcRegConfig2 = crate::Reg<gpadc_reg_config2::GpadcRegConfig2Spec>;
#[doc = "gpadc_reg_config2."]
pub mod gpadc_reg_config2;
#[doc = "gpadc_reg_scn_pos1 (rw) register accessor: adc converation sequence 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_pos1`] module"]
#[doc(alias = "gpadc_reg_scn_pos1")]
pub type GpadcRegScnPos1 = crate::Reg<gpadc_reg_scn_pos1::GpadcRegScnPos1Spec>;
#[doc = "adc converation sequence 1"]
pub mod gpadc_reg_scn_pos1;
#[doc = "gpadc_reg_scn_pos2 (rw) register accessor: adc converation sequence 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_pos2`] module"]
#[doc(alias = "gpadc_reg_scn_pos2")]
pub type GpadcRegScnPos2 = crate::Reg<gpadc_reg_scn_pos2::GpadcRegScnPos2Spec>;
#[doc = "adc converation sequence 2"]
pub mod gpadc_reg_scn_pos2;
#[doc = "gpadc_reg_scn_neg1 (rw) register accessor: adc converation sequence 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_neg1`] module"]
#[doc(alias = "gpadc_reg_scn_neg1")]
pub type GpadcRegScnNeg1 = crate::Reg<gpadc_reg_scn_neg1::GpadcRegScnNeg1Spec>;
#[doc = "adc converation sequence 3"]
pub mod gpadc_reg_scn_neg1;
#[doc = "gpadc_reg_scn_neg2 (rw) register accessor: adc converation sequence 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_neg2`] module"]
#[doc(alias = "gpadc_reg_scn_neg2")]
pub type GpadcRegScnNeg2 = crate::Reg<gpadc_reg_scn_neg2::GpadcRegScnNeg2Spec>;
#[doc = "adc converation sequence 4"]
pub mod gpadc_reg_scn_neg2;
#[doc = "gpadc_reg_status (rw) register accessor: gpadc_reg_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_status`] module"]
#[doc(alias = "gpadc_reg_status")]
pub type GpadcRegStatus = crate::Reg<gpadc_reg_status::GpadcRegStatusSpec>;
#[doc = "gpadc_reg_status."]
pub mod gpadc_reg_status;
#[doc = "gpadc_reg_isr (rw) register accessor: gpadc_reg_isr.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_isr`] module"]
#[doc(alias = "gpadc_reg_isr")]
pub type GpadcRegIsr = crate::Reg<gpadc_reg_isr::GpadcRegIsrSpec>;
#[doc = "gpadc_reg_isr."]
pub mod gpadc_reg_isr;
#[doc = "gpadc_reg_result (rw) register accessor: gpadc_reg_result.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_result`] module"]
#[doc(alias = "gpadc_reg_result")]
pub type GpadcRegResult = crate::Reg<gpadc_reg_result::GpadcRegResultSpec>;
#[doc = "gpadc_reg_result."]
pub mod gpadc_reg_result;
#[doc = "gpadc_reg_raw_result (rw) register accessor: gpadc_reg_raw_result.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_raw_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_raw_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_raw_result`] module"]
#[doc(alias = "gpadc_reg_raw_result")]
pub type GpadcRegRawResult = crate::Reg<gpadc_reg_raw_result::GpadcRegRawResultSpec>;
#[doc = "gpadc_reg_raw_result."]
pub mod gpadc_reg_raw_result;
#[doc = "gpadc_reg_define (rw) register accessor: gpadc_reg_define.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_define::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_define::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_define`] module"]
#[doc(alias = "gpadc_reg_define")]
pub type GpadcRegDefine = crate::Reg<gpadc_reg_define::GpadcRegDefineSpec>;
#[doc = "gpadc_reg_define."]
pub mod gpadc_reg_define;
#[doc = "hbncore_resv0 (rw) register accessor: hbncore_resv0.\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv0`] module"]
#[doc(alias = "hbncore_resv0")]
pub type HbncoreResv0 = crate::Reg<hbncore_resv0::HbncoreResv0Spec>;
#[doc = "hbncore_resv0."]
pub mod hbncore_resv0;
#[doc = "hbncore_resv1 (rw) register accessor: hbncore_resv1.\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv1`] module"]
#[doc(alias = "hbncore_resv1")]
pub type HbncoreResv1 = crate::Reg<hbncore_resv1::HbncoreResv1Spec>;
#[doc = "hbncore_resv1."]
pub mod hbncore_resv1;
