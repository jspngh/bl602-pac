#[doc = "Register `WVR` reader"]
pub type R = crate::R<WvrSpec>;
#[doc = "Field `wvr` reader - "]
pub type WvrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&self) -> WvrR {
        WvrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "WVR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WvrSpec;
impl crate::RegisterSpec for WvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wvr::R`](R) reader structure"]
impl crate::Readable for WvrSpec {}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WvrSpec {}
