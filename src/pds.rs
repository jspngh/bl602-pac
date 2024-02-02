#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pds_ctl: PdsCtl,
    pds_time1: PdsTime1,
    _reserved2: [u8; 0x04],
    pds_int: PdsInt,
    pds_ctl2: PdsCtl2,
    pds_ctl3: PdsCtl3,
    pds_ctl4: PdsCtl4,
    pds_stat: PdsStat,
    pds_ram1: PdsRam1,
    _reserved8: [u8; 0x02dc],
    rc32m_ctrl0: Rc32mCtrl0,
    rc32m_ctrl1: Rc32mCtrl1,
    _reserved10: [u8; 0xf8],
    pu_rst_clkpll: PuRstClkpll,
    clkpll_top_ctrl: ClkpllTopCtrl,
    clkpll_cp: ClkpllCp,
    clkpll_rz: ClkpllRz,
    clkpll_fbdv: ClkpllFbdv,
    clkpll_vco: ClkpllVco,
    clkpll_sdm: ClkpllSdm,
    clkpll_output_en: ClkpllOutputEn,
}
impl RegisterBlock {
    #[doc = "0x00 - PDS_CTL."]
    #[inline(always)]
    pub const fn pds_ctl(&self) -> &PdsCtl {
        &self.pds_ctl
    }
    #[doc = "0x04 - PDS_TIME1."]
    #[inline(always)]
    pub const fn pds_time1(&self) -> &PdsTime1 {
        &self.pds_time1
    }
    #[doc = "0x0c - PDS_INT."]
    #[inline(always)]
    pub const fn pds_int(&self) -> &PdsInt {
        &self.pds_int
    }
    #[doc = "0x10 - PDS_CTL2."]
    #[inline(always)]
    pub const fn pds_ctl2(&self) -> &PdsCtl2 {
        &self.pds_ctl2
    }
    #[doc = "0x14 - PDS_CTL3."]
    #[inline(always)]
    pub const fn pds_ctl3(&self) -> &PdsCtl3 {
        &self.pds_ctl3
    }
    #[doc = "0x18 - PDS_CTL4."]
    #[inline(always)]
    pub const fn pds_ctl4(&self) -> &PdsCtl4 {
        &self.pds_ctl4
    }
    #[doc = "0x1c - pds_stat."]
    #[inline(always)]
    pub const fn pds_stat(&self) -> &PdsStat {
        &self.pds_stat
    }
    #[doc = "0x20 - pds_ram1."]
    #[inline(always)]
    pub const fn pds_ram1(&self) -> &PdsRam1 {
        &self.pds_ram1
    }
    #[doc = "0x300 - rc32m_ctrl0."]
    #[inline(always)]
    pub const fn rc32m_ctrl0(&self) -> &Rc32mCtrl0 {
        &self.rc32m_ctrl0
    }
    #[doc = "0x304 - rc32m_ctrl1."]
    #[inline(always)]
    pub const fn rc32m_ctrl1(&self) -> &Rc32mCtrl1 {
        &self.rc32m_ctrl1
    }
    #[doc = "0x400 - pu_rst_clkpll."]
    #[inline(always)]
    pub const fn pu_rst_clkpll(&self) -> &PuRstClkpll {
        &self.pu_rst_clkpll
    }
    #[doc = "0x404 - clkpll_top_ctrl."]
    #[inline(always)]
    pub const fn clkpll_top_ctrl(&self) -> &ClkpllTopCtrl {
        &self.clkpll_top_ctrl
    }
    #[doc = "0x408 - clkpll_cp."]
    #[inline(always)]
    pub const fn clkpll_cp(&self) -> &ClkpllCp {
        &self.clkpll_cp
    }
    #[doc = "0x40c - clkpll_rz."]
    #[inline(always)]
    pub const fn clkpll_rz(&self) -> &ClkpllRz {
        &self.clkpll_rz
    }
    #[doc = "0x410 - clkpll_fbdv."]
    #[inline(always)]
    pub const fn clkpll_fbdv(&self) -> &ClkpllFbdv {
        &self.clkpll_fbdv
    }
    #[doc = "0x414 - clkpll_vco."]
    #[inline(always)]
    pub const fn clkpll_vco(&self) -> &ClkpllVco {
        &self.clkpll_vco
    }
    #[doc = "0x418 - clkpll_sdm."]
    #[inline(always)]
    pub const fn clkpll_sdm(&self) -> &ClkpllSdm {
        &self.clkpll_sdm
    }
    #[doc = "0x41c - clkpll_output_en."]
    #[inline(always)]
    pub const fn clkpll_output_en(&self) -> &ClkpllOutputEn {
        &self.clkpll_output_en
    }
}
#[doc = "PDS_CTL (rw) register accessor: PDS_CTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl`] module"]
#[doc(alias = "PDS_CTL")]
pub type PdsCtl = crate::Reg<pds_ctl::PdsCtlSpec>;
#[doc = "PDS_CTL."]
pub mod pds_ctl;
#[doc = "PDS_TIME1 (rw) register accessor: PDS_TIME1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_time1`] module"]
#[doc(alias = "PDS_TIME1")]
pub type PdsTime1 = crate::Reg<pds_time1::PdsTime1Spec>;
#[doc = "PDS_TIME1."]
pub mod pds_time1;
#[doc = "PDS_INT (rw) register accessor: PDS_INT.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_int`] module"]
#[doc(alias = "PDS_INT")]
pub type PdsInt = crate::Reg<pds_int::PdsIntSpec>;
#[doc = "PDS_INT."]
pub mod pds_int;
#[doc = "PDS_CTL2 (rw) register accessor: PDS_CTL2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl2`] module"]
#[doc(alias = "PDS_CTL2")]
pub type PdsCtl2 = crate::Reg<pds_ctl2::PdsCtl2Spec>;
#[doc = "PDS_CTL2."]
pub mod pds_ctl2;
#[doc = "PDS_CTL3 (rw) register accessor: PDS_CTL3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl3`] module"]
#[doc(alias = "PDS_CTL3")]
pub type PdsCtl3 = crate::Reg<pds_ctl3::PdsCtl3Spec>;
#[doc = "PDS_CTL3."]
pub mod pds_ctl3;
#[doc = "PDS_CTL4 (rw) register accessor: PDS_CTL4.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl4`] module"]
#[doc(alias = "PDS_CTL4")]
pub type PdsCtl4 = crate::Reg<pds_ctl4::PdsCtl4Spec>;
#[doc = "PDS_CTL4."]
pub mod pds_ctl4;
#[doc = "pds_stat (r) register accessor: pds_stat.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_stat`] module"]
#[doc(alias = "pds_stat")]
pub type PdsStat = crate::Reg<pds_stat::PdsStatSpec>;
#[doc = "pds_stat."]
pub mod pds_stat;
#[doc = "pds_ram1 (rw) register accessor: pds_ram1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram1`] module"]
#[doc(alias = "pds_ram1")]
pub type PdsRam1 = crate::Reg<pds_ram1::PdsRam1Spec>;
#[doc = "pds_ram1."]
pub mod pds_ram1;
#[doc = "rc32m_ctrl0 (rw) register accessor: rc32m_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl0`] module"]
#[doc(alias = "rc32m_ctrl0")]
pub type Rc32mCtrl0 = crate::Reg<rc32m_ctrl0::Rc32mCtrl0Spec>;
#[doc = "rc32m_ctrl0."]
pub mod rc32m_ctrl0;
#[doc = "rc32m_ctrl1 (rw) register accessor: rc32m_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl1`] module"]
#[doc(alias = "rc32m_ctrl1")]
pub type Rc32mCtrl1 = crate::Reg<rc32m_ctrl1::Rc32mCtrl1Spec>;
#[doc = "rc32m_ctrl1."]
pub mod rc32m_ctrl1;
#[doc = "pu_rst_clkpll (rw) register accessor: pu_rst_clkpll.\n\nYou can [`read`](crate::Reg::read) this register and get [`pu_rst_clkpll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pu_rst_clkpll`] module"]
#[doc(alias = "pu_rst_clkpll")]
pub type PuRstClkpll = crate::Reg<pu_rst_clkpll::PuRstClkpllSpec>;
#[doc = "pu_rst_clkpll."]
pub mod pu_rst_clkpll;
#[doc = "clkpll_top_ctrl (rw) register accessor: clkpll_top_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_top_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_top_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_top_ctrl`] module"]
#[doc(alias = "clkpll_top_ctrl")]
pub type ClkpllTopCtrl = crate::Reg<clkpll_top_ctrl::ClkpllTopCtrlSpec>;
#[doc = "clkpll_top_ctrl."]
pub mod clkpll_top_ctrl;
#[doc = "clkpll_cp (rw) register accessor: clkpll_cp.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_cp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_cp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_cp`] module"]
#[doc(alias = "clkpll_cp")]
pub type ClkpllCp = crate::Reg<clkpll_cp::ClkpllCpSpec>;
#[doc = "clkpll_cp."]
pub mod clkpll_cp;
#[doc = "clkpll_rz (rw) register accessor: clkpll_rz.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_rz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_rz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_rz`] module"]
#[doc(alias = "clkpll_rz")]
pub type ClkpllRz = crate::Reg<clkpll_rz::ClkpllRzSpec>;
#[doc = "clkpll_rz."]
pub mod clkpll_rz;
#[doc = "clkpll_fbdv (rw) register accessor: clkpll_fbdv.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_fbdv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_fbdv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_fbdv`] module"]
#[doc(alias = "clkpll_fbdv")]
pub type ClkpllFbdv = crate::Reg<clkpll_fbdv::ClkpllFbdvSpec>;
#[doc = "clkpll_fbdv."]
pub mod clkpll_fbdv;
#[doc = "clkpll_vco (rw) register accessor: clkpll_vco.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_vco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_vco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_vco`] module"]
#[doc(alias = "clkpll_vco")]
pub type ClkpllVco = crate::Reg<clkpll_vco::ClkpllVcoSpec>;
#[doc = "clkpll_vco."]
pub mod clkpll_vco;
#[doc = "clkpll_sdm (rw) register accessor: clkpll_sdm.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_sdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_sdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_sdm`] module"]
#[doc(alias = "clkpll_sdm")]
pub type ClkpllSdm = crate::Reg<clkpll_sdm::ClkpllSdmSpec>;
#[doc = "clkpll_sdm."]
pub mod clkpll_sdm;
#[doc = "clkpll_output_en (rw) register accessor: clkpll_output_en.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_output_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_output_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpll_output_en`] module"]
#[doc(alias = "clkpll_output_en")]
pub type ClkpllOutputEn = crate::Reg<clkpll_output_en::ClkpllOutputEnSpec>;
#[doc = "clkpll_output_en."]
pub mod clkpll_output_en;
