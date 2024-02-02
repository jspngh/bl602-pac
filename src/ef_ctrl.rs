#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ef_if_ctrl_0: EfIfCtrl0,
    ef_if_cyc_0: EfIfCyc0,
    ef_if_cyc_1: EfIfCyc1,
    ef_if_0_manual: EfIf0Manual,
    ef_if_0_status: EfIf0Status,
    ef_if_cfg_0: EfIfCfg0,
    ef_sw_cfg_0: EfSwCfg0,
    ef_reserved: EfReserved,
    ef_if_ana_trim_0: EfIfAnaTrim0,
    ef_if_sw_usage_0: EfIfSwUsage0,
    _reserved10: [u8; 0x01d8],
    ef_crc_ctrl_0: EfCrcCtrl0,
    ef_crc_ctrl_1: EfCrcCtrl1,
    ef_crc_ctrl_2: EfCrcCtrl2,
    ef_crc_ctrl_3: EfCrcCtrl3,
    ef_crc_ctrl_4: EfCrcCtrl4,
    ef_crc_ctrl_5: EfCrcCtrl5,
}
impl RegisterBlock {
    #[doc = "0x00 - ef_if_ctrl_0."]
    #[inline(always)]
    pub const fn ef_if_ctrl_0(&self) -> &EfIfCtrl0 {
        &self.ef_if_ctrl_0
    }
    #[doc = "0x04 - ef_if_cyc_0."]
    #[inline(always)]
    pub const fn ef_if_cyc_0(&self) -> &EfIfCyc0 {
        &self.ef_if_cyc_0
    }
    #[doc = "0x08 - ef_if_cyc_1."]
    #[inline(always)]
    pub const fn ef_if_cyc_1(&self) -> &EfIfCyc1 {
        &self.ef_if_cyc_1
    }
    #[doc = "0x0c - ef_if_0_manual."]
    #[inline(always)]
    pub const fn ef_if_0_manual(&self) -> &EfIf0Manual {
        &self.ef_if_0_manual
    }
    #[doc = "0x10 - ef_if_0_status."]
    #[inline(always)]
    pub const fn ef_if_0_status(&self) -> &EfIf0Status {
        &self.ef_if_0_status
    }
    #[doc = "0x14 - ef_if_cfg_0."]
    #[inline(always)]
    pub const fn ef_if_cfg_0(&self) -> &EfIfCfg0 {
        &self.ef_if_cfg_0
    }
    #[doc = "0x18 - ef_sw_cfg_0."]
    #[inline(always)]
    pub const fn ef_sw_cfg_0(&self) -> &EfSwCfg0 {
        &self.ef_sw_cfg_0
    }
    #[doc = "0x1c - ef_reserved."]
    #[inline(always)]
    pub const fn ef_reserved(&self) -> &EfReserved {
        &self.ef_reserved
    }
    #[doc = "0x20 - ef_if_ana_trim_0."]
    #[inline(always)]
    pub const fn ef_if_ana_trim_0(&self) -> &EfIfAnaTrim0 {
        &self.ef_if_ana_trim_0
    }
    #[doc = "0x24 - ef_if_sw_usage_0."]
    #[inline(always)]
    pub const fn ef_if_sw_usage_0(&self) -> &EfIfSwUsage0 {
        &self.ef_if_sw_usage_0
    }
    #[doc = "0x200 - ef_crc_ctrl_0."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_0(&self) -> &EfCrcCtrl0 {
        &self.ef_crc_ctrl_0
    }
    #[doc = "0x204 - ef_crc_ctrl_1."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_1(&self) -> &EfCrcCtrl1 {
        &self.ef_crc_ctrl_1
    }
    #[doc = "0x208 - ef_crc_ctrl_2."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_2(&self) -> &EfCrcCtrl2 {
        &self.ef_crc_ctrl_2
    }
    #[doc = "0x20c - ef_crc_ctrl_3."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_3(&self) -> &EfCrcCtrl3 {
        &self.ef_crc_ctrl_3
    }
    #[doc = "0x210 - ef_crc_ctrl_4."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_4(&self) -> &EfCrcCtrl4 {
        &self.ef_crc_ctrl_4
    }
    #[doc = "0x214 - ef_crc_ctrl_5."]
    #[inline(always)]
    pub const fn ef_crc_ctrl_5(&self) -> &EfCrcCtrl5 {
        &self.ef_crc_ctrl_5
    }
}
#[doc = "ef_if_ctrl_0 (rw) register accessor: ef_if_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_ctrl_0`] module"]
#[doc(alias = "ef_if_ctrl_0")]
pub type EfIfCtrl0 = crate::Reg<ef_if_ctrl_0::EfIfCtrl0Spec>;
#[doc = "ef_if_ctrl_0."]
pub mod ef_if_ctrl_0;
#[doc = "ef_if_cyc_0 (rw) register accessor: ef_if_cyc_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cyc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_cyc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_cyc_0`] module"]
#[doc(alias = "ef_if_cyc_0")]
pub type EfIfCyc0 = crate::Reg<ef_if_cyc_0::EfIfCyc0Spec>;
#[doc = "ef_if_cyc_0."]
pub mod ef_if_cyc_0;
#[doc = "ef_if_cyc_1 (rw) register accessor: ef_if_cyc_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cyc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_cyc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_cyc_1`] module"]
#[doc(alias = "ef_if_cyc_1")]
pub type EfIfCyc1 = crate::Reg<ef_if_cyc_1::EfIfCyc1Spec>;
#[doc = "ef_if_cyc_1."]
pub mod ef_if_cyc_1;
#[doc = "ef_if_0_manual (rw) register accessor: ef_if_0_manual.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_0_manual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_0_manual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_0_manual`] module"]
#[doc(alias = "ef_if_0_manual")]
pub type EfIf0Manual = crate::Reg<ef_if_0_manual::EfIf0ManualSpec>;
#[doc = "ef_if_0_manual."]
pub mod ef_if_0_manual;
#[doc = "ef_if_0_status (rw) register accessor: ef_if_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_0_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_0_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_0_status`] module"]
#[doc(alias = "ef_if_0_status")]
pub type EfIf0Status = crate::Reg<ef_if_0_status::EfIf0StatusSpec>;
#[doc = "ef_if_0_status."]
pub mod ef_if_0_status;
#[doc = "ef_if_cfg_0 (r) register accessor: ef_if_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cfg_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_cfg_0`] module"]
#[doc(alias = "ef_if_cfg_0")]
pub type EfIfCfg0 = crate::Reg<ef_if_cfg_0::EfIfCfg0Spec>;
#[doc = "ef_if_cfg_0."]
pub mod ef_if_cfg_0;
#[doc = "ef_sw_cfg_0 (rw) register accessor: ef_sw_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_cfg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_cfg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_cfg_0`] module"]
#[doc(alias = "ef_sw_cfg_0")]
pub type EfSwCfg0 = crate::Reg<ef_sw_cfg_0::EfSwCfg0Spec>;
#[doc = "ef_sw_cfg_0."]
pub mod ef_sw_cfg_0;
#[doc = "ef_reserved (rw) register accessor: ef_reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_reserved`] module"]
#[doc(alias = "ef_reserved")]
pub type EfReserved = crate::Reg<ef_reserved::EfReservedSpec>;
#[doc = "ef_reserved."]
pub mod ef_reserved;
#[doc = "ef_if_ana_trim_0 (rw) register accessor: ef_if_ana_trim_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_ana_trim_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_ana_trim_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_ana_trim_0`] module"]
#[doc(alias = "ef_if_ana_trim_0")]
pub type EfIfAnaTrim0 = crate::Reg<ef_if_ana_trim_0::EfIfAnaTrim0Spec>;
#[doc = "ef_if_ana_trim_0."]
pub mod ef_if_ana_trim_0;
#[doc = "ef_if_sw_usage_0 (rw) register accessor: ef_if_sw_usage_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_sw_usage_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_sw_usage_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_if_sw_usage_0`] module"]
#[doc(alias = "ef_if_sw_usage_0")]
pub type EfIfSwUsage0 = crate::Reg<ef_if_sw_usage_0::EfIfSwUsage0Spec>;
#[doc = "ef_if_sw_usage_0."]
pub mod ef_if_sw_usage_0;
#[doc = "ef_crc_ctrl_0 (rw) register accessor: ef_crc_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_0`] module"]
#[doc(alias = "ef_crc_ctrl_0")]
pub type EfCrcCtrl0 = crate::Reg<ef_crc_ctrl_0::EfCrcCtrl0Spec>;
#[doc = "ef_crc_ctrl_0."]
pub mod ef_crc_ctrl_0;
#[doc = "ef_crc_ctrl_1 (rw) register accessor: ef_crc_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_1`] module"]
#[doc(alias = "ef_crc_ctrl_1")]
pub type EfCrcCtrl1 = crate::Reg<ef_crc_ctrl_1::EfCrcCtrl1Spec>;
#[doc = "ef_crc_ctrl_1."]
pub mod ef_crc_ctrl_1;
#[doc = "ef_crc_ctrl_2 (rw) register accessor: ef_crc_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_2`] module"]
#[doc(alias = "ef_crc_ctrl_2")]
pub type EfCrcCtrl2 = crate::Reg<ef_crc_ctrl_2::EfCrcCtrl2Spec>;
#[doc = "ef_crc_ctrl_2."]
pub mod ef_crc_ctrl_2;
#[doc = "ef_crc_ctrl_3 (rw) register accessor: ef_crc_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_3`] module"]
#[doc(alias = "ef_crc_ctrl_3")]
pub type EfCrcCtrl3 = crate::Reg<ef_crc_ctrl_3::EfCrcCtrl3Spec>;
#[doc = "ef_crc_ctrl_3."]
pub mod ef_crc_ctrl_3;
#[doc = "ef_crc_ctrl_4 (rw) register accessor: ef_crc_ctrl_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_4`] module"]
#[doc(alias = "ef_crc_ctrl_4")]
pub type EfCrcCtrl4 = crate::Reg<ef_crc_ctrl_4::EfCrcCtrl4Spec>;
#[doc = "ef_crc_ctrl_4."]
pub mod ef_crc_ctrl_4;
#[doc = "ef_crc_ctrl_5 (rw) register accessor: ef_crc_ctrl_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_5`] module"]
#[doc(alias = "ef_crc_ctrl_5")]
pub type EfCrcCtrl5 = crate::Reg<ef_crc_ctrl_5::EfCrcCtrl5Spec>;
#[doc = "ef_crc_ctrl_5."]
pub mod ef_crc_ctrl_5;
