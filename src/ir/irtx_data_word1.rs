#[doc = "Register `irtx_data_word1` reader"]
pub type R = crate::R<IrtxDataWord1Spec>;
#[doc = "Register `irtx_data_word1` writer"]
pub type W = crate::W<IrtxDataWord1Spec>;
#[doc = "Field `cr_irtx_data_word1` reader - "]
pub type CrIrtxDataWord1R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_data_word1` writer - "]
pub type CrIrtxDataWord1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_data_word1(&self) -> CrIrtxDataWord1R {
        CrIrtxDataWord1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_data_word1(&mut self) -> CrIrtxDataWord1W<'_, IrtxDataWord1Spec> {
        CrIrtxDataWord1W::new(self, 0)
    }
}
#[doc = "irtx_data_word1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_data_word1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_data_word1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxDataWord1Spec;
impl crate::RegisterSpec for IrtxDataWord1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_data_word1::R`](R) reader structure"]
impl crate::Readable for IrtxDataWord1Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_data_word1::W`](W) writer structure"]
impl crate::Writable for IrtxDataWord1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_data_word1 to value 0"]
impl crate::Resettable for IrtxDataWord1Spec {}
