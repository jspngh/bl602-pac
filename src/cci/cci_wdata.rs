#[doc = "Register `cci_wdata` reader"]
pub type R = crate::R<CciWdataSpec>;
#[doc = "Register `cci_wdata` writer"]
pub type W = crate::W<CciWdataSpec>;
#[doc = "Field `apb_cci_wdata` reader - "]
pub type ApbCciWdataR = crate::FieldReader<u32>;
#[doc = "Field `apb_cci_wdata` writer - "]
pub type ApbCciWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_wdata(&self) -> ApbCciWdataR {
        ApbCciWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_wdata(&mut self) -> ApbCciWdataW<'_, CciWdataSpec> {
        ApbCciWdataW::new(self, 0)
    }
}
#[doc = "cci_wdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciWdataSpec;
impl crate::RegisterSpec for CciWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_wdata::R`](R) reader structure"]
impl crate::Readable for CciWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cci_wdata::W`](W) writer structure"]
impl crate::Writable for CciWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cci_wdata to value 0"]
impl crate::Resettable for CciWdataSpec {}
