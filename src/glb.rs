#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_cfg0: ClkCfg0,
    clk_cfg1: ClkCfg1,
    clk_cfg2: ClkCfg2,
    clk_cfg3: ClkCfg3,
    swrst_cfg0: SwrstCfg0,
    swrst_cfg1: SwrstCfg1,
    swrst_cfg2: SwrstCfg2,
    swrst_cfg3: SwrstCfg3,
    cgen_cfg0: CgenCfg0,
    cgen_cfg1: CgenCfg1,
    cgen_cfg2: CgenCfg2,
    cgen_cfg3: CgenCfg3,
    mbist_ctl: MbistCtl,
    mbist_stat: MbistStat,
    _reserved14: [u8; 0x18],
    bmx_cfg1: BmxCfg1,
    bmx_cfg2: BmxCfg2,
    bmx_err_addr: BmxErrAddr,
    bmx_dbg_out: BmxDbgOut,
    rsv0: Rsv0,
    rsv1: Rsv1,
    rsv2: Rsv2,
    rsv3: Rsv3,
    sram_ret: SramRet,
    sram_slp: SramSlp,
    sram_parm: SramParm,
    seam_misc: SeamMisc,
    glb_parm: GlbParm,
    _reserved27: [u8; 0x0c],
    cpu_clk_cfg: CpuClkCfg,
    _reserved28: [u8; 0x10],
    gpadc_32m_src_ctrl: Gpadc32mSrcCtrl,
    dig32k_wakeup_ctrl: Dig32kWakeupCtrl,
    wifi_bt_coex_ctrl: WifiBtCoexCtrl,
    _reserved31: [u8; 0x10],
    uart_sig_sel_0: UartSigSel0,
    _reserved32: [u8; 0x0c],
    dbg_sel_ll: DbgSelLl,
    dbg_sel_lh: DbgSelLh,
    dbg_sel_hl: DbgSelHl,
    dbg_sel_hh: DbgSelHh,
    debug: Debug,
    _reserved37: [u8; 0x1c],
    gpio_cfgctl0: GpioCfgctl0,
    gpio_cfgctl1: GpioCfgctl1,
    gpio_cfgctl2: GpioCfgctl2,
    gpio_cfgctl3: GpioCfgctl3,
    gpio_cfgctl4: GpioCfgctl4,
    gpio_cfgctl5: GpioCfgctl5,
    gpio_cfgctl6: GpioCfgctl6,
    gpio_cfgctl7: GpioCfgctl7,
    gpio_cfgctl8: GpioCfgctl8,
    gpio_cfgctl9: GpioCfgctl9,
    gpio_cfgctl10: GpioCfgctl10,
    gpio_cfgctl11: GpioCfgctl11,
    gpio_cfgctl12: GpioCfgctl12,
    gpio_cfgctl13: GpioCfgctl13,
    gpio_cfgctl14: GpioCfgctl14,
    _reserved52: [u8; 0x44],
    gpio_cfgctl30: GpioCfgctl30,
    gpio_cfgctl31: GpioCfgctl31,
    gpio_cfgctl32: GpioCfgctl32,
    gpio_cfgctl33: GpioCfgctl33,
    gpio_cfgctl34: GpioCfgctl34,
    gpio_cfgctl35: GpioCfgctl35,
    _reserved58: [u8; 0x08],
    gpio_int_mask1: GpioIntMask1,
    _reserved59: [u8; 0x04],
    gpio_int_stat1: GpioIntStat1,
    _reserved60: [u8; 0x04],
    gpio_int_clr1: GpioIntClr1,
    _reserved61: [u8; 0x0c],
    gpio_int_mode_set1: GpioIntModeSet1,
    gpio_int_mode_set2: GpioIntModeSet2,
    gpio_int_mode_set3: GpioIntModeSet3,
    _reserved64: [u8; 0x58],
    led_driver: LedDriver,
    _reserved65: [u8; 0xe0],
    gpdac_ctrl: GpdacCtrl,
    gpdac_actrl: GpdacActrl,
    gpdac_bctrl: GpdacBctrl,
    gpdac_data: GpdacData,
    _reserved69: [u8; 0x0be8],
    tzc_glb_ctrl_0: TzcGlbCtrl0,
    tzc_glb_ctrl_1: TzcGlbCtrl1,
    tzc_glb_ctrl_2: TzcGlbCtrl2,
    tzc_glb_ctrl_3: TzcGlbCtrl3,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock configuration for processor and bus"]
    #[inline(always)]
    pub const fn clk_cfg0(&self) -> &ClkCfg0 {
        &self.clk_cfg0
    }
    #[doc = "0x04 - clk_cfg1."]
    #[inline(always)]
    pub const fn clk_cfg1(&self) -> &ClkCfg1 {
        &self.clk_cfg1
    }
    #[doc = "0x08 - Clock configuration for UART and Flash"]
    #[inline(always)]
    pub const fn clk_cfg2(&self) -> &ClkCfg2 {
        &self.clk_cfg2
    }
    #[doc = "0x0c - Clock configuration for I2C and SPI"]
    #[inline(always)]
    pub const fn clk_cfg3(&self) -> &ClkCfg3 {
        &self.clk_cfg3
    }
    #[doc = "0x10 - swrst_cfg0."]
    #[inline(always)]
    pub const fn swrst_cfg0(&self) -> &SwrstCfg0 {
        &self.swrst_cfg0
    }
    #[doc = "0x14 - swrst_cfg1."]
    #[inline(always)]
    pub const fn swrst_cfg1(&self) -> &SwrstCfg1 {
        &self.swrst_cfg1
    }
    #[doc = "0x18 - swrst_cfg2."]
    #[inline(always)]
    pub const fn swrst_cfg2(&self) -> &SwrstCfg2 {
        &self.swrst_cfg2
    }
    #[doc = "0x1c - swrst_cfg3."]
    #[inline(always)]
    pub const fn swrst_cfg3(&self) -> &SwrstCfg3 {
        &self.swrst_cfg3
    }
    #[doc = "0x20 - cgen_cfg0."]
    #[inline(always)]
    pub const fn cgen_cfg0(&self) -> &CgenCfg0 {
        &self.cgen_cfg0
    }
    #[doc = "0x24 - cgen_cfg1."]
    #[inline(always)]
    pub const fn cgen_cfg1(&self) -> &CgenCfg1 {
        &self.cgen_cfg1
    }
    #[doc = "0x28 - cgen_cfg2."]
    #[inline(always)]
    pub const fn cgen_cfg2(&self) -> &CgenCfg2 {
        &self.cgen_cfg2
    }
    #[doc = "0x2c - cgen_cfg3."]
    #[inline(always)]
    pub const fn cgen_cfg3(&self) -> &CgenCfg3 {
        &self.cgen_cfg3
    }
    #[doc = "0x30 - MBIST_CTL."]
    #[inline(always)]
    pub const fn mbist_ctl(&self) -> &MbistCtl {
        &self.mbist_ctl
    }
    #[doc = "0x34 - MBIST_STAT."]
    #[inline(always)]
    pub const fn mbist_stat(&self) -> &MbistStat {
        &self.mbist_stat
    }
    #[doc = "0x50 - bmx_cfg1."]
    #[inline(always)]
    pub const fn bmx_cfg1(&self) -> &BmxCfg1 {
        &self.bmx_cfg1
    }
    #[doc = "0x54 - bmx_cfg2."]
    #[inline(always)]
    pub const fn bmx_cfg2(&self) -> &BmxCfg2 {
        &self.bmx_cfg2
    }
    #[doc = "0x58 - bmx_err_addr."]
    #[inline(always)]
    pub const fn bmx_err_addr(&self) -> &BmxErrAddr {
        &self.bmx_err_addr
    }
    #[doc = "0x5c - bmx_dbg_out."]
    #[inline(always)]
    pub const fn bmx_dbg_out(&self) -> &BmxDbgOut {
        &self.bmx_dbg_out
    }
    #[doc = "0x60 - rsv0."]
    #[inline(always)]
    pub const fn rsv0(&self) -> &Rsv0 {
        &self.rsv0
    }
    #[doc = "0x64 - rsv1."]
    #[inline(always)]
    pub const fn rsv1(&self) -> &Rsv1 {
        &self.rsv1
    }
    #[doc = "0x68 - rsv2."]
    #[inline(always)]
    pub const fn rsv2(&self) -> &Rsv2 {
        &self.rsv2
    }
    #[doc = "0x6c - rsv3."]
    #[inline(always)]
    pub const fn rsv3(&self) -> &Rsv3 {
        &self.rsv3
    }
    #[doc = "0x70 - sram_ret."]
    #[inline(always)]
    pub const fn sram_ret(&self) -> &SramRet {
        &self.sram_ret
    }
    #[doc = "0x74 - sram_slp."]
    #[inline(always)]
    pub const fn sram_slp(&self) -> &SramSlp {
        &self.sram_slp
    }
    #[doc = "0x78 - sram_parm."]
    #[inline(always)]
    pub const fn sram_parm(&self) -> &SramParm {
        &self.sram_parm
    }
    #[doc = "0x7c - seam_misc."]
    #[inline(always)]
    pub const fn seam_misc(&self) -> &SeamMisc {
        &self.seam_misc
    }
    #[doc = "0x80 - glb_parm."]
    #[inline(always)]
    pub const fn glb_parm(&self) -> &GlbParm {
        &self.glb_parm
    }
    #[doc = "0x90 - CPU_CLK_CFG."]
    #[inline(always)]
    pub const fn cpu_clk_cfg(&self) -> &CpuClkCfg {
        &self.cpu_clk_cfg
    }
    #[doc = "0xa4 - Clock configuration for GPADC"]
    #[inline(always)]
    pub const fn gpadc_32m_src_ctrl(&self) -> &Gpadc32mSrcCtrl {
        &self.gpadc_32m_src_ctrl
    }
    #[doc = "0xa8 - DIG32K_WAKEUP_CTRL."]
    #[inline(always)]
    pub const fn dig32k_wakeup_ctrl(&self) -> &Dig32kWakeupCtrl {
        &self.dig32k_wakeup_ctrl
    }
    #[doc = "0xac - WIFI_BT_COEX_CTRL."]
    #[inline(always)]
    pub const fn wifi_bt_coex_ctrl(&self) -> &WifiBtCoexCtrl {
        &self.wifi_bt_coex_ctrl
    }
    #[doc = "0xc0 - UART_SIG_SEL_0."]
    #[inline(always)]
    pub const fn uart_sig_sel_0(&self) -> &UartSigSel0 {
        &self.uart_sig_sel_0
    }
    #[doc = "0xd0 - DBG_SEL_LL."]
    #[inline(always)]
    pub const fn dbg_sel_ll(&self) -> &DbgSelLl {
        &self.dbg_sel_ll
    }
    #[doc = "0xd4 - DBG_SEL_LH."]
    #[inline(always)]
    pub const fn dbg_sel_lh(&self) -> &DbgSelLh {
        &self.dbg_sel_lh
    }
    #[doc = "0xd8 - DBG_SEL_HL."]
    #[inline(always)]
    pub const fn dbg_sel_hl(&self) -> &DbgSelHl {
        &self.dbg_sel_hl
    }
    #[doc = "0xdc - DBG_SEL_HH."]
    #[inline(always)]
    pub const fn dbg_sel_hh(&self) -> &DbgSelHh {
        &self.dbg_sel_hh
    }
    #[doc = "0xe0 - debug."]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x100 - GPIO0, GPIO1 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl0(&self) -> &GpioCfgctl0 {
        &self.gpio_cfgctl0
    }
    #[doc = "0x104 - GPIO2, GPIO3 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl1(&self) -> &GpioCfgctl1 {
        &self.gpio_cfgctl1
    }
    #[doc = "0x108 - GPIO4, GPIO5 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl2(&self) -> &GpioCfgctl2 {
        &self.gpio_cfgctl2
    }
    #[doc = "0x10c - GPIO6, GPIO7 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl3(&self) -> &GpioCfgctl3 {
        &self.gpio_cfgctl3
    }
    #[doc = "0x110 - GPIO8, GPIO9 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl4(&self) -> &GpioCfgctl4 {
        &self.gpio_cfgctl4
    }
    #[doc = "0x114 - GPIO10, GPIO11 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl5(&self) -> &GpioCfgctl5 {
        &self.gpio_cfgctl5
    }
    #[doc = "0x118 - GPIO12, GPIO13 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl6(&self) -> &GpioCfgctl6 {
        &self.gpio_cfgctl6
    }
    #[doc = "0x11c - GPIO14, GPIO15 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl7(&self) -> &GpioCfgctl7 {
        &self.gpio_cfgctl7
    }
    #[doc = "0x120 - GPIO16, GPIO17 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl8(&self) -> &GpioCfgctl8 {
        &self.gpio_cfgctl8
    }
    #[doc = "0x124 - GPIO18, GPIO19 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl9(&self) -> &GpioCfgctl9 {
        &self.gpio_cfgctl9
    }
    #[doc = "0x128 - GPIO20, GPIO21 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl10(&self) -> &GpioCfgctl10 {
        &self.gpio_cfgctl10
    }
    #[doc = "0x12c - GPIO22, GPIO23 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl11(&self) -> &GpioCfgctl11 {
        &self.gpio_cfgctl11
    }
    #[doc = "0x130 - GPIO24, GPIO25 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl12(&self) -> &GpioCfgctl12 {
        &self.gpio_cfgctl12
    }
    #[doc = "0x134 - GPIO26, GPIO27 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl13(&self) -> &GpioCfgctl13 {
        &self.gpio_cfgctl13
    }
    #[doc = "0x138 - GPIO28 configuration"]
    #[inline(always)]
    pub const fn gpio_cfgctl14(&self) -> &GpioCfgctl14 {
        &self.gpio_cfgctl14
    }
    #[doc = "0x180 - Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work."]
    #[inline(always)]
    pub const fn gpio_cfgctl30(&self) -> &GpioCfgctl30 {
        &self.gpio_cfgctl30
    }
    #[doc = "0x184 - Reserved according to SDK."]
    #[inline(always)]
    pub const fn gpio_cfgctl31(&self) -> &GpioCfgctl31 {
        &self.gpio_cfgctl31
    }
    #[doc = "0x188 - Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work."]
    #[inline(always)]
    pub const fn gpio_cfgctl32(&self) -> &GpioCfgctl32 {
        &self.gpio_cfgctl32
    }
    #[doc = "0x18c - Reserved according to SDK."]
    #[inline(always)]
    pub const fn gpio_cfgctl33(&self) -> &GpioCfgctl33 {
        &self.gpio_cfgctl33
    }
    #[doc = "0x190 - Output enable register for GPIO."]
    #[inline(always)]
    pub const fn gpio_cfgctl34(&self) -> &GpioCfgctl34 {
        &self.gpio_cfgctl34
    }
    #[doc = "0x194 - Reserved according to SDK."]
    #[inline(always)]
    pub const fn gpio_cfgctl35(&self) -> &GpioCfgctl35 {
        &self.gpio_cfgctl35
    }
    #[doc = "0x1a0 - Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
    #[inline(always)]
    pub const fn gpio_int_mask1(&self) -> &GpioIntMask1 {
        &self.gpio_int_mask1
    }
    #[doc = "0x1a8 - Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
    #[inline(always)]
    pub const fn gpio_int_stat1(&self) -> &GpioIntStat1 {
        &self.gpio_int_stat1
    }
    #[doc = "0x1b0 - Interrupt clearing register."]
    #[inline(always)]
    pub const fn gpio_int_clr1(&self) -> &GpioIntClr1 {
        &self.gpio_int_clr1
    }
    #[doc = "0x1c0 - GPIO interrupt trigger and control register for GPIO0-GPIO9."]
    #[inline(always)]
    pub const fn gpio_int_mode_set1(&self) -> &GpioIntModeSet1 {
        &self.gpio_int_mode_set1
    }
    #[doc = "0x1c4 - GPIO interrupt trigger and control register for GPIO10-GPIO19."]
    #[inline(always)]
    pub const fn gpio_int_mode_set2(&self) -> &GpioIntModeSet2 {
        &self.gpio_int_mode_set2
    }
    #[doc = "0x1c8 - GPIO interrupt trigger and control register for GPIO20-GPIO29."]
    #[inline(always)]
    pub const fn gpio_int_mode_set3(&self) -> &GpioIntModeSet3 {
        &self.gpio_int_mode_set3
    }
    #[doc = "0x224 - led_driver."]
    #[inline(always)]
    pub const fn led_driver(&self) -> &LedDriver {
        &self.led_driver
    }
    #[doc = "0x308 - gpdac_ctrl."]
    #[inline(always)]
    pub const fn gpdac_ctrl(&self) -> &GpdacCtrl {
        &self.gpdac_ctrl
    }
    #[doc = "0x30c - gpdac_actrl."]
    #[inline(always)]
    pub const fn gpdac_actrl(&self) -> &GpdacActrl {
        &self.gpdac_actrl
    }
    #[doc = "0x310 - gpdac_bctrl."]
    #[inline(always)]
    pub const fn gpdac_bctrl(&self) -> &GpdacBctrl {
        &self.gpdac_bctrl
    }
    #[doc = "0x314 - gpdac_data."]
    #[inline(always)]
    pub const fn gpdac_data(&self) -> &GpdacData {
        &self.gpdac_data
    }
    #[doc = "0xf00 - tzc_glb_ctrl_0."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_0(&self) -> &TzcGlbCtrl0 {
        &self.tzc_glb_ctrl_0
    }
    #[doc = "0xf04 - tzc_glb_ctrl_1."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_1(&self) -> &TzcGlbCtrl1 {
        &self.tzc_glb_ctrl_1
    }
    #[doc = "0xf08 - tzc_glb_ctrl_2."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_2(&self) -> &TzcGlbCtrl2 {
        &self.tzc_glb_ctrl_2
    }
    #[doc = "0xf0c - Reserved according to SDK."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_3(&self) -> &TzcGlbCtrl3 {
        &self.tzc_glb_ctrl_3
    }
}
#[doc = "clk_cfg0 (rw) register accessor: Clock configuration for processor and bus\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg0`] module"]
#[doc(alias = "clk_cfg0")]
pub type ClkCfg0 = crate::Reg<clk_cfg0::ClkCfg0Spec>;
#[doc = "Clock configuration for processor and bus"]
pub mod clk_cfg0;
#[doc = "clk_cfg1 (rw) register accessor: clk_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg1`] module"]
#[doc(alias = "clk_cfg1")]
pub type ClkCfg1 = crate::Reg<clk_cfg1::ClkCfg1Spec>;
#[doc = "clk_cfg1."]
pub mod clk_cfg1;
#[doc = "clk_cfg2 (rw) register accessor: Clock configuration for UART and Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg2`] module"]
#[doc(alias = "clk_cfg2")]
pub type ClkCfg2 = crate::Reg<clk_cfg2::ClkCfg2Spec>;
#[doc = "Clock configuration for UART and Flash"]
pub mod clk_cfg2;
#[doc = "clk_cfg3 (rw) register accessor: Clock configuration for I2C and SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg3`] module"]
#[doc(alias = "clk_cfg3")]
pub type ClkCfg3 = crate::Reg<clk_cfg3::ClkCfg3Spec>;
#[doc = "Clock configuration for I2C and SPI"]
pub mod clk_cfg3;
#[doc = "swrst_cfg0 (rw) register accessor: swrst_cfg0.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg0`] module"]
#[doc(alias = "swrst_cfg0")]
pub type SwrstCfg0 = crate::Reg<swrst_cfg0::SwrstCfg0Spec>;
#[doc = "swrst_cfg0."]
pub mod swrst_cfg0;
#[doc = "swrst_cfg1 (rw) register accessor: swrst_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg1`] module"]
#[doc(alias = "swrst_cfg1")]
pub type SwrstCfg1 = crate::Reg<swrst_cfg1::SwrstCfg1Spec>;
#[doc = "swrst_cfg1."]
pub mod swrst_cfg1;
#[doc = "swrst_cfg2 (rw) register accessor: swrst_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg2`] module"]
#[doc(alias = "swrst_cfg2")]
pub type SwrstCfg2 = crate::Reg<swrst_cfg2::SwrstCfg2Spec>;
#[doc = "swrst_cfg2."]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3 (rw) register accessor: swrst_cfg3.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg3`] module"]
#[doc(alias = "swrst_cfg3")]
pub type SwrstCfg3 = crate::Reg<swrst_cfg3::SwrstCfg3Spec>;
#[doc = "swrst_cfg3."]
pub mod swrst_cfg3;
#[doc = "cgen_cfg0 (rw) register accessor: cgen_cfg0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg0`] module"]
#[doc(alias = "cgen_cfg0")]
pub type CgenCfg0 = crate::Reg<cgen_cfg0::CgenCfg0Spec>;
#[doc = "cgen_cfg0."]
pub mod cgen_cfg0;
#[doc = "cgen_cfg1 (rw) register accessor: cgen_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg1`] module"]
#[doc(alias = "cgen_cfg1")]
pub type CgenCfg1 = crate::Reg<cgen_cfg1::CgenCfg1Spec>;
#[doc = "cgen_cfg1."]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2 (rw) register accessor: cgen_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg2`] module"]
#[doc(alias = "cgen_cfg2")]
pub type CgenCfg2 = crate::Reg<cgen_cfg2::CgenCfg2Spec>;
#[doc = "cgen_cfg2."]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3 (rw) register accessor: cgen_cfg3.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg3`] module"]
#[doc(alias = "cgen_cfg3")]
pub type CgenCfg3 = crate::Reg<cgen_cfg3::CgenCfg3Spec>;
#[doc = "cgen_cfg3."]
pub mod cgen_cfg3;
#[doc = "MBIST_CTL (rw) register accessor: MBIST_CTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbist_ctl`] module"]
#[doc(alias = "MBIST_CTL")]
pub type MbistCtl = crate::Reg<mbist_ctl::MbistCtlSpec>;
#[doc = "MBIST_CTL."]
pub mod mbist_ctl;
#[doc = "MBIST_STAT (r) register accessor: MBIST_STAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbist_stat`] module"]
#[doc(alias = "MBIST_STAT")]
pub type MbistStat = crate::Reg<mbist_stat::MbistStatSpec>;
#[doc = "MBIST_STAT."]
pub mod mbist_stat;
#[doc = "bmx_cfg1 (rw) register accessor: bmx_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg1`] module"]
#[doc(alias = "bmx_cfg1")]
pub type BmxCfg1 = crate::Reg<bmx_cfg1::BmxCfg1Spec>;
#[doc = "bmx_cfg1."]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2 (rw) register accessor: bmx_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg2`] module"]
#[doc(alias = "bmx_cfg2")]
pub type BmxCfg2 = crate::Reg<bmx_cfg2::BmxCfg2Spec>;
#[doc = "bmx_cfg2."]
pub mod bmx_cfg2;
#[doc = "bmx_err_addr (r) register accessor: bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_err_addr`] module"]
#[doc(alias = "bmx_err_addr")]
pub type BmxErrAddr = crate::Reg<bmx_err_addr::BmxErrAddrSpec>;
#[doc = "bmx_err_addr."]
pub mod bmx_err_addr;
#[doc = "bmx_dbg_out (r) register accessor: bmx_dbg_out.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_dbg_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_dbg_out`] module"]
#[doc(alias = "bmx_dbg_out")]
pub type BmxDbgOut = crate::Reg<bmx_dbg_out::BmxDbgOutSpec>;
#[doc = "bmx_dbg_out."]
pub mod bmx_dbg_out;
#[doc = "rsv0 (rw) register accessor: rsv0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv0`] module"]
#[doc(alias = "rsv0")]
pub type Rsv0 = crate::Reg<rsv0::Rsv0Spec>;
#[doc = "rsv0."]
pub mod rsv0;
#[doc = "rsv1 (rw) register accessor: rsv1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv1`] module"]
#[doc(alias = "rsv1")]
pub type Rsv1 = crate::Reg<rsv1::Rsv1Spec>;
#[doc = "rsv1."]
pub mod rsv1;
#[doc = "rsv2 (rw) register accessor: rsv2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv2`] module"]
#[doc(alias = "rsv2")]
pub type Rsv2 = crate::Reg<rsv2::Rsv2Spec>;
#[doc = "rsv2."]
pub mod rsv2;
#[doc = "rsv3 (rw) register accessor: rsv3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv3`] module"]
#[doc(alias = "rsv3")]
pub type Rsv3 = crate::Reg<rsv3::Rsv3Spec>;
#[doc = "rsv3."]
pub mod rsv3;
#[doc = "sram_ret (rw) register accessor: sram_ret.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ret`] module"]
#[doc(alias = "sram_ret")]
pub type SramRet = crate::Reg<sram_ret::SramRetSpec>;
#[doc = "sram_ret."]
pub mod sram_ret;
#[doc = "sram_slp (rw) register accessor: sram_slp.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_slp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_slp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_slp`] module"]
#[doc(alias = "sram_slp")]
pub type SramSlp = crate::Reg<sram_slp::SramSlpSpec>;
#[doc = "sram_slp."]
pub mod sram_slp;
#[doc = "sram_parm (rw) register accessor: sram_parm.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_parm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_parm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_parm`] module"]
#[doc(alias = "sram_parm")]
pub type SramParm = crate::Reg<sram_parm::SramParmSpec>;
#[doc = "sram_parm."]
pub mod sram_parm;
#[doc = "seam_misc (rw) register accessor: seam_misc.\n\nYou can [`read`](crate::Reg::read) this register and get [`seam_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seam_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seam_misc`] module"]
#[doc(alias = "seam_misc")]
pub type SeamMisc = crate::Reg<seam_misc::SeamMiscSpec>;
#[doc = "seam_misc."]
pub mod seam_misc;
#[doc = "glb_parm (rw) register accessor: glb_parm.\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_parm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_parm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_parm`] module"]
#[doc(alias = "glb_parm")]
pub type GlbParm = crate::Reg<glb_parm::GlbParmSpec>;
#[doc = "glb_parm."]
pub mod glb_parm;
#[doc = "CPU_CLK_CFG (rw) register accessor: CPU_CLK_CFG.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_clk_cfg`] module"]
#[doc(alias = "CPU_CLK_CFG")]
pub type CpuClkCfg = crate::Reg<cpu_clk_cfg::CpuClkCfgSpec>;
#[doc = "CPU_CLK_CFG."]
pub mod cpu_clk_cfg;
#[doc = "GPADC_32M_SRC_CTRL (rw) register accessor: Clock configuration for GPADC\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_32m_src_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_32m_src_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_32m_src_ctrl`] module"]
#[doc(alias = "GPADC_32M_SRC_CTRL")]
pub type Gpadc32mSrcCtrl = crate::Reg<gpadc_32m_src_ctrl::Gpadc32mSrcCtrlSpec>;
#[doc = "Clock configuration for GPADC"]
pub mod gpadc_32m_src_ctrl;
#[doc = "DIG32K_WAKEUP_CTRL (rw) register accessor: DIG32K_WAKEUP_CTRL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dig32k_wakeup_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig32k_wakeup_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig32k_wakeup_ctrl`] module"]
#[doc(alias = "DIG32K_WAKEUP_CTRL")]
pub type Dig32kWakeupCtrl = crate::Reg<dig32k_wakeup_ctrl::Dig32kWakeupCtrlSpec>;
#[doc = "DIG32K_WAKEUP_CTRL."]
pub mod dig32k_wakeup_ctrl;
#[doc = "WIFI_BT_COEX_CTRL (rw) register accessor: WIFI_BT_COEX_CTRL.\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_bt_coex_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_bt_coex_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bt_coex_ctrl`] module"]
#[doc(alias = "WIFI_BT_COEX_CTRL")]
pub type WifiBtCoexCtrl = crate::Reg<wifi_bt_coex_ctrl::WifiBtCoexCtrlSpec>;
#[doc = "WIFI_BT_COEX_CTRL."]
pub mod wifi_bt_coex_ctrl;
#[doc = "UART_SIG_SEL_0 (rw) register accessor: UART_SIG_SEL_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sig_sel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sig_sel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sig_sel_0`] module"]
#[doc(alias = "UART_SIG_SEL_0")]
pub type UartSigSel0 = crate::Reg<uart_sig_sel_0::UartSigSel0Spec>;
#[doc = "UART_SIG_SEL_0."]
pub mod uart_sig_sel_0;
#[doc = "DBG_SEL_LL (rw) register accessor: DBG_SEL_LL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_ll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_ll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel_ll`] module"]
#[doc(alias = "DBG_SEL_LL")]
pub type DbgSelLl = crate::Reg<dbg_sel_ll::DbgSelLlSpec>;
#[doc = "DBG_SEL_LL."]
pub mod dbg_sel_ll;
#[doc = "DBG_SEL_LH (rw) register accessor: DBG_SEL_LH.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_lh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_lh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel_lh`] module"]
#[doc(alias = "DBG_SEL_LH")]
pub type DbgSelLh = crate::Reg<dbg_sel_lh::DbgSelLhSpec>;
#[doc = "DBG_SEL_LH."]
pub mod dbg_sel_lh;
#[doc = "DBG_SEL_HL (rw) register accessor: DBG_SEL_HL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_hl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_hl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel_hl`] module"]
#[doc(alias = "DBG_SEL_HL")]
pub type DbgSelHl = crate::Reg<dbg_sel_hl::DbgSelHlSpec>;
#[doc = "DBG_SEL_HL."]
pub mod dbg_sel_hl;
#[doc = "DBG_SEL_HH (rw) register accessor: DBG_SEL_HH.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_hh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_hh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel_hh`] module"]
#[doc(alias = "DBG_SEL_HH")]
pub type DbgSelHh = crate::Reg<dbg_sel_hh::DbgSelHhSpec>;
#[doc = "DBG_SEL_HH."]
pub mod dbg_sel_hh;
#[doc = "debug (rw) register accessor: debug.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`] module"]
#[doc(alias = "debug")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "debug."]
pub mod debug;
#[doc = "GPIO_CFGCTL0 (rw) register accessor: GPIO0, GPIO1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl0`] module"]
#[doc(alias = "GPIO_CFGCTL0")]
pub type GpioCfgctl0 = crate::Reg<gpio_cfgctl0::GpioCfgctl0Spec>;
#[doc = "GPIO0, GPIO1 configuration"]
pub mod gpio_cfgctl0;
#[doc = "GPIO_CFGCTL1 (rw) register accessor: GPIO2, GPIO3 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl1`] module"]
#[doc(alias = "GPIO_CFGCTL1")]
pub type GpioCfgctl1 = crate::Reg<gpio_cfgctl1::GpioCfgctl1Spec>;
#[doc = "GPIO2, GPIO3 configuration"]
pub mod gpio_cfgctl1;
#[doc = "GPIO_CFGCTL2 (rw) register accessor: GPIO4, GPIO5 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl2`] module"]
#[doc(alias = "GPIO_CFGCTL2")]
pub type GpioCfgctl2 = crate::Reg<gpio_cfgctl2::GpioCfgctl2Spec>;
#[doc = "GPIO4, GPIO5 configuration"]
pub mod gpio_cfgctl2;
#[doc = "GPIO_CFGCTL3 (rw) register accessor: GPIO6, GPIO7 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl3`] module"]
#[doc(alias = "GPIO_CFGCTL3")]
pub type GpioCfgctl3 = crate::Reg<gpio_cfgctl3::GpioCfgctl3Spec>;
#[doc = "GPIO6, GPIO7 configuration"]
pub mod gpio_cfgctl3;
#[doc = "GPIO_CFGCTL4 (rw) register accessor: GPIO8, GPIO9 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl4`] module"]
#[doc(alias = "GPIO_CFGCTL4")]
pub type GpioCfgctl4 = crate::Reg<gpio_cfgctl4::GpioCfgctl4Spec>;
#[doc = "GPIO8, GPIO9 configuration"]
pub mod gpio_cfgctl4;
#[doc = "GPIO_CFGCTL5 (rw) register accessor: GPIO10, GPIO11 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl5`] module"]
#[doc(alias = "GPIO_CFGCTL5")]
pub type GpioCfgctl5 = crate::Reg<gpio_cfgctl5::GpioCfgctl5Spec>;
#[doc = "GPIO10, GPIO11 configuration"]
pub mod gpio_cfgctl5;
#[doc = "GPIO_CFGCTL6 (rw) register accessor: GPIO12, GPIO13 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl6`] module"]
#[doc(alias = "GPIO_CFGCTL6")]
pub type GpioCfgctl6 = crate::Reg<gpio_cfgctl6::GpioCfgctl6Spec>;
#[doc = "GPIO12, GPIO13 configuration"]
pub mod gpio_cfgctl6;
#[doc = "GPIO_CFGCTL7 (rw) register accessor: GPIO14, GPIO15 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl7`] module"]
#[doc(alias = "GPIO_CFGCTL7")]
pub type GpioCfgctl7 = crate::Reg<gpio_cfgctl7::GpioCfgctl7Spec>;
#[doc = "GPIO14, GPIO15 configuration"]
pub mod gpio_cfgctl7;
#[doc = "GPIO_CFGCTL8 (rw) register accessor: GPIO16, GPIO17 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl8`] module"]
#[doc(alias = "GPIO_CFGCTL8")]
pub type GpioCfgctl8 = crate::Reg<gpio_cfgctl8::GpioCfgctl8Spec>;
#[doc = "GPIO16, GPIO17 configuration"]
pub mod gpio_cfgctl8;
#[doc = "GPIO_CFGCTL9 (rw) register accessor: GPIO18, GPIO19 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl9`] module"]
#[doc(alias = "GPIO_CFGCTL9")]
pub type GpioCfgctl9 = crate::Reg<gpio_cfgctl9::GpioCfgctl9Spec>;
#[doc = "GPIO18, GPIO19 configuration"]
pub mod gpio_cfgctl9;
#[doc = "GPIO_CFGCTL10 (rw) register accessor: GPIO20, GPIO21 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl10`] module"]
#[doc(alias = "GPIO_CFGCTL10")]
pub type GpioCfgctl10 = crate::Reg<gpio_cfgctl10::GpioCfgctl10Spec>;
#[doc = "GPIO20, GPIO21 configuration"]
pub mod gpio_cfgctl10;
#[doc = "GPIO_CFGCTL11 (rw) register accessor: GPIO22, GPIO23 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl11`] module"]
#[doc(alias = "GPIO_CFGCTL11")]
pub type GpioCfgctl11 = crate::Reg<gpio_cfgctl11::GpioCfgctl11Spec>;
#[doc = "GPIO22, GPIO23 configuration"]
pub mod gpio_cfgctl11;
#[doc = "GPIO_CFGCTL12 (rw) register accessor: GPIO24, GPIO25 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl12`] module"]
#[doc(alias = "GPIO_CFGCTL12")]
pub type GpioCfgctl12 = crate::Reg<gpio_cfgctl12::GpioCfgctl12Spec>;
#[doc = "GPIO24, GPIO25 configuration"]
pub mod gpio_cfgctl12;
#[doc = "GPIO_CFGCTL13 (rw) register accessor: GPIO26, GPIO27 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl13`] module"]
#[doc(alias = "GPIO_CFGCTL13")]
pub type GpioCfgctl13 = crate::Reg<gpio_cfgctl13::GpioCfgctl13Spec>;
#[doc = "GPIO26, GPIO27 configuration"]
pub mod gpio_cfgctl13;
#[doc = "GPIO_CFGCTL14 (rw) register accessor: GPIO28 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl14`] module"]
#[doc(alias = "GPIO_CFGCTL14")]
pub type GpioCfgctl14 = crate::Reg<gpio_cfgctl14::GpioCfgctl14Spec>;
#[doc = "GPIO28 configuration"]
pub mod gpio_cfgctl14;
#[doc = "GPIO_CFGCTL30 (r) register accessor: Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl30`] module"]
#[doc(alias = "GPIO_CFGCTL30")]
pub type GpioCfgctl30 = crate::Reg<gpio_cfgctl30::GpioCfgctl30Spec>;
#[doc = "Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work."]
pub mod gpio_cfgctl30;
#[doc = "GPIO_CFGCTL31 (rw) register accessor: Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl31`] module"]
#[doc(alias = "GPIO_CFGCTL31")]
pub type GpioCfgctl31 = crate::Reg<gpio_cfgctl31::GpioCfgctl31Spec>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl31;
#[doc = "GPIO_CFGCTL32 (rw) register accessor: Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl32`] module"]
#[doc(alias = "GPIO_CFGCTL32")]
pub type GpioCfgctl32 = crate::Reg<gpio_cfgctl32::GpioCfgctl32Spec>;
#[doc = "Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work."]
pub mod gpio_cfgctl32;
#[doc = "GPIO_CFGCTL33 (rw) register accessor: Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl33`] module"]
#[doc(alias = "GPIO_CFGCTL33")]
pub type GpioCfgctl33 = crate::Reg<gpio_cfgctl33::GpioCfgctl33Spec>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl33;
#[doc = "GPIO_CFGCTL34 (rw) register accessor: Output enable register for GPIO.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl34`] module"]
#[doc(alias = "GPIO_CFGCTL34")]
pub type GpioCfgctl34 = crate::Reg<gpio_cfgctl34::GpioCfgctl34Spec>;
#[doc = "Output enable register for GPIO."]
pub mod gpio_cfgctl34;
#[doc = "GPIO_CFGCTL35 (rw) register accessor: Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfgctl35`] module"]
#[doc(alias = "GPIO_CFGCTL35")]
pub type GpioCfgctl35 = crate::Reg<gpio_cfgctl35::GpioCfgctl35Spec>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl35;
#[doc = "GPIO_INT_MASK1 (rw) register accessor: Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_mask1`] module"]
#[doc(alias = "GPIO_INT_MASK1")]
pub type GpioIntMask1 = crate::Reg<gpio_int_mask1::GpioIntMask1Spec>;
#[doc = "Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
pub mod gpio_int_mask1;
#[doc = "GPIO_INT_STAT1 (r) register accessor: Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_stat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_stat1`] module"]
#[doc(alias = "GPIO_INT_STAT1")]
pub type GpioIntStat1 = crate::Reg<gpio_int_stat1::GpioIntStat1Spec>;
#[doc = "Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
pub mod gpio_int_stat1;
#[doc = "GPIO_INT_CLR1 (rw) register accessor: Interrupt clearing register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_clr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_clr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_clr1`] module"]
#[doc(alias = "GPIO_INT_CLR1")]
pub type GpioIntClr1 = crate::Reg<gpio_int_clr1::GpioIntClr1Spec>;
#[doc = "Interrupt clearing register."]
pub mod gpio_int_clr1;
#[doc = "GPIO_INT_MODE_SET1 (rw) register accessor: GPIO interrupt trigger and control register for GPIO0-GPIO9.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_mode_set1`] module"]
#[doc(alias = "GPIO_INT_MODE_SET1")]
pub type GpioIntModeSet1 = crate::Reg<gpio_int_mode_set1::GpioIntModeSet1Spec>;
#[doc = "GPIO interrupt trigger and control register for GPIO0-GPIO9."]
pub mod gpio_int_mode_set1;
#[doc = "GPIO_INT_MODE_SET2 (rw) register accessor: GPIO interrupt trigger and control register for GPIO10-GPIO19.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_mode_set2`] module"]
#[doc(alias = "GPIO_INT_MODE_SET2")]
pub type GpioIntModeSet2 = crate::Reg<gpio_int_mode_set2::GpioIntModeSet2Spec>;
#[doc = "GPIO interrupt trigger and control register for GPIO10-GPIO19."]
pub mod gpio_int_mode_set2;
#[doc = "GPIO_INT_MODE_SET3 (rw) register accessor: GPIO interrupt trigger and control register for GPIO20-GPIO29.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_mode_set3`] module"]
#[doc(alias = "GPIO_INT_MODE_SET3")]
pub type GpioIntModeSet3 = crate::Reg<gpio_int_mode_set3::GpioIntModeSet3Spec>;
#[doc = "GPIO interrupt trigger and control register for GPIO20-GPIO29."]
pub mod gpio_int_mode_set3;
#[doc = "led_driver (rw) register accessor: led_driver.\n\nYou can [`read`](crate::Reg::read) this register and get [`led_driver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led_driver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led_driver`] module"]
#[doc(alias = "led_driver")]
pub type LedDriver = crate::Reg<led_driver::LedDriverSpec>;
#[doc = "led_driver."]
pub mod led_driver;
#[doc = "gpdac_ctrl (rw) register accessor: gpdac_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_ctrl`] module"]
#[doc(alias = "gpdac_ctrl")]
pub type GpdacCtrl = crate::Reg<gpdac_ctrl::GpdacCtrlSpec>;
#[doc = "gpdac_ctrl."]
pub mod gpdac_ctrl;
#[doc = "gpdac_actrl (rw) register accessor: gpdac_actrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_actrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_actrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_actrl`] module"]
#[doc(alias = "gpdac_actrl")]
pub type GpdacActrl = crate::Reg<gpdac_actrl::GpdacActrlSpec>;
#[doc = "gpdac_actrl."]
pub mod gpdac_actrl;
#[doc = "gpdac_bctrl (rw) register accessor: gpdac_bctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_bctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_bctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_bctrl`] module"]
#[doc(alias = "gpdac_bctrl")]
pub type GpdacBctrl = crate::Reg<gpdac_bctrl::GpdacBctrlSpec>;
#[doc = "gpdac_bctrl."]
pub mod gpdac_bctrl;
#[doc = "gpdac_data (rw) register accessor: gpdac_data.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_data`] module"]
#[doc(alias = "gpdac_data")]
pub type GpdacData = crate::Reg<gpdac_data::GpdacDataSpec>;
#[doc = "gpdac_data."]
pub mod gpdac_data;
#[doc = "tzc_glb_ctrl_0 (r) register accessor: tzc_glb_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_0`] module"]
#[doc(alias = "tzc_glb_ctrl_0")]
pub type TzcGlbCtrl0 = crate::Reg<tzc_glb_ctrl_0::TzcGlbCtrl0Spec>;
#[doc = "tzc_glb_ctrl_0."]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_1 (r) register accessor: tzc_glb_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_1`] module"]
#[doc(alias = "tzc_glb_ctrl_1")]
pub type TzcGlbCtrl1 = crate::Reg<tzc_glb_ctrl_1::TzcGlbCtrl1Spec>;
#[doc = "tzc_glb_ctrl_1."]
pub mod tzc_glb_ctrl_1;
#[doc = "tzc_glb_ctrl_2 (r) register accessor: tzc_glb_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_2`] module"]
#[doc(alias = "tzc_glb_ctrl_2")]
pub type TzcGlbCtrl2 = crate::Reg<tzc_glb_ctrl_2::TzcGlbCtrl2Spec>;
#[doc = "tzc_glb_ctrl_2."]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_glb_ctrl_3 (rw) register accessor: Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_3`] module"]
#[doc(alias = "tzc_glb_ctrl_3")]
pub type TzcGlbCtrl3 = crate::Reg<tzc_glb_ctrl_3::TzcGlbCtrl3Spec>;
#[doc = "Reserved according to SDK."]
pub mod tzc_glb_ctrl_3;
