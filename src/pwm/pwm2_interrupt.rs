#[doc = "Register `pwm2_interrupt` reader"]
pub type R = crate::R<Pwm2InterruptSpec>;
#[doc = "Register `pwm2_interrupt` writer"]
pub type W = crate::W<Pwm2InterruptSpec>;
#[doc = "Field `pwm_int_period_cnt` reader - "]
pub type PwmIntPeriodCntR = crate::FieldReader<u16>;
#[doc = "Field `pwm_int_period_cnt` writer - "]
pub type PwmIntPeriodCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pwm_int_enable` reader - "]
pub type PwmIntEnableR = crate::BitReader;
#[doc = "Field `pwm_int_enable` writer - "]
pub type PwmIntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&self) -> PwmIntPeriodCntR {
        PwmIntPeriodCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&self) -> PwmIntEnableR {
        PwmIntEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&mut self) -> PwmIntPeriodCntW<'_, Pwm2InterruptSpec> {
        PwmIntPeriodCntW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&mut self) -> PwmIntEnableW<'_, Pwm2InterruptSpec> {
        PwmIntEnableW::new(self, 16)
    }
}
#[doc = "pwm2_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2InterruptSpec;
impl crate::RegisterSpec for Pwm2InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_interrupt::R`](R) reader structure"]
impl crate::Readable for Pwm2InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_interrupt::W`](W) writer structure"]
impl crate::Writable for Pwm2InterruptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm2_interrupt to value 0"]
impl crate::Resettable for Pwm2InterruptSpec {}
