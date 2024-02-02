#[doc = "Register `pwm_int_config` reader"]
pub type R = crate::R<PwmIntConfigSpec>;
#[doc = "Register `pwm_int_config` writer"]
pub type W = crate::W<PwmIntConfigSpec>;
#[doc = "Field `pwm_interrupt_sts` reader - "]
pub type PwmInterruptStsR = crate::FieldReader;
#[doc = "Field `pwm_int_clear` writer - "]
pub type PwmIntClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwm_interrupt_sts(&self) -> PwmInterruptStsR {
        PwmInterruptStsR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pwm_int_clear(&mut self) -> PwmIntClearW<'_, PwmIntConfigSpec> {
        PwmIntClearW::new(self, 8)
    }
}
#[doc = "pwm_int_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_int_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_int_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmIntConfigSpec;
impl crate::RegisterSpec for PwmIntConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_int_config::R`](R) reader structure"]
impl crate::Readable for PwmIntConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_int_config::W`](W) writer structure"]
impl crate::Writable for PwmIntConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm_int_config to value 0"]
impl crate::Resettable for PwmIntConfigSpec {}
