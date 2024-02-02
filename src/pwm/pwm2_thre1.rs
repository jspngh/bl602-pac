#[doc = "Register `pwm2_thre1` reader"]
pub type R = crate::R<Pwm2Thre1Spec>;
#[doc = "Register `pwm2_thre1` writer"]
pub type W = crate::W<Pwm2Thre1Spec>;
#[doc = "Field `pwm_thre1` reader - "]
pub type PwmThre1R = crate::FieldReader<u16>;
#[doc = "Field `pwm_thre1` writer - "]
pub type PwmThre1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre1(&self) -> PwmThre1R {
        PwmThre1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre1(&mut self) -> PwmThre1W<'_, Pwm2Thre1Spec> {
        PwmThre1W::new(self, 0)
    }
}
#[doc = "pwm2_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_thre1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_thre1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2Thre1Spec;
impl crate::RegisterSpec for Pwm2Thre1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_thre1::R`](R) reader structure"]
impl crate::Readable for Pwm2Thre1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_thre1::W`](W) writer structure"]
impl crate::Writable for Pwm2Thre1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm2_thre1 to value 0"]
impl crate::Resettable for Pwm2Thre1Spec {}
