#[doc = "Register `rf_base_ctrl1` reader"]
pub type R = crate::R<RfBaseCtrl1Spec>;
#[doc = "Register `rf_base_ctrl1` writer"]
pub type W = crate::W<RfBaseCtrl1Spec>;
#[doc = "Field `aupll_sdm_rst_dly` reader - "]
pub type AupllSdmRstDlyR = crate::FieldReader;
#[doc = "Field `aupll_sdm_rst_dly` writer - "]
pub type AupllSdmRstDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_rst_dly` reader - "]
pub type LoSdmRstDlyR = crate::FieldReader;
#[doc = "Field `lo_sdm_rst_dly` writer - "]
pub type LoSdmRstDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppu_lead` reader - "]
pub type PpuLeadR = crate::FieldReader;
#[doc = "Field `ppu_lead` writer - "]
pub type PpuLeadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pud_vco_dly` reader - "]
pub type PudVcoDlyR = crate::FieldReader;
#[doc = "Field `pud_vco_dly` writer - "]
pub type PudVcoDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pud_iref_dly` reader - "]
pub type PudIrefDlyR = crate::FieldReader;
#[doc = "Field `pud_iref_dly` writer - "]
pub type PudIrefDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pud_pa_dly` reader - "]
pub type PudPaDlyR = crate::FieldReader;
#[doc = "Field `pud_pa_dly` writer - "]
pub type PudPaDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mbg_trim` reader - "]
pub type MbgTrimR = crate::FieldReader;
#[doc = "Field `mbg_trim` writer - "]
pub type MbgTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&self) -> AupllSdmRstDlyR {
        AupllSdmRstDlyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&self) -> LoSdmRstDlyR {
        LoSdmRstDlyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&self) -> PpuLeadR {
        PpuLeadR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&self) -> PudVcoDlyR {
        PudVcoDlyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&self) -> PudIrefDlyR {
        PudIrefDlyR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&self) -> PudPaDlyR {
        PudPaDlyR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&self) -> MbgTrimR {
        MbgTrimR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&mut self) -> AupllSdmRstDlyW<'_, RfBaseCtrl1Spec> {
        AupllSdmRstDlyW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&mut self) -> LoSdmRstDlyW<'_, RfBaseCtrl1Spec> {
        LoSdmRstDlyW::new(self, 2)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&mut self) -> PpuLeadW<'_, RfBaseCtrl1Spec> {
        PpuLeadW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&mut self) -> PudVcoDlyW<'_, RfBaseCtrl1Spec> {
        PudVcoDlyW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&mut self) -> PudIrefDlyW<'_, RfBaseCtrl1Spec> {
        PudIrefDlyW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&mut self) -> PudPaDlyW<'_, RfBaseCtrl1Spec> {
        PudPaDlyW::new(self, 14)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&mut self) -> MbgTrimW<'_, RfBaseCtrl1Spec> {
        MbgTrimW::new(self, 27)
    }
}
#[doc = "ZRF Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_base_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_base_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfBaseCtrl1Spec;
impl crate::RegisterSpec for RfBaseCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_base_ctrl1::R`](R) reader structure"]
impl crate::Readable for RfBaseCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_base_ctrl1::W`](W) writer structure"]
impl crate::Writable for RfBaseCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_base_ctrl1 to value 0"]
impl crate::Resettable for RfBaseCtrl1Spec {}
