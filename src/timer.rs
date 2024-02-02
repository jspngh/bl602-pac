#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tccr: Tccr,
    _reserved1: [u8; 0x0c],
    tmr2_0: Tmr2_0,
    tmr2_1: Tmr2_1,
    tmr2_2: Tmr2_2,
    tmr3_0: Tmr3_0,
    tmr3_1: Tmr3_1,
    tmr3_2: Tmr3_2,
    _reserved7: [u8; 0x04],
    tcr2: Tcr2,
    tcr3: Tcr3,
    _reserved9: [u8; 0x04],
    tmsr2: Tmsr2,
    tmsr3: Tmsr3,
    _reserved11: [u8; 0x04],
    tier2: Tier2,
    tier3: Tier3,
    _reserved13: [u8; 0x04],
    tplvr2: Tplvr2,
    tplvr3: Tplvr3,
    _reserved15: [u8; 0x04],
    tplcr2: Tplcr2,
    tplcr3: Tplcr3,
    wmer: Wmer,
    wmr: Wmr,
    wvr: Wvr,
    wsr: Wsr,
    _reserved21: [u8; 0x04],
    ticr2: Ticr2,
    ticr3: Ticr3,
    wicr: Wicr,
    tcer: Tcer,
    tcmr: Tcmr,
    _reserved26: [u8; 0x04],
    tilr2: Tilr2,
    tilr3: Tilr3,
    wcr: Wcr,
    wfar: Wfar,
    wsar: Wsar,
    _reserved31: [u8; 0x04],
    tcvwr2: Tcvwr2,
    tcvwr3: Tcvwr3,
    _reserved33: [u8; 0x04],
    tcvsyn2: Tcvsyn2,
    tcvsyn3: Tcvsyn3,
    tcdr: Tcdr,
}
impl RegisterBlock {
    #[doc = "0x00 - TCCR."]
    #[inline(always)]
    pub const fn tccr(&self) -> &Tccr {
        &self.tccr
    }
    #[doc = "0x10 - TMR2_0."]
    #[inline(always)]
    pub const fn tmr2_0(&self) -> &Tmr2_0 {
        &self.tmr2_0
    }
    #[doc = "0x14 - TMR2_1."]
    #[inline(always)]
    pub const fn tmr2_1(&self) -> &Tmr2_1 {
        &self.tmr2_1
    }
    #[doc = "0x18 - TMR2_2."]
    #[inline(always)]
    pub const fn tmr2_2(&self) -> &Tmr2_2 {
        &self.tmr2_2
    }
    #[doc = "0x1c - TMR3_0."]
    #[inline(always)]
    pub const fn tmr3_0(&self) -> &Tmr3_0 {
        &self.tmr3_0
    }
    #[doc = "0x20 - TMR3_1."]
    #[inline(always)]
    pub const fn tmr3_1(&self) -> &Tmr3_1 {
        &self.tmr3_1
    }
    #[doc = "0x24 - TMR3_2."]
    #[inline(always)]
    pub const fn tmr3_2(&self) -> &Tmr3_2 {
        &self.tmr3_2
    }
    #[doc = "0x2c - TCR2."]
    #[inline(always)]
    pub const fn tcr2(&self) -> &Tcr2 {
        &self.tcr2
    }
    #[doc = "0x30 - TCR3."]
    #[inline(always)]
    pub const fn tcr3(&self) -> &Tcr3 {
        &self.tcr3
    }
    #[doc = "0x38 - TMSR2."]
    #[inline(always)]
    pub const fn tmsr2(&self) -> &Tmsr2 {
        &self.tmsr2
    }
    #[doc = "0x3c - TMSR3."]
    #[inline(always)]
    pub const fn tmsr3(&self) -> &Tmsr3 {
        &self.tmsr3
    }
    #[doc = "0x44 - TIER2."]
    #[inline(always)]
    pub const fn tier2(&self) -> &Tier2 {
        &self.tier2
    }
    #[doc = "0x48 - TIER3."]
    #[inline(always)]
    pub const fn tier3(&self) -> &Tier3 {
        &self.tier3
    }
    #[doc = "0x50 - TPLVR2."]
    #[inline(always)]
    pub const fn tplvr2(&self) -> &Tplvr2 {
        &self.tplvr2
    }
    #[doc = "0x54 - TPLVR3."]
    #[inline(always)]
    pub const fn tplvr3(&self) -> &Tplvr3 {
        &self.tplvr3
    }
    #[doc = "0x5c - TPLCR2."]
    #[inline(always)]
    pub const fn tplcr2(&self) -> &Tplcr2 {
        &self.tplcr2
    }
    #[doc = "0x60 - TPLCR3."]
    #[inline(always)]
    pub const fn tplcr3(&self) -> &Tplcr3 {
        &self.tplcr3
    }
    #[doc = "0x64 - WMER."]
    #[inline(always)]
    pub const fn wmer(&self) -> &Wmer {
        &self.wmer
    }
    #[doc = "0x68 - WMR."]
    #[inline(always)]
    pub const fn wmr(&self) -> &Wmr {
        &self.wmr
    }
    #[doc = "0x6c - WVR."]
    #[inline(always)]
    pub const fn wvr(&self) -> &Wvr {
        &self.wvr
    }
    #[doc = "0x70 - WSR."]
    #[inline(always)]
    pub const fn wsr(&self) -> &Wsr {
        &self.wsr
    }
    #[doc = "0x78 - TICR2."]
    #[inline(always)]
    pub const fn ticr2(&self) -> &Ticr2 {
        &self.ticr2
    }
    #[doc = "0x7c - TICR3."]
    #[inline(always)]
    pub const fn ticr3(&self) -> &Ticr3 {
        &self.ticr3
    }
    #[doc = "0x80 - WICR."]
    #[inline(always)]
    pub const fn wicr(&self) -> &Wicr {
        &self.wicr
    }
    #[doc = "0x84 - TCER."]
    #[inline(always)]
    pub const fn tcer(&self) -> &Tcer {
        &self.tcer
    }
    #[doc = "0x88 - TCMR."]
    #[inline(always)]
    pub const fn tcmr(&self) -> &Tcmr {
        &self.tcmr
    }
    #[doc = "0x90 - TILR2."]
    #[inline(always)]
    pub const fn tilr2(&self) -> &Tilr2 {
        &self.tilr2
    }
    #[doc = "0x94 - TILR3."]
    #[inline(always)]
    pub const fn tilr3(&self) -> &Tilr3 {
        &self.tilr3
    }
    #[doc = "0x98 - WCR."]
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    #[doc = "0x9c - WFAR."]
    #[inline(always)]
    pub const fn wfar(&self) -> &Wfar {
        &self.wfar
    }
    #[doc = "0xa0 - WSAR."]
    #[inline(always)]
    pub const fn wsar(&self) -> &Wsar {
        &self.wsar
    }
    #[doc = "0xa8 - TCVWR2."]
    #[inline(always)]
    pub const fn tcvwr2(&self) -> &Tcvwr2 {
        &self.tcvwr2
    }
    #[doc = "0xac - TCVWR3."]
    #[inline(always)]
    pub const fn tcvwr3(&self) -> &Tcvwr3 {
        &self.tcvwr3
    }
    #[doc = "0xb4 - TCVSYN2."]
    #[inline(always)]
    pub const fn tcvsyn2(&self) -> &Tcvsyn2 {
        &self.tcvsyn2
    }
    #[doc = "0xb8 - TCVSYN3."]
    #[inline(always)]
    pub const fn tcvsyn3(&self) -> &Tcvsyn3 {
        &self.tcvsyn3
    }
    #[doc = "0xbc - TCDR."]
    #[inline(always)]
    pub const fn tcdr(&self) -> &Tcdr {
        &self.tcdr
    }
}
#[doc = "TCCR (rw) register accessor: TCCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr`] module"]
#[doc(alias = "TCCR")]
pub type Tccr = crate::Reg<tccr::TccrSpec>;
#[doc = "TCCR."]
pub mod tccr;
#[doc = "TMR2_0 (rw) register accessor: TMR2_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_0`] module"]
#[doc(alias = "TMR2_0")]
pub type Tmr2_0 = crate::Reg<tmr2_0::Tmr2_0Spec>;
#[doc = "TMR2_0."]
pub mod tmr2_0;
#[doc = "TMR2_1 (rw) register accessor: TMR2_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_1`] module"]
#[doc(alias = "TMR2_1")]
pub type Tmr2_1 = crate::Reg<tmr2_1::Tmr2_1Spec>;
#[doc = "TMR2_1."]
pub mod tmr2_1;
#[doc = "TMR2_2 (rw) register accessor: TMR2_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_2`] module"]
#[doc(alias = "TMR2_2")]
pub type Tmr2_2 = crate::Reg<tmr2_2::Tmr2_2Spec>;
#[doc = "TMR2_2."]
pub mod tmr2_2;
#[doc = "TMR3_0 (rw) register accessor: TMR3_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3_0`] module"]
#[doc(alias = "TMR3_0")]
pub type Tmr3_0 = crate::Reg<tmr3_0::Tmr3_0Spec>;
#[doc = "TMR3_0."]
pub mod tmr3_0;
#[doc = "TMR3_1 (rw) register accessor: TMR3_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3_1`] module"]
#[doc(alias = "TMR3_1")]
pub type Tmr3_1 = crate::Reg<tmr3_1::Tmr3_1Spec>;
#[doc = "TMR3_1."]
pub mod tmr3_1;
#[doc = "TMR3_2 (rw) register accessor: TMR3_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3_2`] module"]
#[doc(alias = "TMR3_2")]
pub type Tmr3_2 = crate::Reg<tmr3_2::Tmr3_2Spec>;
#[doc = "TMR3_2."]
pub mod tmr3_2;
#[doc = "TCR2 (r) register accessor: TCR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr2`] module"]
#[doc(alias = "TCR2")]
pub type Tcr2 = crate::Reg<tcr2::Tcr2Spec>;
#[doc = "TCR2."]
pub mod tcr2;
#[doc = "TCR3 (r) register accessor: TCR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr3`] module"]
#[doc(alias = "TCR3")]
pub type Tcr3 = crate::Reg<tcr3::Tcr3Spec>;
#[doc = "TCR3."]
pub mod tcr3;
#[doc = "TMSR2 (r) register accessor: TMSR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmsr2`] module"]
#[doc(alias = "TMSR2")]
pub type Tmsr2 = crate::Reg<tmsr2::Tmsr2Spec>;
#[doc = "TMSR2."]
pub mod tmsr2;
#[doc = "TMSR3 (r) register accessor: TMSR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmsr3`] module"]
#[doc(alias = "TMSR3")]
pub type Tmsr3 = crate::Reg<tmsr3::Tmsr3Spec>;
#[doc = "TMSR3."]
pub mod tmsr3;
#[doc = "TIER2 (rw) register accessor: TIER2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tier2`] module"]
#[doc(alias = "TIER2")]
pub type Tier2 = crate::Reg<tier2::Tier2Spec>;
#[doc = "TIER2."]
pub mod tier2;
#[doc = "TIER3 (rw) register accessor: TIER3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tier3`] module"]
#[doc(alias = "TIER3")]
pub type Tier3 = crate::Reg<tier3::Tier3Spec>;
#[doc = "TIER3."]
pub mod tier3;
#[doc = "TPLVR2 (rw) register accessor: TPLVR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplvr2`] module"]
#[doc(alias = "TPLVR2")]
pub type Tplvr2 = crate::Reg<tplvr2::Tplvr2Spec>;
#[doc = "TPLVR2."]
pub mod tplvr2;
#[doc = "TPLVR3 (rw) register accessor: TPLVR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplvr3`] module"]
#[doc(alias = "TPLVR3")]
pub type Tplvr3 = crate::Reg<tplvr3::Tplvr3Spec>;
#[doc = "TPLVR3."]
pub mod tplvr3;
#[doc = "TPLCR2 (rw) register accessor: TPLCR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplcr2`] module"]
#[doc(alias = "TPLCR2")]
pub type Tplcr2 = crate::Reg<tplcr2::Tplcr2Spec>;
#[doc = "TPLCR2."]
pub mod tplcr2;
#[doc = "TPLCR3 (rw) register accessor: TPLCR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplcr3`] module"]
#[doc(alias = "TPLCR3")]
pub type Tplcr3 = crate::Reg<tplcr3::Tplcr3Spec>;
#[doc = "TPLCR3."]
pub mod tplcr3;
#[doc = "WMER (rw) register accessor: WMER.\n\nYou can [`read`](crate::Reg::read) this register and get [`wmer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wmer`] module"]
#[doc(alias = "WMER")]
pub type Wmer = crate::Reg<wmer::WmerSpec>;
#[doc = "WMER."]
pub mod wmer;
#[doc = "WMR (rw) register accessor: WMR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wmr`] module"]
#[doc(alias = "WMR")]
pub type Wmr = crate::Reg<wmr::WmrSpec>;
#[doc = "WMR."]
pub mod wmr;
#[doc = "WVR (r) register accessor: WVR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wvr`] module"]
#[doc(alias = "WVR")]
pub type Wvr = crate::Reg<wvr::WvrSpec>;
#[doc = "WVR."]
pub mod wvr;
#[doc = "WSR (rw) register accessor: WSR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsr`] module"]
#[doc(alias = "WSR")]
pub type Wsr = crate::Reg<wsr::WsrSpec>;
#[doc = "WSR."]
pub mod wsr;
#[doc = "TICR2 (w) register accessor: TICR2.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticr2`] module"]
#[doc(alias = "TICR2")]
pub type Ticr2 = crate::Reg<ticr2::Ticr2Spec>;
#[doc = "TICR2."]
pub mod ticr2;
#[doc = "TICR3 (w) register accessor: TICR3.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticr3`] module"]
#[doc(alias = "TICR3")]
pub type Ticr3 = crate::Reg<ticr3::Ticr3Spec>;
#[doc = "TICR3."]
pub mod ticr3;
#[doc = "WICR (w) register accessor: WICR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wicr`] module"]
#[doc(alias = "WICR")]
pub type Wicr = crate::Reg<wicr::WicrSpec>;
#[doc = "WICR."]
pub mod wicr;
#[doc = "TCER (rw) register accessor: TCER.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcer`] module"]
#[doc(alias = "TCER")]
pub type Tcer = crate::Reg<tcer::TcerSpec>;
#[doc = "TCER."]
pub mod tcer;
#[doc = "TCMR (rw) register accessor: TCMR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr`] module"]
#[doc(alias = "TCMR")]
pub type Tcmr = crate::Reg<tcmr::TcmrSpec>;
#[doc = "TCMR."]
pub mod tcmr;
#[doc = "TILR2 (rw) register accessor: TILR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tilr2`] module"]
#[doc(alias = "TILR2")]
pub type Tilr2 = crate::Reg<tilr2::Tilr2Spec>;
#[doc = "TILR2."]
pub mod tilr2;
#[doc = "TILR3 (rw) register accessor: TILR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tilr3`] module"]
#[doc(alias = "TILR3")]
pub type Tilr3 = crate::Reg<tilr3::Tilr3Spec>;
#[doc = "TILR3."]
pub mod tilr3;
#[doc = "WCR (w) register accessor: WCR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`] module"]
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WcrSpec>;
#[doc = "WCR."]
pub mod wcr;
#[doc = "WFAR (w) register accessor: WFAR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfar`] module"]
#[doc(alias = "WFAR")]
pub type Wfar = crate::Reg<wfar::WfarSpec>;
#[doc = "WFAR."]
pub mod wfar;
#[doc = "WSAR (w) register accessor: WSAR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsar`] module"]
#[doc(alias = "WSAR")]
pub type Wsar = crate::Reg<wsar::WsarSpec>;
#[doc = "WSAR."]
pub mod wsar;
#[doc = "TCVWR2 (r) register accessor: TCVWR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvwr2`] module"]
#[doc(alias = "TCVWR2")]
pub type Tcvwr2 = crate::Reg<tcvwr2::Tcvwr2Spec>;
#[doc = "TCVWR2."]
pub mod tcvwr2;
#[doc = "TCVWR3 (r) register accessor: TCVWR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvwr3`] module"]
#[doc(alias = "TCVWR3")]
pub type Tcvwr3 = crate::Reg<tcvwr3::Tcvwr3Spec>;
#[doc = "TCVWR3."]
pub mod tcvwr3;
#[doc = "TCVSYN2 (r) register accessor: TCVSYN2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvsyn2`] module"]
#[doc(alias = "TCVSYN2")]
pub type Tcvsyn2 = crate::Reg<tcvsyn2::Tcvsyn2Spec>;
#[doc = "TCVSYN2."]
pub mod tcvsyn2;
#[doc = "TCVSYN3 (r) register accessor: TCVSYN3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvsyn3`] module"]
#[doc(alias = "TCVSYN3")]
pub type Tcvsyn3 = crate::Reg<tcvsyn3::Tcvsyn3Spec>;
#[doc = "TCVSYN3."]
pub mod tcvsyn3;
#[doc = "TCDR (rw) register accessor: TCDR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcdr`] module"]
#[doc(alias = "TCDR")]
pub type Tcdr = crate::Reg<tcdr::TcdrSpec>;
#[doc = "TCDR."]
pub mod tcdr;
