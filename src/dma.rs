#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_int_status: DmaIntStatus,
    dma_int_tcstatus: DmaIntTcstatus,
    dma_int_tcclear: DmaIntTcclear,
    dma_int_error_status: DmaIntErrorStatus,
    dma_int_err_clr: DmaIntErrClr,
    dma_raw_int_tcstatus: DmaRawIntTcstatus,
    dma_raw_int_error_status: DmaRawIntErrorStatus,
    dma_enbld_chns: DmaEnbldChns,
    dma_soft_breq: DmaSoftBreq,
    dma_soft_sreq: DmaSoftSreq,
    dma_soft_lbreq: DmaSoftLbreq,
    dma_soft_lsreq: DmaSoftLsreq,
    dma_top_config: DmaTopConfig,
    dma_sync: DmaSync,
    _reserved14: [u8; 0xc8],
    ch: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    #[inline(always)]
    pub const fn dma_int_status(&self) -> &DmaIntStatus {
        &self.dma_int_status
    }
    #[doc = "0x04 - DMA_IntTCStatus."]
    #[inline(always)]
    pub const fn dma_int_tcstatus(&self) -> &DmaIntTcstatus {
        &self.dma_int_tcstatus
    }
    #[doc = "0x08 - DMA_IntTCClear."]
    #[inline(always)]
    pub const fn dma_int_tcclear(&self) -> &DmaIntTcclear {
        &self.dma_int_tcclear
    }
    #[doc = "0x0c - DMA_IntErrorStatus."]
    #[inline(always)]
    pub const fn dma_int_error_status(&self) -> &DmaIntErrorStatus {
        &self.dma_int_error_status
    }
    #[doc = "0x10 - DMA_IntErrClr."]
    #[inline(always)]
    pub const fn dma_int_err_clr(&self) -> &DmaIntErrClr {
        &self.dma_int_err_clr
    }
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    #[inline(always)]
    pub const fn dma_raw_int_tcstatus(&self) -> &DmaRawIntTcstatus {
        &self.dma_raw_int_tcstatus
    }
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    #[inline(always)]
    pub const fn dma_raw_int_error_status(&self) -> &DmaRawIntErrorStatus {
        &self.dma_raw_int_error_status
    }
    #[doc = "0x1c - DMA_EnbldChns."]
    #[inline(always)]
    pub const fn dma_enbld_chns(&self) -> &DmaEnbldChns {
        &self.dma_enbld_chns
    }
    #[doc = "0x20 - DMA_SoftBReq."]
    #[inline(always)]
    pub const fn dma_soft_breq(&self) -> &DmaSoftBreq {
        &self.dma_soft_breq
    }
    #[doc = "0x24 - DMA_SoftSReq."]
    #[inline(always)]
    pub const fn dma_soft_sreq(&self) -> &DmaSoftSreq {
        &self.dma_soft_sreq
    }
    #[doc = "0x28 - DMA_SoftLBReq."]
    #[inline(always)]
    pub const fn dma_soft_lbreq(&self) -> &DmaSoftLbreq {
        &self.dma_soft_lbreq
    }
    #[doc = "0x2c - DMA_SoftLSReq."]
    #[inline(always)]
    pub const fn dma_soft_lsreq(&self) -> &DmaSoftLsreq {
        &self.dma_soft_lsreq
    }
    #[doc = "0x30 - DMA_Top_Config."]
    #[inline(always)]
    pub const fn dma_top_config(&self) -> &DmaTopConfig {
        &self.dma_top_config
    }
    #[doc = "0x34 - DMA_Sync."]
    #[inline(always)]
    pub const fn dma_sync(&self) -> &DmaSync {
        &self.dma_sync
    }
    #[doc = "0x100..0x150 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x150 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "DMA_IntStatus (rw) register accessor: DMA_IntStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_status`] module"]
#[doc(alias = "DMA_IntStatus")]
pub type DmaIntStatus = crate::Reg<dma_int_status::DmaIntStatusSpec>;
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus (rw) register accessor: DMA_IntTCStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcstatus`] module"]
#[doc(alias = "DMA_IntTCStatus")]
pub type DmaIntTcstatus = crate::Reg<dma_int_tcstatus::DmaIntTcstatusSpec>;
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear (rw) register accessor: DMA_IntTCClear.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcclear`] module"]
#[doc(alias = "DMA_IntTCClear")]
pub type DmaIntTcclear = crate::Reg<dma_int_tcclear::DmaIntTcclearSpec>;
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus (rw) register accessor: DMA_IntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_error_status`] module"]
#[doc(alias = "DMA_IntErrorStatus")]
pub type DmaIntErrorStatus = crate::Reg<dma_int_error_status::DmaIntErrorStatusSpec>;
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr (rw) register accessor: DMA_IntErrClr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_err_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_err_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_err_clr`] module"]
#[doc(alias = "DMA_IntErrClr")]
pub type DmaIntErrClr = crate::Reg<dma_int_err_clr::DmaIntErrClrSpec>;
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus (rw) register accessor: DMA_RawIntTCStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_int_tcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_int_tcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_raw_int_tcstatus`] module"]
#[doc(alias = "DMA_RawIntTCStatus")]
pub type DmaRawIntTcstatus = crate::Reg<dma_raw_int_tcstatus::DmaRawIntTcstatusSpec>;
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus (rw) register accessor: DMA_RawIntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_int_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_int_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_raw_int_error_status`] module"]
#[doc(alias = "DMA_RawIntErrorStatus")]
pub type DmaRawIntErrorStatus = crate::Reg<dma_raw_int_error_status::DmaRawIntErrorStatusSpec>;
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns (rw) register accessor: DMA_EnbldChns.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enbld_chns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enbld_chns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enbld_chns`] module"]
#[doc(alias = "DMA_EnbldChns")]
pub type DmaEnbldChns = crate::Reg<dma_enbld_chns::DmaEnbldChnsSpec>;
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq (rw) register accessor: DMA_SoftBReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_breq`] module"]
#[doc(alias = "DMA_SoftBReq")]
pub type DmaSoftBreq = crate::Reg<dma_soft_breq::DmaSoftBreqSpec>;
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq (rw) register accessor: DMA_SoftSReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_sreq`] module"]
#[doc(alias = "DMA_SoftSReq")]
pub type DmaSoftSreq = crate::Reg<dma_soft_sreq::DmaSoftSreqSpec>;
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq (rw) register accessor: DMA_SoftLBReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lbreq`] module"]
#[doc(alias = "DMA_SoftLBReq")]
pub type DmaSoftLbreq = crate::Reg<dma_soft_lbreq::DmaSoftLbreqSpec>;
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq (rw) register accessor: DMA_SoftLSReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lsreq`] module"]
#[doc(alias = "DMA_SoftLSReq")]
pub type DmaSoftLsreq = crate::Reg<dma_soft_lsreq::DmaSoftLsreqSpec>;
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config (rw) register accessor: DMA_Top_Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_top_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_top_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_top_config`] module"]
#[doc(alias = "DMA_Top_Config")]
pub type DmaTopConfig = crate::Reg<dma_top_config::DmaTopConfigSpec>;
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync (rw) register accessor: DMA_Sync.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sync`] module"]
#[doc(alias = "DMA_Sync")]
pub type DmaSync = crate::Reg<dma_sync::DmaSyncSpec>;
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
pub mod ch;
