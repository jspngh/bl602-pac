#[doc = "Register `irtx_swm_pw_7` reader"]
pub type R = crate::R<IrtxSwmPw7Spec>;
#[doc = "Register `irtx_swm_pw_7` writer"]
pub type W = crate::W<IrtxSwmPw7Spec>;
#[doc = "Field `cr_irtx_swm_pw_7` reader - "]
pub type CrIrtxSwmPw7R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_7` writer - "]
pub type CrIrtxSwmPw7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_7(&self) -> CrIrtxSwmPw7R {
        CrIrtxSwmPw7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_7(&mut self) -> CrIrtxSwmPw7W<'_, IrtxSwmPw7Spec> {
        CrIrtxSwmPw7W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw7Spec;
impl crate::RegisterSpec for IrtxSwmPw7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_7::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw7Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_7::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_7 to value 0"]
impl crate::Resettable for IrtxSwmPw7Spec {}
