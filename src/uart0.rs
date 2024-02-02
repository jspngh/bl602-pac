#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    utx_config: UtxConfig,
    urx_config: UrxConfig,
    uart_bit_prd: UartBitPrd,
    data_config: DataConfig,
    utx_ir_position: UtxIrPosition,
    urx_ir_position: UrxIrPosition,
    urx_rto_timer: UrxRtoTimer,
    _reserved7: [u8; 0x04],
    uart_int_sts: UartIntSts,
    uart_int_mask: UartIntMask,
    uart_int_clear: UartIntClear,
    uart_int_en: UartIntEn,
    uart_status: UartStatus,
    sts_urx_abr_prd: StsUrxAbrPrd,
    _reserved13: [u8; 0x48],
    uart_fifo_config_0: UartFifoConfig0,
    uart_fifo_config_1: UartFifoConfig1,
    uart_fifo_wdata: UartFifoWdata,
    uart_fifo_rdata: UartFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - utx_config."]
    #[inline(always)]
    pub const fn utx_config(&self) -> &UtxConfig {
        &self.utx_config
    }
    #[doc = "0x04 - urx_config."]
    #[inline(always)]
    pub const fn urx_config(&self) -> &UrxConfig {
        &self.urx_config
    }
    #[doc = "0x08 - uart_bit_prd."]
    #[inline(always)]
    pub const fn uart_bit_prd(&self) -> &UartBitPrd {
        &self.uart_bit_prd
    }
    #[doc = "0x0c - data_config."]
    #[inline(always)]
    pub const fn data_config(&self) -> &DataConfig {
        &self.data_config
    }
    #[doc = "0x10 - utx_ir_position."]
    #[inline(always)]
    pub const fn utx_ir_position(&self) -> &UtxIrPosition {
        &self.utx_ir_position
    }
    #[doc = "0x14 - urx_ir_position."]
    #[inline(always)]
    pub const fn urx_ir_position(&self) -> &UrxIrPosition {
        &self.urx_ir_position
    }
    #[doc = "0x18 - urx_rto_timer."]
    #[inline(always)]
    pub const fn urx_rto_timer(&self) -> &UrxRtoTimer {
        &self.urx_rto_timer
    }
    #[doc = "0x20 - UART interrupt status"]
    #[inline(always)]
    pub const fn uart_int_sts(&self) -> &UartIntSts {
        &self.uart_int_sts
    }
    #[doc = "0x24 - UART interrupt mask"]
    #[inline(always)]
    pub const fn uart_int_mask(&self) -> &UartIntMask {
        &self.uart_int_mask
    }
    #[doc = "0x28 - UART interrupt clear"]
    #[inline(always)]
    pub const fn uart_int_clear(&self) -> &UartIntClear {
        &self.uart_int_clear
    }
    #[doc = "0x2c - UART interrupt enable"]
    #[inline(always)]
    pub const fn uart_int_en(&self) -> &UartIntEn {
        &self.uart_int_en
    }
    #[doc = "0x30 - uart_status."]
    #[inline(always)]
    pub const fn uart_status(&self) -> &UartStatus {
        &self.uart_status
    }
    #[doc = "0x34 - sts_urx_abr_prd."]
    #[inline(always)]
    pub const fn sts_urx_abr_prd(&self) -> &StsUrxAbrPrd {
        &self.sts_urx_abr_prd
    }
    #[doc = "0x80 - uart_fifo_config_0."]
    #[inline(always)]
    pub const fn uart_fifo_config_0(&self) -> &UartFifoConfig0 {
        &self.uart_fifo_config_0
    }
    #[doc = "0x84 - uart_fifo_config_1."]
    #[inline(always)]
    pub const fn uart_fifo_config_1(&self) -> &UartFifoConfig1 {
        &self.uart_fifo_config_1
    }
    #[doc = "0x88 - uart_fifo_wdata."]
    #[inline(always)]
    pub const fn uart_fifo_wdata(&self) -> &UartFifoWdata {
        &self.uart_fifo_wdata
    }
    #[doc = "0x8c - uart_fifo_rdata."]
    #[inline(always)]
    pub const fn uart_fifo_rdata(&self) -> &UartFifoRdata {
        &self.uart_fifo_rdata
    }
}
#[doc = "utx_config (rw) register accessor: utx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utx_config`] module"]
#[doc(alias = "utx_config")]
pub type UtxConfig = crate::Reg<utx_config::UtxConfigSpec>;
#[doc = "utx_config."]
pub mod utx_config;
#[doc = "urx_config (rw) register accessor: urx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_config`] module"]
#[doc(alias = "urx_config")]
pub type UrxConfig = crate::Reg<urx_config::UrxConfigSpec>;
#[doc = "urx_config."]
pub mod urx_config;
#[doc = "uart_bit_prd (rw) register accessor: uart_bit_prd.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_bit_prd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_bit_prd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_bit_prd`] module"]
#[doc(alias = "uart_bit_prd")]
pub type UartBitPrd = crate::Reg<uart_bit_prd::UartBitPrdSpec>;
#[doc = "uart_bit_prd."]
pub mod uart_bit_prd;
#[doc = "data_config (rw) register accessor: data_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_config`] module"]
#[doc(alias = "data_config")]
pub type DataConfig = crate::Reg<data_config::DataConfigSpec>;
#[doc = "data_config."]
pub mod data_config;
#[doc = "utx_ir_position (rw) register accessor: utx_ir_position.\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_ir_position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_ir_position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utx_ir_position`] module"]
#[doc(alias = "utx_ir_position")]
pub type UtxIrPosition = crate::Reg<utx_ir_position::UtxIrPositionSpec>;
#[doc = "utx_ir_position."]
pub mod utx_ir_position;
#[doc = "urx_ir_position (rw) register accessor: urx_ir_position.\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_ir_position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_ir_position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_ir_position`] module"]
#[doc(alias = "urx_ir_position")]
pub type UrxIrPosition = crate::Reg<urx_ir_position::UrxIrPositionSpec>;
#[doc = "urx_ir_position."]
pub mod urx_ir_position;
#[doc = "urx_rto_timer (rw) register accessor: urx_rto_timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_rto_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_rto_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_rto_timer`] module"]
#[doc(alias = "urx_rto_timer")]
pub type UrxRtoTimer = crate::Reg<urx_rto_timer::UrxRtoTimerSpec>;
#[doc = "urx_rto_timer."]
pub mod urx_rto_timer;
#[doc = "uart_int_sts (r) register accessor: UART interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_sts`] module"]
#[doc(alias = "uart_int_sts")]
pub type UartIntSts = crate::Reg<uart_int_sts::UartIntStsSpec>;
#[doc = "UART interrupt status"]
pub mod uart_int_sts;
#[doc = "uart_int_mask (rw) register accessor: UART interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_mask`] module"]
#[doc(alias = "uart_int_mask")]
pub type UartIntMask = crate::Reg<uart_int_mask::UartIntMaskSpec>;
#[doc = "UART interrupt mask"]
pub mod uart_int_mask;
#[doc = "uart_int_clear (rw) register accessor: UART interrupt clear\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_clear`] module"]
#[doc(alias = "uart_int_clear")]
pub type UartIntClear = crate::Reg<uart_int_clear::UartIntClearSpec>;
#[doc = "UART interrupt clear"]
pub mod uart_int_clear;
#[doc = "uart_int_en (rw) register accessor: UART interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_en`] module"]
#[doc(alias = "uart_int_en")]
pub type UartIntEn = crate::Reg<uart_int_en::UartIntEnSpec>;
#[doc = "UART interrupt enable"]
pub mod uart_int_en;
#[doc = "uart_status (r) register accessor: uart_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_status`] module"]
#[doc(alias = "uart_status")]
pub type UartStatus = crate::Reg<uart_status::UartStatusSpec>;
#[doc = "uart_status."]
pub mod uart_status;
#[doc = "sts_urx_abr_prd (r) register accessor: sts_urx_abr_prd.\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_urx_abr_prd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_urx_abr_prd`] module"]
#[doc(alias = "sts_urx_abr_prd")]
pub type StsUrxAbrPrd = crate::Reg<sts_urx_abr_prd::StsUrxAbrPrdSpec>;
#[doc = "sts_urx_abr_prd."]
pub mod sts_urx_abr_prd;
#[doc = "uart_fifo_config_0 (rw) register accessor: uart_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_config_0`] module"]
#[doc(alias = "uart_fifo_config_0")]
pub type UartFifoConfig0 = crate::Reg<uart_fifo_config_0::UartFifoConfig0Spec>;
#[doc = "uart_fifo_config_0."]
pub mod uart_fifo_config_0;
#[doc = "uart_fifo_config_1 (rw) register accessor: uart_fifo_config_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_config_1`] module"]
#[doc(alias = "uart_fifo_config_1")]
pub type UartFifoConfig1 = crate::Reg<uart_fifo_config_1::UartFifoConfig1Spec>;
#[doc = "uart_fifo_config_1."]
pub mod uart_fifo_config_1;
#[doc = "uart_fifo_wdata (w) register accessor: uart_fifo_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_wdata`] module"]
#[doc(alias = "uart_fifo_wdata")]
pub type UartFifoWdata = crate::Reg<uart_fifo_wdata::UartFifoWdataSpec>;
#[doc = "uart_fifo_wdata."]
pub mod uart_fifo_wdata;
#[doc = "uart_fifo_rdata (r) register accessor: uart_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_rdata`] module"]
#[doc(alias = "uart_fifo_rdata")]
pub type UartFifoRdata = crate::Reg<uart_fifo_rdata::UartFifoRdataSpec>;
#[doc = "uart_fifo_rdata."]
pub mod uart_fifo_rdata;
