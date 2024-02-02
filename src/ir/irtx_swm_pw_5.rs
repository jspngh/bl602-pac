#[doc = "Register `irtx_swm_pw_5` reader"]
pub type R = crate::R<IrtxSwmPw5Spec>;
#[doc = "Register `irtx_swm_pw_5` writer"]
pub type W = crate::W<IrtxSwmPw5Spec>;
#[doc = "Field `cr_irtx_swm_pw_5` reader - "]
pub type CrIrtxSwmPw5R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_5` writer - "]
pub type CrIrtxSwmPw5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_5(&self) -> CrIrtxSwmPw5R {
        CrIrtxSwmPw5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_5(&mut self) -> CrIrtxSwmPw5W<'_, IrtxSwmPw5Spec> {
        CrIrtxSwmPw5W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw5Spec;
impl crate::RegisterSpec for IrtxSwmPw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_5::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw5Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_5::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_5 to value 0"]
impl crate::Resettable for IrtxSwmPw5Spec {}
