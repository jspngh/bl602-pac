#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_key_slot_6_w0: RegKeySlot6W0,
    reg_key_slot_6_w1: RegKeySlot6W1,
    reg_key_slot_6_w2: RegKeySlot6W2,
    reg_key_slot_6_w3: RegKeySlot6W3,
    reg_key_slot_7_w0: RegKeySlot7W0,
    reg_key_slot_7_w1: RegKeySlot7W1,
    reg_key_slot_7_w2: RegKeySlot7W2,
    reg_key_slot_7_w3: RegKeySlot7W3,
    reg_key_slot_8_w0: RegKeySlot8W0,
    reg_key_slot_8_w1: RegKeySlot8W1,
    reg_key_slot_8_w2: RegKeySlot8W2,
    reg_key_slot_8_w3: RegKeySlot8W3,
    reg_key_slot_9_w0: RegKeySlot9W0,
    reg_key_slot_9_w1: RegKeySlot9W1,
    reg_key_slot_9_w2: RegKeySlot9W2,
    reg_key_slot_9_w3: RegKeySlot9W3,
    reg_key_slot_10_w0: RegKeySlot10W0,
    reg_key_slot_10_w1: RegKeySlot10W1,
    reg_key_slot_10_w2: RegKeySlot10W2,
    reg_key_slot_10_w3: RegKeySlot10W3,
    reg_key_slot_11_w0: RegKeySlot11W0,
    reg_key_slot_11_w1: RegKeySlot11W1,
    reg_key_slot_11_w2: RegKeySlot11W2,
    reg_key_slot_11_w3: RegKeySlot11W3,
    reg_data_1_lock: RegData1Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - reg_key_slot_6_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_6_w0(&self) -> &RegKeySlot6W0 {
        &self.reg_key_slot_6_w0
    }
    #[doc = "0x04 - reg_key_slot_6_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_6_w1(&self) -> &RegKeySlot6W1 {
        &self.reg_key_slot_6_w1
    }
    #[doc = "0x08 - reg_key_slot_6_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_6_w2(&self) -> &RegKeySlot6W2 {
        &self.reg_key_slot_6_w2
    }
    #[doc = "0x0c - reg_key_slot_6_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_6_w3(&self) -> &RegKeySlot6W3 {
        &self.reg_key_slot_6_w3
    }
    #[doc = "0x10 - reg_key_slot_7_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_7_w0(&self) -> &RegKeySlot7W0 {
        &self.reg_key_slot_7_w0
    }
    #[doc = "0x14 - reg_key_slot_7_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_7_w1(&self) -> &RegKeySlot7W1 {
        &self.reg_key_slot_7_w1
    }
    #[doc = "0x18 - reg_key_slot_7_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_7_w2(&self) -> &RegKeySlot7W2 {
        &self.reg_key_slot_7_w2
    }
    #[doc = "0x1c - reg_key_slot_7_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_7_w3(&self) -> &RegKeySlot7W3 {
        &self.reg_key_slot_7_w3
    }
    #[doc = "0x20 - reg_key_slot_8_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_8_w0(&self) -> &RegKeySlot8W0 {
        &self.reg_key_slot_8_w0
    }
    #[doc = "0x24 - reg_key_slot_8_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_8_w1(&self) -> &RegKeySlot8W1 {
        &self.reg_key_slot_8_w1
    }
    #[doc = "0x28 - reg_key_slot_8_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_8_w2(&self) -> &RegKeySlot8W2 {
        &self.reg_key_slot_8_w2
    }
    #[doc = "0x2c - reg_key_slot_8_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_8_w3(&self) -> &RegKeySlot8W3 {
        &self.reg_key_slot_8_w3
    }
    #[doc = "0x30 - reg_key_slot_9_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_9_w0(&self) -> &RegKeySlot9W0 {
        &self.reg_key_slot_9_w0
    }
    #[doc = "0x34 - reg_key_slot_9_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_9_w1(&self) -> &RegKeySlot9W1 {
        &self.reg_key_slot_9_w1
    }
    #[doc = "0x38 - reg_key_slot_9_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_9_w2(&self) -> &RegKeySlot9W2 {
        &self.reg_key_slot_9_w2
    }
    #[doc = "0x3c - reg_key_slot_9_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_9_w3(&self) -> &RegKeySlot9W3 {
        &self.reg_key_slot_9_w3
    }
    #[doc = "0x40 - reg_key_slot_10_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_10_w0(&self) -> &RegKeySlot10W0 {
        &self.reg_key_slot_10_w0
    }
    #[doc = "0x44 - reg_key_slot_10_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_10_w1(&self) -> &RegKeySlot10W1 {
        &self.reg_key_slot_10_w1
    }
    #[doc = "0x48 - reg_key_slot_10_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_10_w2(&self) -> &RegKeySlot10W2 {
        &self.reg_key_slot_10_w2
    }
    #[doc = "0x4c - reg_key_slot_10_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_10_w3(&self) -> &RegKeySlot10W3 {
        &self.reg_key_slot_10_w3
    }
    #[doc = "0x50 - reg_key_slot_11_w0."]
    #[inline(always)]
    pub const fn reg_key_slot_11_w0(&self) -> &RegKeySlot11W0 {
        &self.reg_key_slot_11_w0
    }
    #[doc = "0x54 - reg_key_slot_11_w1."]
    #[inline(always)]
    pub const fn reg_key_slot_11_w1(&self) -> &RegKeySlot11W1 {
        &self.reg_key_slot_11_w1
    }
    #[doc = "0x58 - reg_key_slot_11_w2."]
    #[inline(always)]
    pub const fn reg_key_slot_11_w2(&self) -> &RegKeySlot11W2 {
        &self.reg_key_slot_11_w2
    }
    #[doc = "0x5c - reg_key_slot_11_w3."]
    #[inline(always)]
    pub const fn reg_key_slot_11_w3(&self) -> &RegKeySlot11W3 {
        &self.reg_key_slot_11_w3
    }
    #[doc = "0x60 - reg_data_1_lock."]
    #[inline(always)]
    pub const fn reg_data_1_lock(&self) -> &RegData1Lock {
        &self.reg_data_1_lock
    }
}
#[doc = "reg_key_slot_6_w0 (rw) register accessor: reg_key_slot_6_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_6_w0`] module"]
#[doc(alias = "reg_key_slot_6_w0")]
pub type RegKeySlot6W0 = crate::Reg<reg_key_slot_6_w0::RegKeySlot6W0Spec>;
#[doc = "reg_key_slot_6_w0."]
pub mod reg_key_slot_6_w0;
#[doc = "reg_key_slot_6_w1 (rw) register accessor: reg_key_slot_6_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_6_w1`] module"]
#[doc(alias = "reg_key_slot_6_w1")]
pub type RegKeySlot6W1 = crate::Reg<reg_key_slot_6_w1::RegKeySlot6W1Spec>;
#[doc = "reg_key_slot_6_w1."]
pub mod reg_key_slot_6_w1;
#[doc = "reg_key_slot_6_w2 (rw) register accessor: reg_key_slot_6_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_6_w2`] module"]
#[doc(alias = "reg_key_slot_6_w2")]
pub type RegKeySlot6W2 = crate::Reg<reg_key_slot_6_w2::RegKeySlot6W2Spec>;
#[doc = "reg_key_slot_6_w2."]
pub mod reg_key_slot_6_w2;
#[doc = "reg_key_slot_6_w3 (rw) register accessor: reg_key_slot_6_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_6_w3`] module"]
#[doc(alias = "reg_key_slot_6_w3")]
pub type RegKeySlot6W3 = crate::Reg<reg_key_slot_6_w3::RegKeySlot6W3Spec>;
#[doc = "reg_key_slot_6_w3."]
pub mod reg_key_slot_6_w3;
#[doc = "reg_key_slot_7_w0 (rw) register accessor: reg_key_slot_7_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_7_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_7_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_7_w0`] module"]
#[doc(alias = "reg_key_slot_7_w0")]
pub type RegKeySlot7W0 = crate::Reg<reg_key_slot_7_w0::RegKeySlot7W0Spec>;
#[doc = "reg_key_slot_7_w0."]
pub mod reg_key_slot_7_w0;
#[doc = "reg_key_slot_7_w1 (rw) register accessor: reg_key_slot_7_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_7_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_7_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_7_w1`] module"]
#[doc(alias = "reg_key_slot_7_w1")]
pub type RegKeySlot7W1 = crate::Reg<reg_key_slot_7_w1::RegKeySlot7W1Spec>;
#[doc = "reg_key_slot_7_w1."]
pub mod reg_key_slot_7_w1;
#[doc = "reg_key_slot_7_w2 (rw) register accessor: reg_key_slot_7_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_7_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_7_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_7_w2`] module"]
#[doc(alias = "reg_key_slot_7_w2")]
pub type RegKeySlot7W2 = crate::Reg<reg_key_slot_7_w2::RegKeySlot7W2Spec>;
#[doc = "reg_key_slot_7_w2."]
pub mod reg_key_slot_7_w2;
#[doc = "reg_key_slot_7_w3 (rw) register accessor: reg_key_slot_7_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_7_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_7_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_7_w3`] module"]
#[doc(alias = "reg_key_slot_7_w3")]
pub type RegKeySlot7W3 = crate::Reg<reg_key_slot_7_w3::RegKeySlot7W3Spec>;
#[doc = "reg_key_slot_7_w3."]
pub mod reg_key_slot_7_w3;
#[doc = "reg_key_slot_8_w0 (rw) register accessor: reg_key_slot_8_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_8_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_8_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_8_w0`] module"]
#[doc(alias = "reg_key_slot_8_w0")]
pub type RegKeySlot8W0 = crate::Reg<reg_key_slot_8_w0::RegKeySlot8W0Spec>;
#[doc = "reg_key_slot_8_w0."]
pub mod reg_key_slot_8_w0;
#[doc = "reg_key_slot_8_w1 (rw) register accessor: reg_key_slot_8_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_8_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_8_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_8_w1`] module"]
#[doc(alias = "reg_key_slot_8_w1")]
pub type RegKeySlot8W1 = crate::Reg<reg_key_slot_8_w1::RegKeySlot8W1Spec>;
#[doc = "reg_key_slot_8_w1."]
pub mod reg_key_slot_8_w1;
#[doc = "reg_key_slot_8_w2 (rw) register accessor: reg_key_slot_8_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_8_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_8_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_8_w2`] module"]
#[doc(alias = "reg_key_slot_8_w2")]
pub type RegKeySlot8W2 = crate::Reg<reg_key_slot_8_w2::RegKeySlot8W2Spec>;
#[doc = "reg_key_slot_8_w2."]
pub mod reg_key_slot_8_w2;
#[doc = "reg_key_slot_8_w3 (rw) register accessor: reg_key_slot_8_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_8_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_8_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_8_w3`] module"]
#[doc(alias = "reg_key_slot_8_w3")]
pub type RegKeySlot8W3 = crate::Reg<reg_key_slot_8_w3::RegKeySlot8W3Spec>;
#[doc = "reg_key_slot_8_w3."]
pub mod reg_key_slot_8_w3;
#[doc = "reg_key_slot_9_w0 (rw) register accessor: reg_key_slot_9_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_9_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_9_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_9_w0`] module"]
#[doc(alias = "reg_key_slot_9_w0")]
pub type RegKeySlot9W0 = crate::Reg<reg_key_slot_9_w0::RegKeySlot9W0Spec>;
#[doc = "reg_key_slot_9_w0."]
pub mod reg_key_slot_9_w0;
#[doc = "reg_key_slot_9_w1 (rw) register accessor: reg_key_slot_9_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_9_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_9_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_9_w1`] module"]
#[doc(alias = "reg_key_slot_9_w1")]
pub type RegKeySlot9W1 = crate::Reg<reg_key_slot_9_w1::RegKeySlot9W1Spec>;
#[doc = "reg_key_slot_9_w1."]
pub mod reg_key_slot_9_w1;
#[doc = "reg_key_slot_9_w2 (rw) register accessor: reg_key_slot_9_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_9_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_9_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_9_w2`] module"]
#[doc(alias = "reg_key_slot_9_w2")]
pub type RegKeySlot9W2 = crate::Reg<reg_key_slot_9_w2::RegKeySlot9W2Spec>;
#[doc = "reg_key_slot_9_w2."]
pub mod reg_key_slot_9_w2;
#[doc = "reg_key_slot_9_w3 (rw) register accessor: reg_key_slot_9_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_9_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_9_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_9_w3`] module"]
#[doc(alias = "reg_key_slot_9_w3")]
pub type RegKeySlot9W3 = crate::Reg<reg_key_slot_9_w3::RegKeySlot9W3Spec>;
#[doc = "reg_key_slot_9_w3."]
pub mod reg_key_slot_9_w3;
#[doc = "reg_key_slot_10_w0 (rw) register accessor: reg_key_slot_10_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_10_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_10_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_10_w0`] module"]
#[doc(alias = "reg_key_slot_10_w0")]
pub type RegKeySlot10W0 = crate::Reg<reg_key_slot_10_w0::RegKeySlot10W0Spec>;
#[doc = "reg_key_slot_10_w0."]
pub mod reg_key_slot_10_w0;
#[doc = "reg_key_slot_10_w1 (rw) register accessor: reg_key_slot_10_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_10_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_10_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_10_w1`] module"]
#[doc(alias = "reg_key_slot_10_w1")]
pub type RegKeySlot10W1 = crate::Reg<reg_key_slot_10_w1::RegKeySlot10W1Spec>;
#[doc = "reg_key_slot_10_w1."]
pub mod reg_key_slot_10_w1;
#[doc = "reg_key_slot_10_w2 (rw) register accessor: reg_key_slot_10_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_10_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_10_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_10_w2`] module"]
#[doc(alias = "reg_key_slot_10_w2")]
pub type RegKeySlot10W2 = crate::Reg<reg_key_slot_10_w2::RegKeySlot10W2Spec>;
#[doc = "reg_key_slot_10_w2."]
pub mod reg_key_slot_10_w2;
#[doc = "reg_key_slot_10_w3 (rw) register accessor: reg_key_slot_10_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_10_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_10_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_10_w3`] module"]
#[doc(alias = "reg_key_slot_10_w3")]
pub type RegKeySlot10W3 = crate::Reg<reg_key_slot_10_w3::RegKeySlot10W3Spec>;
#[doc = "reg_key_slot_10_w3."]
pub mod reg_key_slot_10_w3;
#[doc = "reg_key_slot_11_w0 (rw) register accessor: reg_key_slot_11_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_11_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_11_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_11_w0`] module"]
#[doc(alias = "reg_key_slot_11_w0")]
pub type RegKeySlot11W0 = crate::Reg<reg_key_slot_11_w0::RegKeySlot11W0Spec>;
#[doc = "reg_key_slot_11_w0."]
pub mod reg_key_slot_11_w0;
#[doc = "reg_key_slot_11_w1 (rw) register accessor: reg_key_slot_11_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_11_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_11_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_11_w1`] module"]
#[doc(alias = "reg_key_slot_11_w1")]
pub type RegKeySlot11W1 = crate::Reg<reg_key_slot_11_w1::RegKeySlot11W1Spec>;
#[doc = "reg_key_slot_11_w1."]
pub mod reg_key_slot_11_w1;
#[doc = "reg_key_slot_11_w2 (rw) register accessor: reg_key_slot_11_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_11_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_11_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_11_w2`] module"]
#[doc(alias = "reg_key_slot_11_w2")]
pub type RegKeySlot11W2 = crate::Reg<reg_key_slot_11_w2::RegKeySlot11W2Spec>;
#[doc = "reg_key_slot_11_w2."]
pub mod reg_key_slot_11_w2;
#[doc = "reg_key_slot_11_w3 (rw) register accessor: reg_key_slot_11_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_11_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_11_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_key_slot_11_w3`] module"]
#[doc(alias = "reg_key_slot_11_w3")]
pub type RegKeySlot11W3 = crate::Reg<reg_key_slot_11_w3::RegKeySlot11W3Spec>;
#[doc = "reg_key_slot_11_w3."]
pub mod reg_key_slot_11_w3;
#[doc = "reg_data_1_lock (rw) register accessor: reg_data_1_lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_1_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_1_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_data_1_lock`] module"]
#[doc(alias = "reg_data_1_lock")]
pub type RegData1Lock = crate::Reg<reg_data_1_lock::RegData1LockSpec>;
#[doc = "reg_data_1_lock."]
pub mod reg_data_1_lock;
