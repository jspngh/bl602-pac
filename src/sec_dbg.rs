#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sd_chip_id_low: SdChipIdLow,
    sd_chip_id_high: SdChipIdHigh,
    sd_wifi_mac_low: SdWifiMacLow,
    sd_wifi_mac_high: SdWifiMacHigh,
    sd_dbg_pwd_low: SdDbgPwdLow,
    sd_dbg_pwd_high: SdDbgPwdHigh,
    sd_status: SdStatus,
    sd_dbg_reserved: SdDbgReserved,
}
impl RegisterBlock {
    #[doc = "0x00 - sd_chip_id_low."]
    #[inline(always)]
    pub const fn sd_chip_id_low(&self) -> &SdChipIdLow {
        &self.sd_chip_id_low
    }
    #[doc = "0x04 - sd_chip_id_high."]
    #[inline(always)]
    pub const fn sd_chip_id_high(&self) -> &SdChipIdHigh {
        &self.sd_chip_id_high
    }
    #[doc = "0x08 - sd_wifi_mac_low."]
    #[inline(always)]
    pub const fn sd_wifi_mac_low(&self) -> &SdWifiMacLow {
        &self.sd_wifi_mac_low
    }
    #[doc = "0x0c - sd_wifi_mac_high."]
    #[inline(always)]
    pub const fn sd_wifi_mac_high(&self) -> &SdWifiMacHigh {
        &self.sd_wifi_mac_high
    }
    #[doc = "0x10 - sd_dbg_pwd_low."]
    #[inline(always)]
    pub const fn sd_dbg_pwd_low(&self) -> &SdDbgPwdLow {
        &self.sd_dbg_pwd_low
    }
    #[doc = "0x14 - sd_dbg_pwd_high."]
    #[inline(always)]
    pub const fn sd_dbg_pwd_high(&self) -> &SdDbgPwdHigh {
        &self.sd_dbg_pwd_high
    }
    #[doc = "0x18 - sd_status."]
    #[inline(always)]
    pub const fn sd_status(&self) -> &SdStatus {
        &self.sd_status
    }
    #[doc = "0x1c - sd_dbg_reserved."]
    #[inline(always)]
    pub const fn sd_dbg_reserved(&self) -> &SdDbgReserved {
        &self.sd_dbg_reserved
    }
}
#[doc = "sd_chip_id_low (r) register accessor: sd_chip_id_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_chip_id_low`] module"]
#[doc(alias = "sd_chip_id_low")]
pub type SdChipIdLow = crate::Reg<sd_chip_id_low::SdChipIdLowSpec>;
#[doc = "sd_chip_id_low."]
pub mod sd_chip_id_low;
#[doc = "sd_chip_id_high (r) register accessor: sd_chip_id_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_chip_id_high`] module"]
#[doc(alias = "sd_chip_id_high")]
pub type SdChipIdHigh = crate::Reg<sd_chip_id_high::SdChipIdHighSpec>;
#[doc = "sd_chip_id_high."]
pub mod sd_chip_id_high;
#[doc = "sd_wifi_mac_low (r) register accessor: sd_wifi_mac_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_wifi_mac_low`] module"]
#[doc(alias = "sd_wifi_mac_low")]
pub type SdWifiMacLow = crate::Reg<sd_wifi_mac_low::SdWifiMacLowSpec>;
#[doc = "sd_wifi_mac_low."]
pub mod sd_wifi_mac_low;
#[doc = "sd_wifi_mac_high (r) register accessor: sd_wifi_mac_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_wifi_mac_high`] module"]
#[doc(alias = "sd_wifi_mac_high")]
pub type SdWifiMacHigh = crate::Reg<sd_wifi_mac_high::SdWifiMacHighSpec>;
#[doc = "sd_wifi_mac_high."]
pub mod sd_wifi_mac_high;
#[doc = "sd_dbg_pwd_low (rw) register accessor: sd_dbg_pwd_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd_low`] module"]
#[doc(alias = "sd_dbg_pwd_low")]
pub type SdDbgPwdLow = crate::Reg<sd_dbg_pwd_low::SdDbgPwdLowSpec>;
#[doc = "sd_dbg_pwd_low."]
pub mod sd_dbg_pwd_low;
#[doc = "sd_dbg_pwd_high (rw) register accessor: sd_dbg_pwd_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd_high`] module"]
#[doc(alias = "sd_dbg_pwd_high")]
pub type SdDbgPwdHigh = crate::Reg<sd_dbg_pwd_high::SdDbgPwdHighSpec>;
#[doc = "sd_dbg_pwd_high."]
pub mod sd_dbg_pwd_high;
#[doc = "sd_status (rw) register accessor: sd_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_status`] module"]
#[doc(alias = "sd_status")]
pub type SdStatus = crate::Reg<sd_status::SdStatusSpec>;
#[doc = "sd_status."]
pub mod sd_status;
#[doc = "sd_dbg_reserved (rw) register accessor: sd_dbg_reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_reserved`] module"]
#[doc(alias = "sd_dbg_reserved")]
pub type SdDbgReserved = crate::Reg<sd_dbg_reserved::SdDbgReservedSpec>;
#[doc = "sd_dbg_reserved."]
pub mod sd_dbg_reserved;
