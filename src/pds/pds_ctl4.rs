#[doc = "Register `PDS_CTL4` reader"]
pub type R = crate::R<PdsCtl4Spec>;
#[doc = "Register `PDS_CTL4` writer"]
pub type W = crate::W<PdsCtl4Spec>;
#[doc = "Field `cr_pds_np_pwr_off` reader - "]
pub type CrPdsNpPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_np_pwr_off` writer - "]
pub type CrPdsNpPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_reset` reader - "]
pub type CrPdsNpResetR = crate::BitReader;
#[doc = "Field `cr_pds_np_reset` writer - "]
pub type CrPdsNpResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_mem_stby` reader - "]
pub type CrPdsNpMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_np_mem_stby` writer - "]
pub type CrPdsNpMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_gate_clk` reader - "]
pub type CrPdsNpGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_np_gate_clk` writer - "]
pub type CrPdsNpGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_pwr_off` reader - "]
pub type CrPdsWbPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_wb_pwr_off` writer - "]
pub type CrPdsWbPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_reset` reader - "]
pub type CrPdsWbResetR = crate::BitReader;
#[doc = "Field `cr_pds_wb_reset` writer - "]
pub type CrPdsWbResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_mem_stby` reader - "]
pub type CrPdsWbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_wb_mem_stby` writer - "]
pub type CrPdsWbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_gate_clk` reader - "]
pub type CrPdsWbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_wb_gate_clk` writer - "]
pub type CrPdsWbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_pwr_off` reader - "]
pub type CrPdsMiscPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_misc_pwr_off` writer - "]
pub type CrPdsMiscPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_reset` reader - "]
pub type CrPdsMiscResetR = crate::BitReader;
#[doc = "Field `cr_pds_misc_reset` writer - "]
pub type CrPdsMiscResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_mem_stby` reader - "]
pub type CrPdsMiscMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_misc_mem_stby` writer - "]
pub type CrPdsMiscMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_gate_clk` reader - "]
pub type CrPdsMiscGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_misc_gate_clk` writer - "]
pub type CrPdsMiscGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&self) -> CrPdsNpPwrOffR {
        CrPdsNpPwrOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CrPdsNpResetR {
        CrPdsNpResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CrPdsNpMemStbyR {
        CrPdsNpMemStbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CrPdsNpGateClkR {
        CrPdsNpGateClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&self) -> CrPdsWbPwrOffR {
        CrPdsWbPwrOffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&self) -> CrPdsWbResetR {
        CrPdsWbResetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&self) -> CrPdsWbMemStbyR {
        CrPdsWbMemStbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&self) -> CrPdsWbGateClkR {
        CrPdsWbGateClkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CrPdsMiscPwrOffR {
        CrPdsMiscPwrOffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CrPdsMiscResetR {
        CrPdsMiscResetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CrPdsMiscMemStbyR {
        CrPdsMiscMemStbyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CrPdsMiscGateClkR {
        CrPdsMiscGateClkR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&mut self) -> CrPdsNpPwrOffW<'_, PdsCtl4Spec> {
        CrPdsNpPwrOffW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&mut self) -> CrPdsNpResetW<'_, PdsCtl4Spec> {
        CrPdsNpResetW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&mut self) -> CrPdsNpMemStbyW<'_, PdsCtl4Spec> {
        CrPdsNpMemStbyW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&mut self) -> CrPdsNpGateClkW<'_, PdsCtl4Spec> {
        CrPdsNpGateClkW::new(self, 3)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&mut self) -> CrPdsWbPwrOffW<'_, PdsCtl4Spec> {
        CrPdsWbPwrOffW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&mut self) -> CrPdsWbResetW<'_, PdsCtl4Spec> {
        CrPdsWbResetW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&mut self) -> CrPdsWbMemStbyW<'_, PdsCtl4Spec> {
        CrPdsWbMemStbyW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&mut self) -> CrPdsWbGateClkW<'_, PdsCtl4Spec> {
        CrPdsWbGateClkW::new(self, 15)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CrPdsMiscPwrOffW<'_, PdsCtl4Spec> {
        CrPdsMiscPwrOffW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&mut self) -> CrPdsMiscResetW<'_, PdsCtl4Spec> {
        CrPdsMiscResetW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CrPdsMiscMemStbyW<'_, PdsCtl4Spec> {
        CrPdsMiscMemStbyW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CrPdsMiscGateClkW<'_, PdsCtl4Spec> {
        CrPdsMiscGateClkW::new(self, 27)
    }
}
#[doc = "PDS_CTL4.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl4Spec;
impl crate::RegisterSpec for PdsCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl4::R`](R) reader structure"]
impl crate::Readable for PdsCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl4::W`](W) writer structure"]
impl crate::Writable for PdsCtl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL4 to value 0x0f00_f00f"]
impl crate::Resettable for PdsCtl4Spec {
    const RESET_VALUE: u32 = 0x0f00_f00f;
}
