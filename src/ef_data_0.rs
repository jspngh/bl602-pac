#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ef_cfg_0: EfCfg0,
    ef_dbg_pwd_low: EfDbgPwdLow,
    ef_dbg_pwd_high: EfDbgPwdHigh,
    ef_ana_trim_0: EfAnaTrim0,
    ef_sw_usage_0: EfSwUsage0,
    ef_wifi_mac_low: EfWifiMacLow,
    ef_wifi_mac_high: EfWifiMacHigh,
    ef_key_slot_0_w0: EfKeySlot0W0,
    ef_key_slot_0_w1: EfKeySlot0W1,
    ef_key_slot_0_w2: EfKeySlot0W2,
    ef_key_slot_0_w3: EfKeySlot0W3,
    ef_key_slot_1_w0: EfKeySlot1W0,
    ef_key_slot_1_w1: EfKeySlot1W1,
    ef_key_slot_1_w2: EfKeySlot1W2,
    ef_key_slot_1_w3: EfKeySlot1W3,
    ef_key_slot_2_w0: EfKeySlot2W0,
    ef_key_slot_2_w1: EfKeySlot2W1,
    ef_key_slot_2_w2: EfKeySlot2W2,
    ef_key_slot_2_w3: EfKeySlot2W3,
    ef_key_slot_3_w0: EfKeySlot3W0,
    ef_key_slot_3_w1: EfKeySlot3W1,
    ef_key_slot_3_w2: EfKeySlot3W2,
    ef_key_slot_3_w3: EfKeySlot3W3,
    ef_key_slot_4_w0: EfKeySlot4W0,
    ef_key_slot_4_w1: EfKeySlot4W1,
    ef_key_slot_4_w2: EfKeySlot4W2,
    ef_key_slot_4_w3: EfKeySlot4W3,
    ef_key_slot_5_w0: EfKeySlot5W0,
    ef_key_slot_5_w1: EfKeySlot5W1,
    ef_key_slot_5_w2: EfKeySlot5W2,
    ef_key_slot_5_w3: EfKeySlot5W3,
    ef_data_0_lock: EfData0Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - ef_cfg_0."]
    #[inline(always)]
    pub const fn ef_cfg_0(&self) -> &EfCfg0 {
        &self.ef_cfg_0
    }
    #[doc = "0x04 - ef_dbg_pwd_low."]
    #[inline(always)]
    pub const fn ef_dbg_pwd_low(&self) -> &EfDbgPwdLow {
        &self.ef_dbg_pwd_low
    }
    #[doc = "0x08 - ef_dbg_pwd_high."]
    #[inline(always)]
    pub const fn ef_dbg_pwd_high(&self) -> &EfDbgPwdHigh {
        &self.ef_dbg_pwd_high
    }
    #[doc = "0x0c - ef_ana_trim_0."]
    #[inline(always)]
    pub const fn ef_ana_trim_0(&self) -> &EfAnaTrim0 {
        &self.ef_ana_trim_0
    }
    #[doc = "0x10 - ef_sw_usage_0."]
    #[inline(always)]
    pub const fn ef_sw_usage_0(&self) -> &EfSwUsage0 {
        &self.ef_sw_usage_0
    }
    #[doc = "0x14 - ef_wifi_mac_low."]
    #[inline(always)]
    pub const fn ef_wifi_mac_low(&self) -> &EfWifiMacLow {
        &self.ef_wifi_mac_low
    }
    #[doc = "0x18 - ef_wifi_mac_high."]
    #[inline(always)]
    pub const fn ef_wifi_mac_high(&self) -> &EfWifiMacHigh {
        &self.ef_wifi_mac_high
    }
    #[doc = "0x1c - ef_key_slot_0_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_0_w0(&self) -> &EfKeySlot0W0 {
        &self.ef_key_slot_0_w0
    }
    #[doc = "0x20 - ef_key_slot_0_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_0_w1(&self) -> &EfKeySlot0W1 {
        &self.ef_key_slot_0_w1
    }
    #[doc = "0x24 - ef_key_slot_0_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_0_w2(&self) -> &EfKeySlot0W2 {
        &self.ef_key_slot_0_w2
    }
    #[doc = "0x28 - ef_key_slot_0_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_0_w3(&self) -> &EfKeySlot0W3 {
        &self.ef_key_slot_0_w3
    }
    #[doc = "0x2c - ef_key_slot_1_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_1_w0(&self) -> &EfKeySlot1W0 {
        &self.ef_key_slot_1_w0
    }
    #[doc = "0x30 - ef_key_slot_1_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_1_w1(&self) -> &EfKeySlot1W1 {
        &self.ef_key_slot_1_w1
    }
    #[doc = "0x34 - ef_key_slot_1_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_1_w2(&self) -> &EfKeySlot1W2 {
        &self.ef_key_slot_1_w2
    }
    #[doc = "0x38 - ef_key_slot_1_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_1_w3(&self) -> &EfKeySlot1W3 {
        &self.ef_key_slot_1_w3
    }
    #[doc = "0x3c - ef_key_slot_2_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_2_w0(&self) -> &EfKeySlot2W0 {
        &self.ef_key_slot_2_w0
    }
    #[doc = "0x40 - ef_key_slot_2_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_2_w1(&self) -> &EfKeySlot2W1 {
        &self.ef_key_slot_2_w1
    }
    #[doc = "0x44 - ef_key_slot_2_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_2_w2(&self) -> &EfKeySlot2W2 {
        &self.ef_key_slot_2_w2
    }
    #[doc = "0x48 - ef_key_slot_2_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_2_w3(&self) -> &EfKeySlot2W3 {
        &self.ef_key_slot_2_w3
    }
    #[doc = "0x4c - ef_key_slot_3_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_3_w0(&self) -> &EfKeySlot3W0 {
        &self.ef_key_slot_3_w0
    }
    #[doc = "0x50 - ef_key_slot_3_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_3_w1(&self) -> &EfKeySlot3W1 {
        &self.ef_key_slot_3_w1
    }
    #[doc = "0x54 - ef_key_slot_3_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_3_w2(&self) -> &EfKeySlot3W2 {
        &self.ef_key_slot_3_w2
    }
    #[doc = "0x58 - ef_key_slot_3_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_3_w3(&self) -> &EfKeySlot3W3 {
        &self.ef_key_slot_3_w3
    }
    #[doc = "0x5c - ef_key_slot_4_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_4_w0(&self) -> &EfKeySlot4W0 {
        &self.ef_key_slot_4_w0
    }
    #[doc = "0x60 - ef_key_slot_4_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_4_w1(&self) -> &EfKeySlot4W1 {
        &self.ef_key_slot_4_w1
    }
    #[doc = "0x64 - ef_key_slot_4_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_4_w2(&self) -> &EfKeySlot4W2 {
        &self.ef_key_slot_4_w2
    }
    #[doc = "0x68 - ef_key_slot_4_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_4_w3(&self) -> &EfKeySlot4W3 {
        &self.ef_key_slot_4_w3
    }
    #[doc = "0x6c - ef_key_slot_5_w0."]
    #[inline(always)]
    pub const fn ef_key_slot_5_w0(&self) -> &EfKeySlot5W0 {
        &self.ef_key_slot_5_w0
    }
    #[doc = "0x70 - ef_key_slot_5_w1."]
    #[inline(always)]
    pub const fn ef_key_slot_5_w1(&self) -> &EfKeySlot5W1 {
        &self.ef_key_slot_5_w1
    }
    #[doc = "0x74 - ef_key_slot_5_w2."]
    #[inline(always)]
    pub const fn ef_key_slot_5_w2(&self) -> &EfKeySlot5W2 {
        &self.ef_key_slot_5_w2
    }
    #[doc = "0x78 - ef_key_slot_5_w3."]
    #[inline(always)]
    pub const fn ef_key_slot_5_w3(&self) -> &EfKeySlot5W3 {
        &self.ef_key_slot_5_w3
    }
    #[doc = "0x7c - ef_data_0_lock."]
    #[inline(always)]
    pub const fn ef_data_0_lock(&self) -> &EfData0Lock {
        &self.ef_data_0_lock
    }
}
#[doc = "ef_cfg_0 (rw) register accessor: ef_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_cfg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_cfg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_cfg_0`] module"]
#[doc(alias = "ef_cfg_0")]
pub type EfCfg0 = crate::Reg<ef_cfg_0::EfCfg0Spec>;
#[doc = "ef_cfg_0."]
pub mod ef_cfg_0;
#[doc = "ef_dbg_pwd_low (rw) register accessor: ef_dbg_pwd_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd_low`] module"]
#[doc(alias = "ef_dbg_pwd_low")]
pub type EfDbgPwdLow = crate::Reg<ef_dbg_pwd_low::EfDbgPwdLowSpec>;
#[doc = "ef_dbg_pwd_low."]
pub mod ef_dbg_pwd_low;
#[doc = "ef_dbg_pwd_high (rw) register accessor: ef_dbg_pwd_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd_high`] module"]
#[doc(alias = "ef_dbg_pwd_high")]
pub type EfDbgPwdHigh = crate::Reg<ef_dbg_pwd_high::EfDbgPwdHighSpec>;
#[doc = "ef_dbg_pwd_high."]
pub mod ef_dbg_pwd_high;
#[doc = "ef_ana_trim_0 (rw) register accessor: ef_ana_trim_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ana_trim_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ana_trim_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ana_trim_0`] module"]
#[doc(alias = "ef_ana_trim_0")]
pub type EfAnaTrim0 = crate::Reg<ef_ana_trim_0::EfAnaTrim0Spec>;
#[doc = "ef_ana_trim_0."]
pub mod ef_ana_trim_0;
#[doc = "ef_sw_usage_0 (rw) register accessor: ef_sw_usage_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_usage_0`] module"]
#[doc(alias = "ef_sw_usage_0")]
pub type EfSwUsage0 = crate::Reg<ef_sw_usage_0::EfSwUsage0Spec>;
#[doc = "ef_sw_usage_0."]
pub mod ef_sw_usage_0;
#[doc = "ef_wifi_mac_low (rw) register accessor: ef_wifi_mac_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_wifi_mac_low`] module"]
#[doc(alias = "ef_wifi_mac_low")]
pub type EfWifiMacLow = crate::Reg<ef_wifi_mac_low::EfWifiMacLowSpec>;
#[doc = "ef_wifi_mac_low."]
pub mod ef_wifi_mac_low;
#[doc = "ef_wifi_mac_high (rw) register accessor: ef_wifi_mac_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_wifi_mac_high`] module"]
#[doc(alias = "ef_wifi_mac_high")]
pub type EfWifiMacHigh = crate::Reg<ef_wifi_mac_high::EfWifiMacHighSpec>;
#[doc = "ef_wifi_mac_high."]
pub mod ef_wifi_mac_high;
#[doc = "ef_key_slot_0_w0 (rw) register accessor: ef_key_slot_0_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w0`] module"]
#[doc(alias = "ef_key_slot_0_w0")]
pub type EfKeySlot0W0 = crate::Reg<ef_key_slot_0_w0::EfKeySlot0W0Spec>;
#[doc = "ef_key_slot_0_w0."]
pub mod ef_key_slot_0_w0;
#[doc = "ef_key_slot_0_w1 (rw) register accessor: ef_key_slot_0_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w1`] module"]
#[doc(alias = "ef_key_slot_0_w1")]
pub type EfKeySlot0W1 = crate::Reg<ef_key_slot_0_w1::EfKeySlot0W1Spec>;
#[doc = "ef_key_slot_0_w1."]
pub mod ef_key_slot_0_w1;
#[doc = "ef_key_slot_0_w2 (rw) register accessor: ef_key_slot_0_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w2`] module"]
#[doc(alias = "ef_key_slot_0_w2")]
pub type EfKeySlot0W2 = crate::Reg<ef_key_slot_0_w2::EfKeySlot0W2Spec>;
#[doc = "ef_key_slot_0_w2."]
pub mod ef_key_slot_0_w2;
#[doc = "ef_key_slot_0_w3 (rw) register accessor: ef_key_slot_0_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w3`] module"]
#[doc(alias = "ef_key_slot_0_w3")]
pub type EfKeySlot0W3 = crate::Reg<ef_key_slot_0_w3::EfKeySlot0W3Spec>;
#[doc = "ef_key_slot_0_w3."]
pub mod ef_key_slot_0_w3;
#[doc = "ef_key_slot_1_w0 (rw) register accessor: ef_key_slot_1_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w0`] module"]
#[doc(alias = "ef_key_slot_1_w0")]
pub type EfKeySlot1W0 = crate::Reg<ef_key_slot_1_w0::EfKeySlot1W0Spec>;
#[doc = "ef_key_slot_1_w0."]
pub mod ef_key_slot_1_w0;
#[doc = "ef_key_slot_1_w1 (rw) register accessor: ef_key_slot_1_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w1`] module"]
#[doc(alias = "ef_key_slot_1_w1")]
pub type EfKeySlot1W1 = crate::Reg<ef_key_slot_1_w1::EfKeySlot1W1Spec>;
#[doc = "ef_key_slot_1_w1."]
pub mod ef_key_slot_1_w1;
#[doc = "ef_key_slot_1_w2 (rw) register accessor: ef_key_slot_1_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w2`] module"]
#[doc(alias = "ef_key_slot_1_w2")]
pub type EfKeySlot1W2 = crate::Reg<ef_key_slot_1_w2::EfKeySlot1W2Spec>;
#[doc = "ef_key_slot_1_w2."]
pub mod ef_key_slot_1_w2;
#[doc = "ef_key_slot_1_w3 (rw) register accessor: ef_key_slot_1_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w3`] module"]
#[doc(alias = "ef_key_slot_1_w3")]
pub type EfKeySlot1W3 = crate::Reg<ef_key_slot_1_w3::EfKeySlot1W3Spec>;
#[doc = "ef_key_slot_1_w3."]
pub mod ef_key_slot_1_w3;
#[doc = "ef_key_slot_2_w0 (rw) register accessor: ef_key_slot_2_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w0`] module"]
#[doc(alias = "ef_key_slot_2_w0")]
pub type EfKeySlot2W0 = crate::Reg<ef_key_slot_2_w0::EfKeySlot2W0Spec>;
#[doc = "ef_key_slot_2_w0."]
pub mod ef_key_slot_2_w0;
#[doc = "ef_key_slot_2_w1 (rw) register accessor: ef_key_slot_2_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w1`] module"]
#[doc(alias = "ef_key_slot_2_w1")]
pub type EfKeySlot2W1 = crate::Reg<ef_key_slot_2_w1::EfKeySlot2W1Spec>;
#[doc = "ef_key_slot_2_w1."]
pub mod ef_key_slot_2_w1;
#[doc = "ef_key_slot_2_w2 (rw) register accessor: ef_key_slot_2_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w2`] module"]
#[doc(alias = "ef_key_slot_2_w2")]
pub type EfKeySlot2W2 = crate::Reg<ef_key_slot_2_w2::EfKeySlot2W2Spec>;
#[doc = "ef_key_slot_2_w2."]
pub mod ef_key_slot_2_w2;
#[doc = "ef_key_slot_2_w3 (rw) register accessor: ef_key_slot_2_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w3`] module"]
#[doc(alias = "ef_key_slot_2_w3")]
pub type EfKeySlot2W3 = crate::Reg<ef_key_slot_2_w3::EfKeySlot2W3Spec>;
#[doc = "ef_key_slot_2_w3."]
pub mod ef_key_slot_2_w3;
#[doc = "ef_key_slot_3_w0 (rw) register accessor: ef_key_slot_3_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w0`] module"]
#[doc(alias = "ef_key_slot_3_w0")]
pub type EfKeySlot3W0 = crate::Reg<ef_key_slot_3_w0::EfKeySlot3W0Spec>;
#[doc = "ef_key_slot_3_w0."]
pub mod ef_key_slot_3_w0;
#[doc = "ef_key_slot_3_w1 (rw) register accessor: ef_key_slot_3_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w1`] module"]
#[doc(alias = "ef_key_slot_3_w1")]
pub type EfKeySlot3W1 = crate::Reg<ef_key_slot_3_w1::EfKeySlot3W1Spec>;
#[doc = "ef_key_slot_3_w1."]
pub mod ef_key_slot_3_w1;
#[doc = "ef_key_slot_3_w2 (rw) register accessor: ef_key_slot_3_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w2`] module"]
#[doc(alias = "ef_key_slot_3_w2")]
pub type EfKeySlot3W2 = crate::Reg<ef_key_slot_3_w2::EfKeySlot3W2Spec>;
#[doc = "ef_key_slot_3_w2."]
pub mod ef_key_slot_3_w2;
#[doc = "ef_key_slot_3_w3 (rw) register accessor: ef_key_slot_3_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w3`] module"]
#[doc(alias = "ef_key_slot_3_w3")]
pub type EfKeySlot3W3 = crate::Reg<ef_key_slot_3_w3::EfKeySlot3W3Spec>;
#[doc = "ef_key_slot_3_w3."]
pub mod ef_key_slot_3_w3;
#[doc = "ef_key_slot_4_w0 (rw) register accessor: ef_key_slot_4_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w0`] module"]
#[doc(alias = "ef_key_slot_4_w0")]
pub type EfKeySlot4W0 = crate::Reg<ef_key_slot_4_w0::EfKeySlot4W0Spec>;
#[doc = "ef_key_slot_4_w0."]
pub mod ef_key_slot_4_w0;
#[doc = "ef_key_slot_4_w1 (rw) register accessor: ef_key_slot_4_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w1`] module"]
#[doc(alias = "ef_key_slot_4_w1")]
pub type EfKeySlot4W1 = crate::Reg<ef_key_slot_4_w1::EfKeySlot4W1Spec>;
#[doc = "ef_key_slot_4_w1."]
pub mod ef_key_slot_4_w1;
#[doc = "ef_key_slot_4_w2 (rw) register accessor: ef_key_slot_4_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w2`] module"]
#[doc(alias = "ef_key_slot_4_w2")]
pub type EfKeySlot4W2 = crate::Reg<ef_key_slot_4_w2::EfKeySlot4W2Spec>;
#[doc = "ef_key_slot_4_w2."]
pub mod ef_key_slot_4_w2;
#[doc = "ef_key_slot_4_w3 (rw) register accessor: ef_key_slot_4_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w3`] module"]
#[doc(alias = "ef_key_slot_4_w3")]
pub type EfKeySlot4W3 = crate::Reg<ef_key_slot_4_w3::EfKeySlot4W3Spec>;
#[doc = "ef_key_slot_4_w3."]
pub mod ef_key_slot_4_w3;
#[doc = "ef_key_slot_5_w0 (rw) register accessor: ef_key_slot_5_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w0`] module"]
#[doc(alias = "ef_key_slot_5_w0")]
pub type EfKeySlot5W0 = crate::Reg<ef_key_slot_5_w0::EfKeySlot5W0Spec>;
#[doc = "ef_key_slot_5_w0."]
pub mod ef_key_slot_5_w0;
#[doc = "ef_key_slot_5_w1 (rw) register accessor: ef_key_slot_5_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w1`] module"]
#[doc(alias = "ef_key_slot_5_w1")]
pub type EfKeySlot5W1 = crate::Reg<ef_key_slot_5_w1::EfKeySlot5W1Spec>;
#[doc = "ef_key_slot_5_w1."]
pub mod ef_key_slot_5_w1;
#[doc = "ef_key_slot_5_w2 (rw) register accessor: ef_key_slot_5_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w2`] module"]
#[doc(alias = "ef_key_slot_5_w2")]
pub type EfKeySlot5W2 = crate::Reg<ef_key_slot_5_w2::EfKeySlot5W2Spec>;
#[doc = "ef_key_slot_5_w2."]
pub mod ef_key_slot_5_w2;
#[doc = "ef_key_slot_5_w3 (rw) register accessor: ef_key_slot_5_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w3`] module"]
#[doc(alias = "ef_key_slot_5_w3")]
pub type EfKeySlot5W3 = crate::Reg<ef_key_slot_5_w3::EfKeySlot5W3Spec>;
#[doc = "ef_key_slot_5_w3."]
pub mod ef_key_slot_5_w3;
#[doc = "ef_data_0_lock (rw) register accessor: ef_data_0_lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_0_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_0_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_data_0_lock`] module"]
#[doc(alias = "ef_data_0_lock")]
pub type EfData0Lock = crate::Reg<ef_data_0_lock::EfData0LockSpec>;
#[doc = "ef_data_0_lock."]
pub mod ef_data_0_lock;
