#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cci_cfg: CciCfg,
    cci_addr: CciAddr,
    cci_wdata: CciWdata,
    cci_rdata: CciRdata,
    cci_ctl: CciCtl,
}
impl RegisterBlock {
    #[doc = "0x00 - cci_cfg."]
    #[inline(always)]
    pub const fn cci_cfg(&self) -> &CciCfg {
        &self.cci_cfg
    }
    #[doc = "0x04 - cci_addr."]
    #[inline(always)]
    pub const fn cci_addr(&self) -> &CciAddr {
        &self.cci_addr
    }
    #[doc = "0x08 - cci_wdata."]
    #[inline(always)]
    pub const fn cci_wdata(&self) -> &CciWdata {
        &self.cci_wdata
    }
    #[doc = "0x0c - cci_rdata."]
    #[inline(always)]
    pub const fn cci_rdata(&self) -> &CciRdata {
        &self.cci_rdata
    }
    #[doc = "0x10 - cci_ctl."]
    #[inline(always)]
    pub const fn cci_ctl(&self) -> &CciCtl {
        &self.cci_ctl
    }
}
#[doc = "cci_cfg (rw) register accessor: cci_cfg.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_cfg`] module"]
#[doc(alias = "cci_cfg")]
pub type CciCfg = crate::Reg<cci_cfg::CciCfgSpec>;
#[doc = "cci_cfg."]
pub mod cci_cfg;
#[doc = "cci_addr (rw) register accessor: cci_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_addr`] module"]
#[doc(alias = "cci_addr")]
pub type CciAddr = crate::Reg<cci_addr::CciAddrSpec>;
#[doc = "cci_addr."]
pub mod cci_addr;
#[doc = "cci_wdata (rw) register accessor: cci_wdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_wdata`] module"]
#[doc(alias = "cci_wdata")]
pub type CciWdata = crate::Reg<cci_wdata::CciWdataSpec>;
#[doc = "cci_wdata."]
pub mod cci_wdata;
#[doc = "cci_rdata (rw) register accessor: cci_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_rdata`] module"]
#[doc(alias = "cci_rdata")]
pub type CciRdata = crate::Reg<cci_rdata::CciRdataSpec>;
#[doc = "cci_rdata."]
pub mod cci_rdata;
#[doc = "cci_ctl (rw) register accessor: cci_ctl.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_ctl`] module"]
#[doc(alias = "cci_ctl")]
pub type CciCtl = crate::Reg<cci_ctl::CciCtlSpec>;
#[doc = "cci_ctl."]
pub mod cci_ctl;
