#[doc = "Register `irtx_swm_pw_0` reader"]
pub type R = crate::R<IrtxSwmPw0Spec>;
#[doc = "Register `irtx_swm_pw_0` writer"]
pub type W = crate::W<IrtxSwmPw0Spec>;
#[doc = "Field `cr_irtx_swm_pw_0` reader - "]
pub type CrIrtxSwmPw0R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_0` writer - "]
pub type CrIrtxSwmPw0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_0(&self) -> CrIrtxSwmPw0R {
        CrIrtxSwmPw0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_0(&mut self) -> CrIrtxSwmPw0W<'_, IrtxSwmPw0Spec> {
        CrIrtxSwmPw0W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw0Spec;
impl crate::RegisterSpec for IrtxSwmPw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_0::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw0Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_0::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_0 to value 0"]
impl crate::Resettable for IrtxSwmPw0Spec {}
