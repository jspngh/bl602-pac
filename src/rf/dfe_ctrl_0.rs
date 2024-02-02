#[doc = "Register `dfe_ctrl_0` reader"]
pub type R = crate::R<DfeCtrl0Spec>;
#[doc = "Register `dfe_ctrl_0` writer"]
pub type W = crate::W<DfeCtrl0Spec>;
#[doc = "Field `tx_iqc_phase` reader - "]
pub type TxIqcPhaseR = crate::FieldReader<u16>;
#[doc = "Field `tx_iqc_phase` writer - "]
pub type TxIqcPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iqc_phase_en` reader - "]
pub type TxIqcPhaseEnR = crate::BitReader;
#[doc = "Field `tx_iqc_phase_en` writer - "]
pub type TxIqcPhaseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_iqc_gain` reader - "]
pub type TxIqcGainR = crate::FieldReader<u16>;
#[doc = "Field `tx_iqc_gain` writer - "]
pub type TxIqcGainW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `tx_iqc_gain_en` reader - "]
pub type TxIqcGainEnR = crate::BitReader;
#[doc = "Field `tx_iqc_gain_en` writer - "]
pub type TxIqcGainEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_dvga_gain_qdb` reader - "]
pub type TxDvgaGainQdbR = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb` writer - "]
pub type TxDvgaGainQdbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_ctrl_hw` reader - "]
pub type TxDvgaGainCtrlHwR = crate::BitReader;
#[doc = "Field `tx_dvga_gain_ctrl_hw` writer - "]
pub type TxDvgaGainCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&self) -> TxIqcPhaseR {
        TxIqcPhaseR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&self) -> TxIqcPhaseEnR {
        TxIqcPhaseEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&self) -> TxIqcGainR {
        TxIqcGainR::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&self) -> TxIqcGainEnR {
        TxIqcGainEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&self) -> TxDvgaGainQdbR {
        TxDvgaGainQdbR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&self) -> TxDvgaGainCtrlHwR {
        TxDvgaGainCtrlHwR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&mut self) -> TxIqcPhaseW<'_, DfeCtrl0Spec> {
        TxIqcPhaseW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&mut self) -> TxIqcPhaseEnW<'_, DfeCtrl0Spec> {
        TxIqcPhaseEnW::new(self, 10)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&mut self) -> TxIqcGainW<'_, DfeCtrl0Spec> {
        TxIqcGainW::new(self, 12)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&mut self) -> TxIqcGainEnW<'_, DfeCtrl0Spec> {
        TxIqcGainEnW::new(self, 23)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&mut self) -> TxDvgaGainQdbW<'_, DfeCtrl0Spec> {
        TxDvgaGainQdbW::new(self, 24)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&mut self) -> TxDvgaGainCtrlHwW<'_, DfeCtrl0Spec> {
        TxDvgaGainCtrlHwW::new(self, 31)
    }
}
#[doc = "dfe_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl0Spec;
impl crate::RegisterSpec for DfeCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_0::R`](R) reader structure"]
impl crate::Readable for DfeCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_0::W`](W) writer structure"]
impl crate::Writable for DfeCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_0 to value 0"]
impl crate::Resettable for DfeCtrl0Spec {}
