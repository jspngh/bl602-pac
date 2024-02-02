#[doc = "Register `rf_rev` reader"]
pub type R = crate::R<RfRevSpec>;
#[doc = "Register `rf_rev` writer"]
pub type W = crate::W<RfRevSpec>;
#[doc = "Field `rf_id` reader - "]
pub type RfIdR = crate::FieldReader;
#[doc = "Field `rf_id` writer - "]
pub type RfIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `fw_rev` reader - "]
pub type FwRevR = crate::FieldReader;
#[doc = "Field `fw_rev` writer - "]
pub type FwRevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hw_rev` reader - "]
pub type HwRevR = crate::FieldReader;
#[doc = "Field `hw_rev` writer - "]
pub type HwRevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&self) -> RfIdR {
        RfIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&self) -> FwRevR {
        FwRevR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&self) -> HwRevR {
        HwRevR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&mut self) -> RfIdW<'_, RfRevSpec> {
        RfIdW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&mut self) -> FwRevW<'_, RfRevSpec> {
        FwRevW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&mut self) -> HwRevW<'_, RfRevSpec> {
        HwRevW::new(self, 16)
    }
}
#[doc = "Silicon revision\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfRevSpec;
impl crate::RegisterSpec for RfRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_rev::R`](R) reader structure"]
impl crate::Readable for RfRevSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_rev::W`](W) writer structure"]
impl crate::Writable for RfRevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_rev to value 0"]
impl crate::Resettable for RfRevSpec {}
