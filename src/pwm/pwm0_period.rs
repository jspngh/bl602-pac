#[doc = "Register `pwm0_period` reader"]
pub type R = crate::R<Pwm0PeriodSpec>;
#[doc = "Register `pwm0_period` writer"]
pub type W = crate::W<Pwm0PeriodSpec>;
#[doc = "Field `pwm_period` reader - "]
pub type PwmPeriodR = crate::FieldReader<u16>;
#[doc = "Field `pwm_period` writer - "]
pub type PwmPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&self) -> PwmPeriodR {
        PwmPeriodR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PwmPeriodW<'_, Pwm0PeriodSpec> {
        PwmPeriodW::new(self, 0)
    }
}
#[doc = "pwm0_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm0PeriodSpec;
impl crate::RegisterSpec for Pwm0PeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm0_period::R`](R) reader structure"]
impl crate::Readable for Pwm0PeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm0_period::W`](W) writer structure"]
impl crate::Writable for Pwm0PeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm0_period to value 0"]
impl crate::Resettable for Pwm0PeriodSpec {}
