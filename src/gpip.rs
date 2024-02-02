#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpadc_config: GpadcConfig,
    gpadc_dma_rdata: GpadcDmaRdata,
    _reserved2: [u8; 0x38],
    gpdac_config: GpdacConfig,
    gpdac_dma_config: GpdacDmaConfig,
    gpdac_dma_wdata: GpdacDmaWdata,
    gpdac_tx_fifo_status: GpdacTxFifoStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - gpadc_config."]
    #[inline(always)]
    pub const fn gpadc_config(&self) -> &GpadcConfig {
        &self.gpadc_config
    }
    #[doc = "0x04 - gpadc_dma_rdata."]
    #[inline(always)]
    pub const fn gpadc_dma_rdata(&self) -> &GpadcDmaRdata {
        &self.gpadc_dma_rdata
    }
    #[doc = "0x40 - gpdac_config."]
    #[inline(always)]
    pub const fn gpdac_config(&self) -> &GpdacConfig {
        &self.gpdac_config
    }
    #[doc = "0x44 - gpdac_dma_config."]
    #[inline(always)]
    pub const fn gpdac_dma_config(&self) -> &GpdacDmaConfig {
        &self.gpdac_dma_config
    }
    #[doc = "0x48 - gpdac_dma_wdata."]
    #[inline(always)]
    pub const fn gpdac_dma_wdata(&self) -> &GpdacDmaWdata {
        &self.gpdac_dma_wdata
    }
    #[doc = "0x4c - gpdac_tx_fifo_status."]
    #[inline(always)]
    pub const fn gpdac_tx_fifo_status(&self) -> &GpdacTxFifoStatus {
        &self.gpdac_tx_fifo_status
    }
}
#[doc = "gpadc_config (rw) register accessor: gpadc_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_config`] module"]
#[doc(alias = "gpadc_config")]
pub type GpadcConfig = crate::Reg<gpadc_config::GpadcConfigSpec>;
#[doc = "gpadc_config."]
pub mod gpadc_config;
#[doc = "gpadc_dma_rdata (r) register accessor: gpadc_dma_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_dma_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_dma_rdata`] module"]
#[doc(alias = "gpadc_dma_rdata")]
pub type GpadcDmaRdata = crate::Reg<gpadc_dma_rdata::GpadcDmaRdataSpec>;
#[doc = "gpadc_dma_rdata."]
pub mod gpadc_dma_rdata;
#[doc = "gpdac_config (rw) register accessor: gpdac_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config`] module"]
#[doc(alias = "gpdac_config")]
pub type GpdacConfig = crate::Reg<gpdac_config::GpdacConfigSpec>;
#[doc = "gpdac_config."]
pub mod gpdac_config;
#[doc = "gpdac_dma_config (rw) register accessor: gpdac_dma_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_dma_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_config`] module"]
#[doc(alias = "gpdac_dma_config")]
pub type GpdacDmaConfig = crate::Reg<gpdac_dma_config::GpdacDmaConfigSpec>;
#[doc = "gpdac_dma_config."]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_wdata (w) register accessor: gpdac_dma_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_wdata`] module"]
#[doc(alias = "gpdac_dma_wdata")]
pub type GpdacDmaWdata = crate::Reg<gpdac_dma_wdata::GpdacDmaWdataSpec>;
#[doc = "gpdac_dma_wdata."]
pub mod gpdac_dma_wdata;
#[doc = "gpdac_tx_fifo_status (r) register accessor: gpdac_tx_fifo_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_tx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_tx_fifo_status`] module"]
#[doc(alias = "gpdac_tx_fifo_status")]
pub type GpdacTxFifoStatus = crate::Reg<gpdac_tx_fifo_status::GpdacTxFifoStatusSpec>;
#[doc = "gpdac_tx_fifo_status."]
pub mod gpdac_tx_fifo_status;
