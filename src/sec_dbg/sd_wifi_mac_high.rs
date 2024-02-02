#[doc = "Register `sd_wifi_mac_high` reader"]
pub type R = crate::R<SdWifiMacHighSpec>;
#[doc = "Field `sd_wifi_mac_high` reader - "]
pub type SdWifiMacHighR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&self) -> SdWifiMacHighR {
        SdWifiMacHighR::new(self.bits)
    }
}
#[doc = "sd_wifi_mac_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_high::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdWifiMacHighSpec;
impl crate::RegisterSpec for SdWifiMacHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_wifi_mac_high::R`](R) reader structure"]
impl crate::Readable for SdWifiMacHighSpec {}
#[doc = "`reset()` method sets sd_wifi_mac_high to value 0"]
impl crate::Resettable for SdWifiMacHighSpec {}
