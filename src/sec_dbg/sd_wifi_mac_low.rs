#[doc = "Register `sd_wifi_mac_low` reader"]
pub type R = crate::R<SdWifiMacLowSpec>;
#[doc = "Field `sd_wifi_mac_low` reader - "]
pub type SdWifiMacLowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_low(&self) -> SdWifiMacLowR {
        SdWifiMacLowR::new(self.bits)
    }
}
#[doc = "sd_wifi_mac_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdWifiMacLowSpec;
impl crate::RegisterSpec for SdWifiMacLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_wifi_mac_low::R`](R) reader structure"]
impl crate::Readable for SdWifiMacLowSpec {}
#[doc = "`reset()` method sets sd_wifi_mac_low to value 0"]
impl crate::Resettable for SdWifiMacLowSpec {}
