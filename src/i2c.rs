#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_config: I2cConfig,
    i2c_int_sts: I2cIntSts,
    i2c_sub_addr: I2cSubAddr,
    i2c_bus_busy: I2cBusBusy,
    i2c_prd_start: I2cPrdStart,
    i2c_prd_stop: I2cPrdStop,
    i2c_prd_data: I2cPrdData,
    _reserved7: [u8; 0x64],
    i2c_fifo_config_0: I2cFifoConfig0,
    i2c_fifo_config_1: I2cFifoConfig1,
    i2c_fifo_wdata: I2cFifoWdata,
    i2c_fifo_rdata: I2cFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - i2c_config."]
    #[inline(always)]
    pub const fn i2c_config(&self) -> &I2cConfig {
        &self.i2c_config
    }
    #[doc = "0x04 - i2c_int_sts."]
    #[inline(always)]
    pub const fn i2c_int_sts(&self) -> &I2cIntSts {
        &self.i2c_int_sts
    }
    #[doc = "0x08 - i2c_sub_addr."]
    #[inline(always)]
    pub const fn i2c_sub_addr(&self) -> &I2cSubAddr {
        &self.i2c_sub_addr
    }
    #[doc = "0x0c - i2c_bus_busy."]
    #[inline(always)]
    pub const fn i2c_bus_busy(&self) -> &I2cBusBusy {
        &self.i2c_bus_busy
    }
    #[doc = "0x10 - i2c_prd_start."]
    #[inline(always)]
    pub const fn i2c_prd_start(&self) -> &I2cPrdStart {
        &self.i2c_prd_start
    }
    #[doc = "0x14 - i2c_prd_stop."]
    #[inline(always)]
    pub const fn i2c_prd_stop(&self) -> &I2cPrdStop {
        &self.i2c_prd_stop
    }
    #[doc = "0x18 - i2c_prd_data."]
    #[inline(always)]
    pub const fn i2c_prd_data(&self) -> &I2cPrdData {
        &self.i2c_prd_data
    }
    #[doc = "0x80 - i2c_fifo_config_0."]
    #[inline(always)]
    pub const fn i2c_fifo_config_0(&self) -> &I2cFifoConfig0 {
        &self.i2c_fifo_config_0
    }
    #[doc = "0x84 - i2c_fifo_config_1."]
    #[inline(always)]
    pub const fn i2c_fifo_config_1(&self) -> &I2cFifoConfig1 {
        &self.i2c_fifo_config_1
    }
    #[doc = "0x88 - i2c_fifo_wdata."]
    #[inline(always)]
    pub const fn i2c_fifo_wdata(&self) -> &I2cFifoWdata {
        &self.i2c_fifo_wdata
    }
    #[doc = "0x8c - i2c_fifo_rdata."]
    #[inline(always)]
    pub const fn i2c_fifo_rdata(&self) -> &I2cFifoRdata {
        &self.i2c_fifo_rdata
    }
}
#[doc = "i2c_config (rw) register accessor: i2c_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_config`] module"]
#[doc(alias = "i2c_config")]
pub type I2cConfig = crate::Reg<i2c_config::I2cConfigSpec>;
#[doc = "i2c_config."]
pub mod i2c_config;
#[doc = "i2c_int_sts (rw) register accessor: i2c_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_int_sts`] module"]
#[doc(alias = "i2c_int_sts")]
pub type I2cIntSts = crate::Reg<i2c_int_sts::I2cIntStsSpec>;
#[doc = "i2c_int_sts."]
pub mod i2c_int_sts;
#[doc = "i2c_sub_addr (rw) register accessor: i2c_sub_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sub_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_sub_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sub_addr`] module"]
#[doc(alias = "i2c_sub_addr")]
pub type I2cSubAddr = crate::Reg<i2c_sub_addr::I2cSubAddrSpec>;
#[doc = "i2c_sub_addr."]
pub mod i2c_sub_addr;
#[doc = "i2c_bus_busy (rw) register accessor: i2c_bus_busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bus_busy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bus_busy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_bus_busy`] module"]
#[doc(alias = "i2c_bus_busy")]
pub type I2cBusBusy = crate::Reg<i2c_bus_busy::I2cBusBusySpec>;
#[doc = "i2c_bus_busy."]
pub mod i2c_bus_busy;
#[doc = "i2c_prd_start (rw) register accessor: i2c_prd_start.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_start`] module"]
#[doc(alias = "i2c_prd_start")]
pub type I2cPrdStart = crate::Reg<i2c_prd_start::I2cPrdStartSpec>;
#[doc = "i2c_prd_start."]
pub mod i2c_prd_start;
#[doc = "i2c_prd_stop (rw) register accessor: i2c_prd_stop.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_stop`] module"]
#[doc(alias = "i2c_prd_stop")]
pub type I2cPrdStop = crate::Reg<i2c_prd_stop::I2cPrdStopSpec>;
#[doc = "i2c_prd_stop."]
pub mod i2c_prd_stop;
#[doc = "i2c_prd_data (rw) register accessor: i2c_prd_data.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_data`] module"]
#[doc(alias = "i2c_prd_data")]
pub type I2cPrdData = crate::Reg<i2c_prd_data::I2cPrdDataSpec>;
#[doc = "i2c_prd_data."]
pub mod i2c_prd_data;
#[doc = "i2c_fifo_config_0 (rw) register accessor: i2c_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_config_0`] module"]
#[doc(alias = "i2c_fifo_config_0")]
pub type I2cFifoConfig0 = crate::Reg<i2c_fifo_config_0::I2cFifoConfig0Spec>;
#[doc = "i2c_fifo_config_0."]
pub mod i2c_fifo_config_0;
#[doc = "i2c_fifo_config_1 (rw) register accessor: i2c_fifo_config_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_config_1`] module"]
#[doc(alias = "i2c_fifo_config_1")]
pub type I2cFifoConfig1 = crate::Reg<i2c_fifo_config_1::I2cFifoConfig1Spec>;
#[doc = "i2c_fifo_config_1."]
pub mod i2c_fifo_config_1;
#[doc = "i2c_fifo_wdata (w) register accessor: i2c_fifo_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_wdata`] module"]
#[doc(alias = "i2c_fifo_wdata")]
pub type I2cFifoWdata = crate::Reg<i2c_fifo_wdata::I2cFifoWdataSpec>;
#[doc = "i2c_fifo_wdata."]
pub mod i2c_fifo_wdata;
#[doc = "i2c_fifo_rdata (r) register accessor: i2c_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_rdata`] module"]
#[doc(alias = "i2c_fifo_rdata")]
pub type I2cFifoRdata = crate::Reg<i2c_fifo_rdata::I2cFifoRdataSpec>;
#[doc = "i2c_fifo_rdata."]
pub mod i2c_fifo_rdata;
