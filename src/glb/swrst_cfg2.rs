#[doc = "Register `swrst_cfg2` reader"]
pub type R = crate::R<SwrstCfg2Spec>;
#[doc = "Register `swrst_cfg2` writer"]
pub type W = crate::W<SwrstCfg2Spec>;
#[doc = "Field `reg_ctrl_pwron_rst` reader - "]
pub type RegCtrlPwronRstR = crate::BitReader;
#[doc = "Field `reg_ctrl_pwron_rst` writer - "]
pub type RegCtrlPwronRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_cpu_reset` reader - "]
pub type RegCtrlCpuResetR = crate::BitReader;
#[doc = "Field `reg_ctrl_cpu_reset` writer - "]
pub type RegCtrlCpuResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_sys_reset` reader - "]
pub type RegCtrlSysResetR = crate::BitReader;
#[doc = "Field `reg_ctrl_sys_reset` writer - "]
pub type RegCtrlSysResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_reset_dummy` reader - "]
pub type RegCtrlResetDummyR = crate::FieldReader;
#[doc = "Field `reg_ctrl_reset_dummy` writer - "]
pub type RegCtrlResetDummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pka_clk_sel` reader - "]
pub type PkaClkSelR = crate::BitReader;
#[doc = "Field `pka_clk_sel` writer - "]
pub type PkaClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> RegCtrlPwronRstR {
        RegCtrlPwronRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> RegCtrlCpuResetR {
        RegCtrlCpuResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> RegCtrlSysResetR {
        RegCtrlSysResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> RegCtrlResetDummyR {
        RegCtrlResetDummyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PkaClkSelR {
        PkaClkSelR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&mut self) -> RegCtrlPwronRstW<'_, SwrstCfg2Spec> {
        RegCtrlPwronRstW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&mut self) -> RegCtrlCpuResetW<'_, SwrstCfg2Spec> {
        RegCtrlCpuResetW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&mut self) -> RegCtrlSysResetW<'_, SwrstCfg2Spec> {
        RegCtrlSysResetW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&mut self) -> RegCtrlResetDummyW<'_, SwrstCfg2Spec> {
        RegCtrlResetDummyW::new(self, 4)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&mut self) -> PkaClkSelW<'_, SwrstCfg2Spec> {
        PkaClkSelW::new(self, 24)
    }
}
#[doc = "swrst_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCfg2Spec;
impl crate::RegisterSpec for SwrstCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_cfg2::R`](R) reader structure"]
impl crate::Readable for SwrstCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_cfg2::W`](W) writer structure"]
impl crate::Writable for SwrstCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets swrst_cfg2 to value 0"]
impl crate::Resettable for SwrstCfg2Spec {}
