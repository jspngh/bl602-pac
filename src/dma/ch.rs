#[repr(C)]
#[doc = "Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
#[doc(alias = "CH")]
pub struct Ch {
    ch_src_addr: ChSrcAddr,
    ch_dst_addr: ChDstAddr,
    ch_lli: ChLli,
    ch_control: ChControl,
    ch_config: ChConfig,
}
impl Ch {
    #[doc = "0x00 - DMA channel source address"]
    #[inline(always)]
    pub const fn ch_src_addr(&self) -> &ChSrcAddr {
        &self.ch_src_addr
    }
    #[doc = "0x04 - DMA channel destination address"]
    #[inline(always)]
    pub const fn ch_dst_addr(&self) -> &ChDstAddr {
        &self.ch_dst_addr
    }
    #[doc = "0x08 - DMA channel linked list item"]
    #[inline(always)]
    pub const fn ch_lli(&self) -> &ChLli {
        &self.ch_lli
    }
    #[doc = "0x0c - DMA channel control"]
    #[inline(always)]
    pub const fn ch_control(&self) -> &ChControl {
        &self.ch_control
    }
    #[doc = "0x10 - DMA channel config"]
    #[inline(always)]
    pub const fn ch_config(&self) -> &ChConfig {
        &self.ch_config
    }
}
#[doc = "CH_SRC_ADDR (rw) register accessor: DMA channel source address\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_src_addr`] module"]
#[doc(alias = "CH_SRC_ADDR")]
pub type ChSrcAddr = crate::Reg<ch_src_addr::ChSrcAddrSpec>;
#[doc = "DMA channel source address"]
pub mod ch_src_addr;
#[doc = "CH_DST_ADDR (rw) register accessor: DMA channel destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_dst_addr`] module"]
#[doc(alias = "CH_DST_ADDR")]
pub type ChDstAddr = crate::Reg<ch_dst_addr::ChDstAddrSpec>;
#[doc = "DMA channel destination address"]
pub mod ch_dst_addr;
#[doc = "CH_LLI (rw) register accessor: DMA channel linked list item\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_lli`] module"]
#[doc(alias = "CH_LLI")]
pub type ChLli = crate::Reg<ch_lli::ChLliSpec>;
#[doc = "DMA channel linked list item"]
pub mod ch_lli;
#[doc = "CH_CONTROL (rw) register accessor: DMA channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_control`] module"]
#[doc(alias = "CH_CONTROL")]
pub type ChControl = crate::Reg<ch_control::ChControlSpec>;
#[doc = "DMA channel control"]
pub mod ch_control;
#[doc = "CH_CONFIG (rw) register accessor: DMA channel config\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_config`] module"]
#[doc(alias = "CH_CONFIG")]
pub type ChConfig = crate::Reg<ch_config::ChConfigSpec>;
#[doc = "DMA channel config"]
pub mod ch_config;
