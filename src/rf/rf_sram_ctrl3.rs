#[doc = "Register `rf_sram_ctrl3` reader"]
pub type R = crate::R<RfSramCtrl3Spec>;
#[doc = "Register `rf_sram_ctrl3` writer"]
pub type W = crate::W<RfSramCtrl3Spec>;
#[doc = "Field `rf_sram_adc_sts` reader - "]
pub type RfSramAdcStsR = crate::FieldReader<u32>;
#[doc = "Field `rf_sram_adc_sts` writer - "]
pub type RfSramAdcStsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_adc_sts(&self) -> RfSramAdcStsR {
        RfSramAdcStsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_adc_sts(&mut self) -> RfSramAdcStsW<'_, RfSramCtrl3Spec> {
        RfSramAdcStsW::new(self, 0)
    }
}
#[doc = "rf_sram_ctrl3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl3Spec;
impl crate::RegisterSpec for RfSramCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl3::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl3::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl3 to value 0"]
impl crate::Resettable for RfSramCtrl3Spec {}
