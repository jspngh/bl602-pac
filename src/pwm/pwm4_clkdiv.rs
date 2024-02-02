#[doc = "Register `pwm4_clkdiv` reader"]
pub type R = crate::R<Pwm4ClkdivSpec>;
#[doc = "Register `pwm4_clkdiv` writer"]
pub type W = crate::W<Pwm4ClkdivSpec>;
#[doc = "Field `pwm_clk_div` reader - "]
pub type PwmClkDivR = crate::FieldReader<u16>;
#[doc = "Field `pwm_clk_div` writer - "]
pub type PwmClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_clk_div(&self) -> PwmClkDivR {
        PwmClkDivR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_clk_div(&mut self) -> PwmClkDivW<'_, Pwm4ClkdivSpec> {
        PwmClkDivW::new(self, 0)
    }
}
#[doc = "pwm4_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm4ClkdivSpec;
impl crate::RegisterSpec for Pwm4ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm4_clkdiv::R`](R) reader structure"]
impl crate::Readable for Pwm4ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm4_clkdiv::W`](W) writer structure"]
impl crate::Writable for Pwm4ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm4_clkdiv to value 0"]
impl crate::Resettable for Pwm4ClkdivSpec {}
