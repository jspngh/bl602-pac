#[doc = "Register `irtx_swm_pw_2` reader"]
pub type R = crate::R<IrtxSwmPw2Spec>;
#[doc = "Register `irtx_swm_pw_2` writer"]
pub type W = crate::W<IrtxSwmPw2Spec>;
#[doc = "Field `cr_irtx_swm_pw_2` reader - "]
pub type CrIrtxSwmPw2R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_2` writer - "]
pub type CrIrtxSwmPw2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_2(&self) -> CrIrtxSwmPw2R {
        CrIrtxSwmPw2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_2(&mut self) -> CrIrtxSwmPw2W<'_, IrtxSwmPw2Spec> {
        CrIrtxSwmPw2W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw2Spec;
impl crate::RegisterSpec for IrtxSwmPw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_2::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw2Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_2::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_2 to value 0"]
impl crate::Resettable for IrtxSwmPw2Spec {}
