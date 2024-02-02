#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_config: SpiConfig,
    spi_int_sts: SpiIntSts,
    spi_bus_busy: SpiBusBusy,
    _reserved3: [u8; 0x04],
    spi_prd_0: SpiPrd0,
    spi_prd_1: SpiPrd1,
    spi_rxd_ignr: SpiRxdIgnr,
    spi_sto_value: SpiStoValue,
    _reserved7: [u8; 0x60],
    spi_fifo_config_0: SpiFifoConfig0,
    spi_fifo_config_1: SpiFifoConfig1,
    spi_fifo_wdata: SpiFifoWdata,
    spi_fifo_rdata: SpiFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - spi_config."]
    #[inline(always)]
    pub const fn spi_config(&self) -> &SpiConfig {
        &self.spi_config
    }
    #[doc = "0x04 - spi_int_sts."]
    #[inline(always)]
    pub const fn spi_int_sts(&self) -> &SpiIntSts {
        &self.spi_int_sts
    }
    #[doc = "0x08 - spi_bus_busy."]
    #[inline(always)]
    pub const fn spi_bus_busy(&self) -> &SpiBusBusy {
        &self.spi_bus_busy
    }
    #[doc = "0x10 - spi_prd_0."]
    #[inline(always)]
    pub const fn spi_prd_0(&self) -> &SpiPrd0 {
        &self.spi_prd_0
    }
    #[doc = "0x14 - spi_prd_1."]
    #[inline(always)]
    pub const fn spi_prd_1(&self) -> &SpiPrd1 {
        &self.spi_prd_1
    }
    #[doc = "0x18 - spi_rxd_ignr."]
    #[inline(always)]
    pub const fn spi_rxd_ignr(&self) -> &SpiRxdIgnr {
        &self.spi_rxd_ignr
    }
    #[doc = "0x1c - spi_sto_value."]
    #[inline(always)]
    pub const fn spi_sto_value(&self) -> &SpiStoValue {
        &self.spi_sto_value
    }
    #[doc = "0x80 - spi_fifo_config_0."]
    #[inline(always)]
    pub const fn spi_fifo_config_0(&self) -> &SpiFifoConfig0 {
        &self.spi_fifo_config_0
    }
    #[doc = "0x84 - spi_fifo_config_1."]
    #[inline(always)]
    pub const fn spi_fifo_config_1(&self) -> &SpiFifoConfig1 {
        &self.spi_fifo_config_1
    }
    #[doc = "0x88 - spi_fifo_wdata."]
    #[inline(always)]
    pub const fn spi_fifo_wdata(&self) -> &SpiFifoWdata {
        &self.spi_fifo_wdata
    }
    #[doc = "0x8c - spi_fifo_rdata."]
    #[inline(always)]
    pub const fn spi_fifo_rdata(&self) -> &SpiFifoRdata {
        &self.spi_fifo_rdata
    }
}
#[doc = "spi_config (rw) register accessor: spi_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_config`] module"]
#[doc(alias = "spi_config")]
pub type SpiConfig = crate::Reg<spi_config::SpiConfigSpec>;
#[doc = "spi_config."]
pub mod spi_config;
#[doc = "spi_int_sts (rw) register accessor: spi_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_int_sts`] module"]
#[doc(alias = "spi_int_sts")]
pub type SpiIntSts = crate::Reg<spi_int_sts::SpiIntStsSpec>;
#[doc = "spi_int_sts."]
pub mod spi_int_sts;
#[doc = "spi_bus_busy (r) register accessor: spi_bus_busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_bus_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_bus_busy`] module"]
#[doc(alias = "spi_bus_busy")]
pub type SpiBusBusy = crate::Reg<spi_bus_busy::SpiBusBusySpec>;
#[doc = "spi_bus_busy."]
pub mod spi_bus_busy;
#[doc = "spi_prd_0 (rw) register accessor: spi_prd_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_prd_0`] module"]
#[doc(alias = "spi_prd_0")]
pub type SpiPrd0 = crate::Reg<spi_prd_0::SpiPrd0Spec>;
#[doc = "spi_prd_0."]
pub mod spi_prd_0;
#[doc = "spi_prd_1 (rw) register accessor: spi_prd_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_prd_1`] module"]
#[doc(alias = "spi_prd_1")]
pub type SpiPrd1 = crate::Reg<spi_prd_1::SpiPrd1Spec>;
#[doc = "spi_prd_1."]
pub mod spi_prd_1;
#[doc = "spi_rxd_ignr (rw) register accessor: spi_rxd_ignr.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_rxd_ignr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_rxd_ignr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxd_ignr`] module"]
#[doc(alias = "spi_rxd_ignr")]
pub type SpiRxdIgnr = crate::Reg<spi_rxd_ignr::SpiRxdIgnrSpec>;
#[doc = "spi_rxd_ignr."]
pub mod spi_rxd_ignr;
#[doc = "spi_sto_value (rw) register accessor: spi_sto_value.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sto_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sto_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sto_value`] module"]
#[doc(alias = "spi_sto_value")]
pub type SpiStoValue = crate::Reg<spi_sto_value::SpiStoValueSpec>;
#[doc = "spi_sto_value."]
pub mod spi_sto_value;
#[doc = "spi_fifo_config_0 (rw) register accessor: spi_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_config_0`] module"]
#[doc(alias = "spi_fifo_config_0")]
pub type SpiFifoConfig0 = crate::Reg<spi_fifo_config_0::SpiFifoConfig0Spec>;
#[doc = "spi_fifo_config_0."]
pub mod spi_fifo_config_0;
#[doc = "spi_fifo_config_1 (rw) register accessor: spi_fifo_config_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_config_1`] module"]
#[doc(alias = "spi_fifo_config_1")]
pub type SpiFifoConfig1 = crate::Reg<spi_fifo_config_1::SpiFifoConfig1Spec>;
#[doc = "spi_fifo_config_1."]
pub mod spi_fifo_config_1;
#[doc = "spi_fifo_wdata (w) register accessor: spi_fifo_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_wdata`] module"]
#[doc(alias = "spi_fifo_wdata")]
pub type SpiFifoWdata = crate::Reg<spi_fifo_wdata::SpiFifoWdataSpec>;
#[doc = "spi_fifo_wdata."]
pub mod spi_fifo_wdata;
#[doc = "spi_fifo_rdata (r) register accessor: spi_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_rdata`] module"]
#[doc(alias = "spi_fifo_rdata")]
pub type SpiFifoRdata = crate::Reg<spi_fifo_rdata::SpiFifoRdataSpec>;
#[doc = "spi_fifo_rdata."]
pub mod spi_fifo_rdata;
