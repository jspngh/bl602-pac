#[doc = "Register `irtx_swm_pw_1` reader"]
pub type R = crate::R<IrtxSwmPw1Spec>;
#[doc = "Register `irtx_swm_pw_1` writer"]
pub type W = crate::W<IrtxSwmPw1Spec>;
#[doc = "Field `cr_irtx_swm_pw_1` reader - "]
pub type CrIrtxSwmPw1R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_1` writer - "]
pub type CrIrtxSwmPw1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_1(&self) -> CrIrtxSwmPw1R {
        CrIrtxSwmPw1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_1(&mut self) -> CrIrtxSwmPw1W<'_, IrtxSwmPw1Spec> {
        CrIrtxSwmPw1W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw1Spec;
impl crate::RegisterSpec for IrtxSwmPw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_1::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw1Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_1::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_1 to value 0"]
impl crate::Resettable for IrtxSwmPw1Spec {}
