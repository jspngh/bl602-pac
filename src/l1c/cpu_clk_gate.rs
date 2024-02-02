#[doc = "Register `cpu_clk_gate` reader"]
pub type R = crate::R<CpuClkGateSpec>;
#[doc = "Register `cpu_clk_gate` writer"]
pub type W = crate::W<CpuClkGateSpec>;
#[doc = "Field `force_e21_clock_on_0` reader - "]
pub type ForceE21ClockOn0R = crate::BitReader;
#[doc = "Field `force_e21_clock_on_0` writer - "]
pub type ForceE21ClockOn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_e21_clock_on_1` reader - "]
pub type ForceE21ClockOn1R = crate::BitReader;
#[doc = "Field `force_e21_clock_on_1` writer - "]
pub type ForceE21ClockOn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_e21_clock_on_2` reader - "]
pub type ForceE21ClockOn2R = crate::BitReader;
#[doc = "Field `force_e21_clock_on_2` writer - "]
pub type ForceE21ClockOn2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&self) -> ForceE21ClockOn0R {
        ForceE21ClockOn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&self) -> ForceE21ClockOn1R {
        ForceE21ClockOn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&self) -> ForceE21ClockOn2R {
        ForceE21ClockOn2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&mut self) -> ForceE21ClockOn0W<'_, CpuClkGateSpec> {
        ForceE21ClockOn0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&mut self) -> ForceE21ClockOn1W<'_, CpuClkGateSpec> {
        ForceE21ClockOn1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&mut self) -> ForceE21ClockOn2W<'_, CpuClkGateSpec> {
        ForceE21ClockOn2W::new(self, 2)
    }
}
#[doc = "cpu_clk_gate.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuClkGateSpec;
impl crate::RegisterSpec for CpuClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_clk_gate::R`](R) reader structure"]
impl crate::Readable for CpuClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_clk_gate::W`](W) writer structure"]
impl crate::Writable for CpuClkGateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cpu_clk_gate to value 0"]
impl crate::Resettable for CpuClkGateSpec {}
