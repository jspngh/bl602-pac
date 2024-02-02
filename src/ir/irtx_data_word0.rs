#[doc = "Register `irtx_data_word0` reader"]
pub type R = crate::R<IrtxDataWord0Spec>;
#[doc = "Register `irtx_data_word0` writer"]
pub type W = crate::W<IrtxDataWord0Spec>;
#[doc = "Field `cr_irtx_data_word0` reader - "]
pub type CrIrtxDataWord0R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_data_word0` writer - "]
pub type CrIrtxDataWord0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_data_word0(&self) -> CrIrtxDataWord0R {
        CrIrtxDataWord0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_data_word0(&mut self) -> CrIrtxDataWord0W<'_, IrtxDataWord0Spec> {
        CrIrtxDataWord0W::new(self, 0)
    }
}
#[doc = "irtx_data_word0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_data_word0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_data_word0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxDataWord0Spec;
impl crate::RegisterSpec for IrtxDataWord0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_data_word0::R`](R) reader structure"]
impl crate::Readable for IrtxDataWord0Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_data_word0::W`](W) writer structure"]
impl crate::Writable for IrtxDataWord0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_data_word0 to value 0"]
impl crate::Resettable for IrtxDataWord0Spec {}
