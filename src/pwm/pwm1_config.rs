#[doc = "Register `pwm1_config` reader"]
pub type R = crate::R<Pwm1ConfigSpec>;
#[doc = "Register `pwm1_config` writer"]
pub type W = crate::W<Pwm1ConfigSpec>;
#[doc = "Field `reg_clk_sel` reader - "]
pub type RegClkSelR = crate::FieldReader;
#[doc = "Field `reg_clk_sel` writer - "]
pub type RegClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pwm_out_inv` reader - "]
pub type PwmOutInvR = crate::BitReader;
#[doc = "Field `pwm_out_inv` writer - "]
pub type PwmOutInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_stop_mode` reader - "]
pub type PwmStopModeR = crate::BitReader;
#[doc = "Field `pwm_stop_mode` writer - "]
pub type PwmStopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_sw_force_val` reader - "]
pub type PwmSwForceValR = crate::BitReader;
#[doc = "Field `pwm_sw_force_val` writer - "]
pub type PwmSwForceValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_sw_mode` reader - "]
pub type PwmSwModeR = crate::BitReader;
#[doc = "Field `pwm_sw_mode` writer - "]
pub type PwmSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_stop_en` reader - "]
pub type PwmStopEnR = crate::BitReader;
#[doc = "Field `pwm_stop_en` writer - "]
pub type PwmStopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_sts_top` reader - "]
pub type PwmStsTopR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> RegClkSelR {
        RegClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&self) -> PwmOutInvR {
        PwmOutInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&self) -> PwmStopModeR {
        PwmStopModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&self) -> PwmSwForceValR {
        PwmSwForceValR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&self) -> PwmSwModeR {
        PwmSwModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&self) -> PwmStopEnR {
        PwmStopEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_sts_top(&self) -> PwmStsTopR {
        PwmStsTopR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&mut self) -> RegClkSelW<'_, Pwm1ConfigSpec> {
        RegClkSelW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&mut self) -> PwmOutInvW<'_, Pwm1ConfigSpec> {
        PwmOutInvW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&mut self) -> PwmStopModeW<'_, Pwm1ConfigSpec> {
        PwmStopModeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&mut self) -> PwmSwForceValW<'_, Pwm1ConfigSpec> {
        PwmSwForceValW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&mut self) -> PwmSwModeW<'_, Pwm1ConfigSpec> {
        PwmSwModeW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&mut self) -> PwmStopEnW<'_, Pwm1ConfigSpec> {
        PwmStopEnW::new(self, 6)
    }
}
#[doc = "pwm1_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1ConfigSpec;
impl crate::RegisterSpec for Pwm1ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_config::R`](R) reader structure"]
impl crate::Readable for Pwm1ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_config::W`](W) writer structure"]
impl crate::Writable for Pwm1ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pwm1_config to value 0x08"]
impl crate::Resettable for Pwm1ConfigSpec {
    const RESET_VALUE: u32 = 0x08;
}
