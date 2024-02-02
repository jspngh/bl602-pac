#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    irtx_config: IrtxConfig,
    irtx_int_sts: IrtxIntSts,
    irtx_data_word0: IrtxDataWord0,
    irtx_data_word1: IrtxDataWord1,
    irtx_pulse_width: IrtxPulseWidth,
    irtx_pw: IrtxPw,
    _reserved6: [u8; 0x28],
    irtx_swm_pw_0: IrtxSwmPw0,
    irtx_swm_pw_1: IrtxSwmPw1,
    irtx_swm_pw_2: IrtxSwmPw2,
    irtx_swm_pw_3: IrtxSwmPw3,
    irtx_swm_pw_4: IrtxSwmPw4,
    irtx_swm_pw_5: IrtxSwmPw5,
    irtx_swm_pw_6: IrtxSwmPw6,
    irtx_swm_pw_7: IrtxSwmPw7,
    _reserved14: [u8; 0x20],
    irrx_config: IrrxConfig,
    irrx_int_sts: IrrxIntSts,
    irrx_pw_config: IrrxPwConfig,
    _reserved17: [u8; 0x04],
    irrx_data_count: IrrxDataCount,
    irrx_data_word0: IrrxDataWord0,
    irrx_data_word1: IrrxDataWord1,
    _reserved20: [u8; 0x24],
    irrx_swm_fifo_config_0: IrrxSwmFifoConfig0,
    irrx_swm_fifo_rdata: IrrxSwmFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - irtx_config."]
    #[inline(always)]
    pub const fn irtx_config(&self) -> &IrtxConfig {
        &self.irtx_config
    }
    #[doc = "0x04 - irtx_int_sts."]
    #[inline(always)]
    pub const fn irtx_int_sts(&self) -> &IrtxIntSts {
        &self.irtx_int_sts
    }
    #[doc = "0x08 - irtx_data_word0."]
    #[inline(always)]
    pub const fn irtx_data_word0(&self) -> &IrtxDataWord0 {
        &self.irtx_data_word0
    }
    #[doc = "0x0c - irtx_data_word1."]
    #[inline(always)]
    pub const fn irtx_data_word1(&self) -> &IrtxDataWord1 {
        &self.irtx_data_word1
    }
    #[doc = "0x10 - irtx_pulse_width."]
    #[inline(always)]
    pub const fn irtx_pulse_width(&self) -> &IrtxPulseWidth {
        &self.irtx_pulse_width
    }
    #[doc = "0x14 - irtx_pw."]
    #[inline(always)]
    pub const fn irtx_pw(&self) -> &IrtxPw {
        &self.irtx_pw
    }
    #[doc = "0x40 - irtx_swm_pw_0."]
    #[inline(always)]
    pub const fn irtx_swm_pw_0(&self) -> &IrtxSwmPw0 {
        &self.irtx_swm_pw_0
    }
    #[doc = "0x44 - irtx_swm_pw_1."]
    #[inline(always)]
    pub const fn irtx_swm_pw_1(&self) -> &IrtxSwmPw1 {
        &self.irtx_swm_pw_1
    }
    #[doc = "0x48 - irtx_swm_pw_2."]
    #[inline(always)]
    pub const fn irtx_swm_pw_2(&self) -> &IrtxSwmPw2 {
        &self.irtx_swm_pw_2
    }
    #[doc = "0x4c - irtx_swm_pw_3."]
    #[inline(always)]
    pub const fn irtx_swm_pw_3(&self) -> &IrtxSwmPw3 {
        &self.irtx_swm_pw_3
    }
    #[doc = "0x50 - irtx_swm_pw_4."]
    #[inline(always)]
    pub const fn irtx_swm_pw_4(&self) -> &IrtxSwmPw4 {
        &self.irtx_swm_pw_4
    }
    #[doc = "0x54 - irtx_swm_pw_5."]
    #[inline(always)]
    pub const fn irtx_swm_pw_5(&self) -> &IrtxSwmPw5 {
        &self.irtx_swm_pw_5
    }
    #[doc = "0x58 - irtx_swm_pw_6."]
    #[inline(always)]
    pub const fn irtx_swm_pw_6(&self) -> &IrtxSwmPw6 {
        &self.irtx_swm_pw_6
    }
    #[doc = "0x5c - irtx_swm_pw_7."]
    #[inline(always)]
    pub const fn irtx_swm_pw_7(&self) -> &IrtxSwmPw7 {
        &self.irtx_swm_pw_7
    }
    #[doc = "0x80 - irrx_config."]
    #[inline(always)]
    pub const fn irrx_config(&self) -> &IrrxConfig {
        &self.irrx_config
    }
    #[doc = "0x84 - irrx_int_sts."]
    #[inline(always)]
    pub const fn irrx_int_sts(&self) -> &IrrxIntSts {
        &self.irrx_int_sts
    }
    #[doc = "0x88 - irrx_pw_config."]
    #[inline(always)]
    pub const fn irrx_pw_config(&self) -> &IrrxPwConfig {
        &self.irrx_pw_config
    }
    #[doc = "0x90 - irrx_data_count."]
    #[inline(always)]
    pub const fn irrx_data_count(&self) -> &IrrxDataCount {
        &self.irrx_data_count
    }
    #[doc = "0x94 - irrx_data_word0."]
    #[inline(always)]
    pub const fn irrx_data_word0(&self) -> &IrrxDataWord0 {
        &self.irrx_data_word0
    }
    #[doc = "0x98 - irrx_data_word1."]
    #[inline(always)]
    pub const fn irrx_data_word1(&self) -> &IrrxDataWord1 {
        &self.irrx_data_word1
    }
    #[doc = "0xc0 - irrx_swm_fifo_config_0."]
    #[inline(always)]
    pub const fn irrx_swm_fifo_config_0(&self) -> &IrrxSwmFifoConfig0 {
        &self.irrx_swm_fifo_config_0
    }
    #[doc = "0xc4 - irrx_swm_fifo_rdata."]
    #[inline(always)]
    pub const fn irrx_swm_fifo_rdata(&self) -> &IrrxSwmFifoRdata {
        &self.irrx_swm_fifo_rdata
    }
}
#[doc = "irtx_config (rw) register accessor: irtx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_config`] module"]
#[doc(alias = "irtx_config")]
pub type IrtxConfig = crate::Reg<irtx_config::IrtxConfigSpec>;
#[doc = "irtx_config."]
pub mod irtx_config;
#[doc = "irtx_int_sts (rw) register accessor: irtx_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_int_sts`] module"]
#[doc(alias = "irtx_int_sts")]
pub type IrtxIntSts = crate::Reg<irtx_int_sts::IrtxIntStsSpec>;
#[doc = "irtx_int_sts."]
pub mod irtx_int_sts;
#[doc = "irtx_data_word0 (rw) register accessor: irtx_data_word0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_data_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_data_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_data_word0`] module"]
#[doc(alias = "irtx_data_word0")]
pub type IrtxDataWord0 = crate::Reg<irtx_data_word0::IrtxDataWord0Spec>;
#[doc = "irtx_data_word0."]
pub mod irtx_data_word0;
#[doc = "irtx_data_word1 (rw) register accessor: irtx_data_word1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_data_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_data_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_data_word1`] module"]
#[doc(alias = "irtx_data_word1")]
pub type IrtxDataWord1 = crate::Reg<irtx_data_word1::IrtxDataWord1Spec>;
#[doc = "irtx_data_word1."]
pub mod irtx_data_word1;
#[doc = "irtx_pulse_width (rw) register accessor: irtx_pulse_width.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pulse_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pulse_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_pulse_width`] module"]
#[doc(alias = "irtx_pulse_width")]
pub type IrtxPulseWidth = crate::Reg<irtx_pulse_width::IrtxPulseWidthSpec>;
#[doc = "irtx_pulse_width."]
pub mod irtx_pulse_width;
#[doc = "irtx_pw (rw) register accessor: irtx_pw.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_pw`] module"]
#[doc(alias = "irtx_pw")]
pub type IrtxPw = crate::Reg<irtx_pw::IrtxPwSpec>;
#[doc = "irtx_pw."]
pub mod irtx_pw;
#[doc = "irtx_swm_pw_0 (rw) register accessor: irtx_swm_pw_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_0`] module"]
#[doc(alias = "irtx_swm_pw_0")]
pub type IrtxSwmPw0 = crate::Reg<irtx_swm_pw_0::IrtxSwmPw0Spec>;
#[doc = "irtx_swm_pw_0."]
pub mod irtx_swm_pw_0;
#[doc = "irtx_swm_pw_1 (rw) register accessor: irtx_swm_pw_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_1`] module"]
#[doc(alias = "irtx_swm_pw_1")]
pub type IrtxSwmPw1 = crate::Reg<irtx_swm_pw_1::IrtxSwmPw1Spec>;
#[doc = "irtx_swm_pw_1."]
pub mod irtx_swm_pw_1;
#[doc = "irtx_swm_pw_2 (rw) register accessor: irtx_swm_pw_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_2`] module"]
#[doc(alias = "irtx_swm_pw_2")]
pub type IrtxSwmPw2 = crate::Reg<irtx_swm_pw_2::IrtxSwmPw2Spec>;
#[doc = "irtx_swm_pw_2."]
pub mod irtx_swm_pw_2;
#[doc = "irtx_swm_pw_3 (rw) register accessor: irtx_swm_pw_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_3`] module"]
#[doc(alias = "irtx_swm_pw_3")]
pub type IrtxSwmPw3 = crate::Reg<irtx_swm_pw_3::IrtxSwmPw3Spec>;
#[doc = "irtx_swm_pw_3."]
pub mod irtx_swm_pw_3;
#[doc = "irtx_swm_pw_4 (rw) register accessor: irtx_swm_pw_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_4`] module"]
#[doc(alias = "irtx_swm_pw_4")]
pub type IrtxSwmPw4 = crate::Reg<irtx_swm_pw_4::IrtxSwmPw4Spec>;
#[doc = "irtx_swm_pw_4."]
pub mod irtx_swm_pw_4;
#[doc = "irtx_swm_pw_5 (rw) register accessor: irtx_swm_pw_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_5`] module"]
#[doc(alias = "irtx_swm_pw_5")]
pub type IrtxSwmPw5 = crate::Reg<irtx_swm_pw_5::IrtxSwmPw5Spec>;
#[doc = "irtx_swm_pw_5."]
pub mod irtx_swm_pw_5;
#[doc = "irtx_swm_pw_6 (rw) register accessor: irtx_swm_pw_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_6`] module"]
#[doc(alias = "irtx_swm_pw_6")]
pub type IrtxSwmPw6 = crate::Reg<irtx_swm_pw_6::IrtxSwmPw6Spec>;
#[doc = "irtx_swm_pw_6."]
pub mod irtx_swm_pw_6;
#[doc = "irtx_swm_pw_7 (rw) register accessor: irtx_swm_pw_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_swm_pw_7`] module"]
#[doc(alias = "irtx_swm_pw_7")]
pub type IrtxSwmPw7 = crate::Reg<irtx_swm_pw_7::IrtxSwmPw7Spec>;
#[doc = "irtx_swm_pw_7."]
pub mod irtx_swm_pw_7;
#[doc = "irrx_config (rw) register accessor: irrx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_config`] module"]
#[doc(alias = "irrx_config")]
pub type IrrxConfig = crate::Reg<irrx_config::IrrxConfigSpec>;
#[doc = "irrx_config."]
pub mod irrx_config;
#[doc = "irrx_int_sts (rw) register accessor: irrx_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_int_sts`] module"]
#[doc(alias = "irrx_int_sts")]
pub type IrrxIntSts = crate::Reg<irrx_int_sts::IrrxIntStsSpec>;
#[doc = "irrx_int_sts."]
pub mod irrx_int_sts;
#[doc = "irrx_pw_config (rw) register accessor: irrx_pw_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_pw_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_pw_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_pw_config`] module"]
#[doc(alias = "irrx_pw_config")]
pub type IrrxPwConfig = crate::Reg<irrx_pw_config::IrrxPwConfigSpec>;
#[doc = "irrx_pw_config."]
pub mod irrx_pw_config;
#[doc = "irrx_data_count (rw) register accessor: irrx_data_count.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_count`] module"]
#[doc(alias = "irrx_data_count")]
pub type IrrxDataCount = crate::Reg<irrx_data_count::IrrxDataCountSpec>;
#[doc = "irrx_data_count."]
pub mod irrx_data_count;
#[doc = "irrx_data_word0 (rw) register accessor: irrx_data_word0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_word0`] module"]
#[doc(alias = "irrx_data_word0")]
pub type IrrxDataWord0 = crate::Reg<irrx_data_word0::IrrxDataWord0Spec>;
#[doc = "irrx_data_word0."]
pub mod irrx_data_word0;
#[doc = "irrx_data_word1 (rw) register accessor: irrx_data_word1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_word1`] module"]
#[doc(alias = "irrx_data_word1")]
pub type IrrxDataWord1 = crate::Reg<irrx_data_word1::IrrxDataWord1Spec>;
#[doc = "irrx_data_word1."]
pub mod irrx_data_word1;
#[doc = "irrx_swm_fifo_config_0 (rw) register accessor: irrx_swm_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_swm_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_swm_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_swm_fifo_config_0`] module"]
#[doc(alias = "irrx_swm_fifo_config_0")]
pub type IrrxSwmFifoConfig0 = crate::Reg<irrx_swm_fifo_config_0::IrrxSwmFifoConfig0Spec>;
#[doc = "irrx_swm_fifo_config_0."]
pub mod irrx_swm_fifo_config_0;
#[doc = "irrx_swm_fifo_rdata (r) register accessor: irrx_swm_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_swm_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_swm_fifo_rdata`] module"]
#[doc(alias = "irrx_swm_fifo_rdata")]
pub type IrrxSwmFifoRdata = crate::Reg<irrx_swm_fifo_rdata::IrrxSwmFifoRdataSpec>;
#[doc = "irrx_swm_fifo_rdata."]
pub mod irrx_swm_fifo_rdata;
