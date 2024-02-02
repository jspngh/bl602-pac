#[doc = "Register `CPU_CLK_CFG` reader"]
pub type R = crate::R<CpuClkCfgSpec>;
#[doc = "Register `CPU_CLK_CFG` writer"]
pub type W = crate::W<CpuClkCfgSpec>;
#[doc = "Field `cpu_rtc_div` reader - "]
pub type CpuRtcDivR = crate::FieldReader<u32>;
#[doc = "Field `cpu_rtc_div` writer - "]
pub type CpuRtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `cpu_rtc_en` reader - "]
pub type CpuRtcEnR = crate::BitReader;
#[doc = "Field `cpu_rtc_en` writer - "]
pub type CpuRtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_rtc_sel` reader - "]
pub type CpuRtcSelR = crate::BitReader;
#[doc = "Field `cpu_rtc_sel` writer - "]
pub type CpuRtcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debug_ndreset_gate` reader - "]
pub type DebugNdresetGateR = crate::BitReader;
#[doc = "Field `debug_ndreset_gate` writer - "]
pub type DebugNdresetGateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CpuRtcDivR {
        CpuRtcDivR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CpuRtcEnR {
        CpuRtcEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CpuRtcSelR {
        CpuRtcSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DebugNdresetGateR {
        DebugNdresetGateR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&mut self) -> CpuRtcDivW<'_, CpuClkCfgSpec> {
        CpuRtcDivW::new(self, 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&mut self) -> CpuRtcEnW<'_, CpuClkCfgSpec> {
        CpuRtcEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&mut self) -> CpuRtcSelW<'_, CpuClkCfgSpec> {
        CpuRtcSelW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&mut self) -> DebugNdresetGateW<'_, CpuClkCfgSpec> {
        DebugNdresetGateW::new(self, 20)
    }
}
#[doc = "CPU_CLK_CFG.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuClkCfgSpec;
impl crate::RegisterSpec for CpuClkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_clk_cfg::R`](R) reader structure"]
impl crate::Readable for CpuClkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_clk_cfg::W`](W) writer structure"]
impl crate::Writable for CpuClkCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CLK_CFG to value 0x0008_0010"]
impl crate::Resettable for CpuClkCfgSpec {
    const RESET_VALUE: u32 = 0x0008_0010;
}
