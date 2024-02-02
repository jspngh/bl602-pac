#[doc = "Register `cci_rdata` reader"]
pub type R = crate::R<CciRdataSpec>;
#[doc = "Register `cci_rdata` writer"]
pub type W = crate::W<CciRdataSpec>;
#[doc = "Field `apb_cci_rdata` reader - "]
pub type ApbCciRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_rdata(&self) -> ApbCciRdataR {
        ApbCciRdataR::new(self.bits)
    }
}
impl W {}
#[doc = "cci_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciRdataSpec;
impl crate::RegisterSpec for CciRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_rdata::R`](R) reader structure"]
impl crate::Readable for CciRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cci_rdata::W`](W) writer structure"]
impl crate::Writable for CciRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cci_rdata to value 0"]
impl crate::Resettable for CciRdataSpec {}
