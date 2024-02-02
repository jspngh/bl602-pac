#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    tzc_rom_ctrl: TzcRomCtrl,
    tzc_rom0_r0: TzcRom0R0,
    tzc_rom0_r1: TzcRom0R1,
    tzc_rom1_r0: TzcRom1R0,
    tzc_rom1_r1: TzcRom1R1,
}
impl RegisterBlock {
    #[doc = "0x40 - tzc_rom_ctrl."]
    #[inline(always)]
    pub const fn tzc_rom_ctrl(&self) -> &TzcRomCtrl {
        &self.tzc_rom_ctrl
    }
    #[doc = "0x44 - tzc_rom0_r0."]
    #[inline(always)]
    pub const fn tzc_rom0_r0(&self) -> &TzcRom0R0 {
        &self.tzc_rom0_r0
    }
    #[doc = "0x48 - tzc_rom0_r1."]
    #[inline(always)]
    pub const fn tzc_rom0_r1(&self) -> &TzcRom0R1 {
        &self.tzc_rom0_r1
    }
    #[doc = "0x4c - tzc_rom1_r0."]
    #[inline(always)]
    pub const fn tzc_rom1_r0(&self) -> &TzcRom1R0 {
        &self.tzc_rom1_r0
    }
    #[doc = "0x50 - tzc_rom1_r1."]
    #[inline(always)]
    pub const fn tzc_rom1_r1(&self) -> &TzcRom1R1 {
        &self.tzc_rom1_r1
    }
}
#[doc = "tzc_rom_ctrl (rw) register accessor: tzc_rom_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_ctrl`] module"]
#[doc(alias = "tzc_rom_ctrl")]
pub type TzcRomCtrl = crate::Reg<tzc_rom_ctrl::TzcRomCtrlSpec>;
#[doc = "tzc_rom_ctrl."]
pub mod tzc_rom_ctrl;
#[doc = "tzc_rom0_r0 (rw) register accessor: tzc_rom0_r0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom0_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom0_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom0_r0`] module"]
#[doc(alias = "tzc_rom0_r0")]
pub type TzcRom0R0 = crate::Reg<tzc_rom0_r0::TzcRom0R0Spec>;
#[doc = "tzc_rom0_r0."]
pub mod tzc_rom0_r0;
#[doc = "tzc_rom0_r1 (rw) register accessor: tzc_rom0_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom0_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom0_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom0_r1`] module"]
#[doc(alias = "tzc_rom0_r1")]
pub type TzcRom0R1 = crate::Reg<tzc_rom0_r1::TzcRom0R1Spec>;
#[doc = "tzc_rom0_r1."]
pub mod tzc_rom0_r1;
#[doc = "tzc_rom1_r0 (rw) register accessor: tzc_rom1_r0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom1_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom1_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom1_r0`] module"]
#[doc(alias = "tzc_rom1_r0")]
pub type TzcRom1R0 = crate::Reg<tzc_rom1_r0::TzcRom1R0Spec>;
#[doc = "tzc_rom1_r0."]
pub mod tzc_rom1_r0;
#[doc = "tzc_rom1_r1 (rw) register accessor: tzc_rom1_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom1_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom1_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom1_r1`] module"]
#[doc(alias = "tzc_rom1_r1")]
pub type TzcRom1R1 = crate::Reg<tzc_rom1_r1::TzcRom1R1Spec>;
#[doc = "tzc_rom1_r1."]
pub mod tzc_rom1_r1;
