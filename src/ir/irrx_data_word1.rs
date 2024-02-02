#[doc = "Register `irrx_data_word1` reader"]
pub type R = crate::R<IrrxDataWord1Spec>;
#[doc = "Register `irrx_data_word1` writer"]
pub type W = crate::W<IrrxDataWord1Spec>;
#[doc = "Field `sts_irrx_data_word1` reader - "]
pub type StsIrrxDataWord1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_irrx_data_word1(&self) -> StsIrrxDataWord1R {
        StsIrrxDataWord1R::new(self.bits)
    }
}
impl W {}
#[doc = "irrx_data_word1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_word1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataWord1Spec;
impl crate::RegisterSpec for IrrxDataWord1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_word1::R`](R) reader structure"]
impl crate::Readable for IrrxDataWord1Spec {}
#[doc = "`write(|w| ..)` method takes [`irrx_data_word1::W`](W) writer structure"]
impl crate::Writable for IrrxDataWord1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_data_word1 to value 0"]
impl crate::Resettable for IrrxDataWord1Spec {}
