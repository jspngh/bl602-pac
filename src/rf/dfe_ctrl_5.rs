#[doc = "Register `dfe_ctrl_5` reader"]
pub type R = crate::R<DfeCtrl5Spec>;
#[doc = "Register `dfe_ctrl_5` writer"]
pub type W = crate::W<DfeCtrl5Spec>;
#[doc = "Field `rx_iqc_phase` reader - "]
pub type RxIqcPhaseR = crate::FieldReader<u16>;
#[doc = "Field `rx_iqc_phase` writer - "]
pub type RxIqcPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_iqc_phase_en` reader - "]
pub type RxIqcPhaseEnR = crate::BitReader;
#[doc = "Field `rx_iqc_phase_en` writer - "]
pub type RxIqcPhaseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_iqc_gain` reader - "]
pub type RxIqcGainR = crate::FieldReader<u16>;
#[doc = "Field `rx_iqc_gain` writer - "]
pub type RxIqcGainW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `rx_iqc_gain_en` reader - "]
pub type RxIqcGainEnR = crate::BitReader;
#[doc = "Field `rx_iqc_gain_en` writer - "]
pub type RxIqcGainEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iqc_phase(&self) -> RxIqcPhaseR {
        RxIqcPhaseR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_iqc_phase_en(&self) -> RxIqcPhaseEnR {
        RxIqcPhaseEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn rx_iqc_gain(&self) -> RxIqcGainR {
        RxIqcGainR::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_iqc_gain_en(&self) -> RxIqcGainEnR {
        RxIqcGainEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iqc_phase(&mut self) -> RxIqcPhaseW<'_, DfeCtrl5Spec> {
        RxIqcPhaseW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_iqc_phase_en(&mut self) -> RxIqcPhaseEnW<'_, DfeCtrl5Spec> {
        RxIqcPhaseEnW::new(self, 10)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn rx_iqc_gain(&mut self) -> RxIqcGainW<'_, DfeCtrl5Spec> {
        RxIqcGainW::new(self, 12)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_iqc_gain_en(&mut self) -> RxIqcGainEnW<'_, DfeCtrl5Spec> {
        RxIqcGainEnW::new(self, 23)
    }
}
#[doc = "dfe_ctrl_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl5Spec;
impl crate::RegisterSpec for DfeCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_5::R`](R) reader structure"]
impl crate::Readable for DfeCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_5::W`](W) writer structure"]
impl crate::Writable for DfeCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_5 to value 0"]
impl crate::Resettable for DfeCtrl5Spec {}
