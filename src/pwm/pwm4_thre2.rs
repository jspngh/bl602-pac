#[doc = "Register `pwm4_thre2` reader"]
pub type R = crate::R<Pwm4Thre2Spec>;
#[doc = "Register `pwm4_thre2` writer"]
pub type W = crate::W<Pwm4Thre2Spec>;
#[doc = "Field `pwm_thre2` reader - "]
pub type PwmThre2R = crate::FieldReader<u16>;
#[doc = "Field `pwm_thre2` writer - "]
pub type PwmThre2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre2(&self) -> PwmThre2R {
        PwmThre2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre2(&mut self) -> PwmThre2W<'_, Pwm4Thre2Spec> {
        PwmThre2W::new(self, 0)
    }
}
#[doc = "pwm4_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_thre2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_thre2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm4Thre2Spec;
impl crate::RegisterSpec for Pwm4Thre2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm4_thre2::R`](R) reader structure"]
impl crate::Readable for Pwm4Thre2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwm4_thre2::W`](W) writer structure"]
impl crate::Writable for Pwm4Thre2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm4_thre2 to value 0"]
impl crate::Resettable for Pwm4Thre2Spec {}
