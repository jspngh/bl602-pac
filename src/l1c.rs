#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    l1c_config: L1cConfig,
    hit_cnt_lsb: HitCntLsb,
    hit_cnt_msb: HitCntMsb,
    miss_cnt: MissCnt,
    l1c_range: L1cRange,
    _reserved5: [u8; 0x01ec],
    l1c_bmx_err_addr_en: L1cBmxErrAddrEn,
    l1c_bmx_err_addr: L1cBmxErrAddr,
    irom1_misr_dataout_0: Irom1MisrDataout0,
    irom1_misr_dataout_1: Irom1MisrDataout1,
    cpu_clk_gate: CpuClkGate,
}
impl RegisterBlock {
    #[doc = "0x00 - l1c_config."]
    #[inline(always)]
    pub const fn l1c_config(&self) -> &L1cConfig {
        &self.l1c_config
    }
    #[doc = "0x04 - hit_cnt_lsb."]
    #[inline(always)]
    pub const fn hit_cnt_lsb(&self) -> &HitCntLsb {
        &self.hit_cnt_lsb
    }
    #[doc = "0x08 - hit_cnt_msb."]
    #[inline(always)]
    pub const fn hit_cnt_msb(&self) -> &HitCntMsb {
        &self.hit_cnt_msb
    }
    #[doc = "0x0c - miss_cnt."]
    #[inline(always)]
    pub const fn miss_cnt(&self) -> &MissCnt {
        &self.miss_cnt
    }
    #[doc = "0x10 - l1c_range."]
    #[inline(always)]
    pub const fn l1c_range(&self) -> &L1cRange {
        &self.l1c_range
    }
    #[doc = "0x200 - l1c_bmx_err_addr_en."]
    #[inline(always)]
    pub const fn l1c_bmx_err_addr_en(&self) -> &L1cBmxErrAddrEn {
        &self.l1c_bmx_err_addr_en
    }
    #[doc = "0x204 - l1c_bmx_err_addr."]
    #[inline(always)]
    pub const fn l1c_bmx_err_addr(&self) -> &L1cBmxErrAddr {
        &self.l1c_bmx_err_addr
    }
    #[doc = "0x208 - irom1_misr_dataout_0."]
    #[inline(always)]
    pub const fn irom1_misr_dataout_0(&self) -> &Irom1MisrDataout0 {
        &self.irom1_misr_dataout_0
    }
    #[doc = "0x20c - irom1_misr_dataout_1."]
    #[inline(always)]
    pub const fn irom1_misr_dataout_1(&self) -> &Irom1MisrDataout1 {
        &self.irom1_misr_dataout_1
    }
    #[doc = "0x210 - cpu_clk_gate."]
    #[inline(always)]
    pub const fn cpu_clk_gate(&self) -> &CpuClkGate {
        &self.cpu_clk_gate
    }
}
#[doc = "l1c_config (rw) register accessor: l1c_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1c_config`] module"]
#[doc(alias = "l1c_config")]
pub type L1cConfig = crate::Reg<l1c_config::L1cConfigSpec>;
#[doc = "l1c_config."]
pub mod l1c_config;
#[doc = "hit_cnt_lsb (r) register accessor: hit_cnt_lsb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_lsb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hit_cnt_lsb`] module"]
#[doc(alias = "hit_cnt_lsb")]
pub type HitCntLsb = crate::Reg<hit_cnt_lsb::HitCntLsbSpec>;
#[doc = "hit_cnt_lsb."]
pub mod hit_cnt_lsb;
#[doc = "hit_cnt_msb (r) register accessor: hit_cnt_msb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_msb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hit_cnt_msb`] module"]
#[doc(alias = "hit_cnt_msb")]
pub type HitCntMsb = crate::Reg<hit_cnt_msb::HitCntMsbSpec>;
#[doc = "hit_cnt_msb."]
pub mod hit_cnt_msb;
#[doc = "miss_cnt (r) register accessor: miss_cnt.\n\nYou can [`read`](crate::Reg::read) this register and get [`miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miss_cnt`] module"]
#[doc(alias = "miss_cnt")]
pub type MissCnt = crate::Reg<miss_cnt::MissCntSpec>;
#[doc = "miss_cnt."]
pub mod miss_cnt;
#[doc = "l1c_range (rw) register accessor: l1c_range.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_range::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_range::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1c_range`] module"]
#[doc(alias = "l1c_range")]
pub type L1cRange = crate::Reg<l1c_range::L1cRangeSpec>;
#[doc = "l1c_range."]
pub mod l1c_range;
#[doc = "l1c_bmx_err_addr_en (rw) register accessor: l1c_bmx_err_addr_en.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_bmx_err_addr_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_bmx_err_addr_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1c_bmx_err_addr_en`] module"]
#[doc(alias = "l1c_bmx_err_addr_en")]
pub type L1cBmxErrAddrEn = crate::Reg<l1c_bmx_err_addr_en::L1cBmxErrAddrEnSpec>;
#[doc = "l1c_bmx_err_addr_en."]
pub mod l1c_bmx_err_addr_en;
#[doc = "l1c_bmx_err_addr (r) register accessor: l1c_bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_bmx_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1c_bmx_err_addr`] module"]
#[doc(alias = "l1c_bmx_err_addr")]
pub type L1cBmxErrAddr = crate::Reg<l1c_bmx_err_addr::L1cBmxErrAddrSpec>;
#[doc = "l1c_bmx_err_addr."]
pub mod l1c_bmx_err_addr;
#[doc = "irom1_misr_dataout_0 (r) register accessor: irom1_misr_dataout_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_0`] module"]
#[doc(alias = "irom1_misr_dataout_0")]
pub type Irom1MisrDataout0 = crate::Reg<irom1_misr_dataout_0::Irom1MisrDataout0Spec>;
#[doc = "irom1_misr_dataout_0."]
pub mod irom1_misr_dataout_0;
#[doc = "irom1_misr_dataout_1 (rw) register accessor: irom1_misr_dataout_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irom1_misr_dataout_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_1`] module"]
#[doc(alias = "irom1_misr_dataout_1")]
pub type Irom1MisrDataout1 = crate::Reg<irom1_misr_dataout_1::Irom1MisrDataout1Spec>;
#[doc = "irom1_misr_dataout_1."]
pub mod irom1_misr_dataout_1;
#[doc = "cpu_clk_gate (rw) register accessor: cpu_clk_gate.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_clk_gate`] module"]
#[doc(alias = "cpu_clk_gate")]
pub type CpuClkGate = crate::Reg<cpu_clk_gate::CpuClkGateSpec>;
#[doc = "cpu_clk_gate."]
pub mod cpu_clk_gate;
