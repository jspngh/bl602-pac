#[doc = "Register `irtx_swm_pw_4` reader"]
pub type R = crate::R<IrtxSwmPw4Spec>;
#[doc = "Register `irtx_swm_pw_4` writer"]
pub type W = crate::W<IrtxSwmPw4Spec>;
#[doc = "Field `cr_irtx_swm_pw_4` reader - "]
pub type CrIrtxSwmPw4R = crate::FieldReader<u32>;
#[doc = "Field `cr_irtx_swm_pw_4` writer - "]
pub type CrIrtxSwmPw4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_4(&self) -> CrIrtxSwmPw4R {
        CrIrtxSwmPw4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_4(&mut self) -> CrIrtxSwmPw4W<'_, IrtxSwmPw4Spec> {
        CrIrtxSwmPw4W::new(self, 0)
    }
}
#[doc = "irtx_swm_pw_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_swm_pw_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_swm_pw_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxSwmPw4Spec;
impl crate::RegisterSpec for IrtxSwmPw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_swm_pw_4::R`](R) reader structure"]
impl crate::Readable for IrtxSwmPw4Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_swm_pw_4::W`](W) writer structure"]
impl crate::Writable for IrtxSwmPw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_swm_pw_4 to value 0"]
impl crate::Resettable for IrtxSwmPw4Spec {}
