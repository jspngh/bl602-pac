#[doc = "Register `PDS_CTL2` reader"]
pub type R = crate::R<PdsCtl2Spec>;
#[doc = "Register `PDS_CTL2` writer"]
pub type W = crate::W<PdsCtl2Spec>;
#[doc = "Field `cr_pds_force_np_pwr_off` reader - "]
pub type CrPdsForceNpPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_pwr_off` writer - "]
pub type CrPdsForceNpPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_pwr_off` reader - "]
pub type CrPdsForceWbPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_pwr_off` writer - "]
pub type CrPdsForceWbPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_iso_en` reader - "]
pub type CrPdsForceNpIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_iso_en` writer - "]
pub type CrPdsForceNpIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_iso_en` reader - "]
pub type CrPdsForceWbIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_iso_en` writer - "]
pub type CrPdsForceWbIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_pds_rst` reader - "]
pub type CrPdsForceNpPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_pds_rst` writer - "]
pub type CrPdsForceNpPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_pds_rst` reader - "]
pub type CrPdsForceWbPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_pds_rst` writer - "]
pub type CrPdsForceWbPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_mem_stby` reader - "]
pub type CrPdsForceNpMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_mem_stby` writer - "]
pub type CrPdsForceNpMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_mem_stby` reader - "]
pub type CrPdsForceWbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_mem_stby` writer - "]
pub type CrPdsForceWbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_gate_clk` reader - "]
pub type CrPdsForceNpGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_gate_clk` writer - "]
pub type CrPdsForceNpGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_gate_clk` reader - "]
pub type CrPdsForceWbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_gate_clk` writer - "]
pub type CrPdsForceWbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&self) -> CrPdsForceNpPwrOffR {
        CrPdsForceNpPwrOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&self) -> CrPdsForceWbPwrOffR {
        CrPdsForceWbPwrOffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&self) -> CrPdsForceNpIsoEnR {
        CrPdsForceNpIsoEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&self) -> CrPdsForceWbIsoEnR {
        CrPdsForceWbIsoEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CrPdsForceNpPdsRstR {
        CrPdsForceNpPdsRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CrPdsForceWbPdsRstR {
        CrPdsForceWbPdsRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CrPdsForceNpMemStbyR {
        CrPdsForceNpMemStbyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CrPdsForceWbMemStbyR {
        CrPdsForceWbMemStbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CrPdsForceNpGateClkR {
        CrPdsForceNpGateClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CrPdsForceWbGateClkR {
        CrPdsForceWbGateClkR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&mut self) -> CrPdsForceNpPwrOffW<'_, PdsCtl2Spec> {
        CrPdsForceNpPwrOffW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&mut self) -> CrPdsForceWbPwrOffW<'_, PdsCtl2Spec> {
        CrPdsForceWbPwrOffW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&mut self) -> CrPdsForceNpIsoEnW<'_, PdsCtl2Spec> {
        CrPdsForceNpIsoEnW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&mut self) -> CrPdsForceWbIsoEnW<'_, PdsCtl2Spec> {
        CrPdsForceWbIsoEnW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CrPdsForceNpPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceNpPdsRstW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CrPdsForceWbPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceWbPdsRstW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CrPdsForceNpMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceNpMemStbyW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CrPdsForceWbMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceWbMemStbyW::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CrPdsForceNpGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceNpGateClkW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CrPdsForceWbGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceWbGateClkW::new(self, 18)
    }
}
#[doc = "PDS_CTL2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl2Spec;
impl crate::RegisterSpec for PdsCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl2::R`](R) reader structure"]
impl crate::Readable for PdsCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl2::W`](W) writer structure"]
impl crate::Writable for PdsCtl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL2 to value 0"]
impl crate::Resettable for PdsCtl2Spec {}
