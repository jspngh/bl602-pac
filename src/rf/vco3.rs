#[doc = "Register `vco3` reader"]
pub type R = crate::R<Vco3Spec>;
#[doc = "Register `vco3` writer"]
pub type W = crate::W<Vco3Spec>;
#[doc = "Field `fcal_div` reader - "]
pub type FcalDivR = crate::FieldReader<u16>;
#[doc = "Field `fcal_div` writer - "]
pub type FcalDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `fcal_cnt_op` reader - "]
pub type FcalCntOpR = crate::FieldReader<u16>;
#[doc = "Field `fcal_cnt_op` writer - "]
pub type FcalCntOpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&self) -> FcalDivR {
        FcalDivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&self) -> FcalCntOpR {
        FcalCntOpR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&mut self) -> FcalDivW<'_, Vco3Spec> {
        FcalDivW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&mut self) -> FcalCntOpW<'_, Vco3Spec> {
        FcalCntOpW::new(self, 16)
    }
}
#[doc = "vco3.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vco3Spec;
impl crate::RegisterSpec for Vco3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco3::R`](R) reader structure"]
impl crate::Readable for Vco3Spec {}
#[doc = "`write(|w| ..)` method takes [`vco3::W`](W) writer structure"]
impl crate::Writable for Vco3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vco3 to value 0"]
impl crate::Resettable for Vco3Spec {}
