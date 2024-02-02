#[doc = "Register `rf_sram_ctrl6` reader"]
pub type R = crate::R<RfSramCtrl6Spec>;
#[doc = "Register `rf_sram_ctrl6` writer"]
pub type W = crate::W<RfSramCtrl6Spec>;
#[doc = "Field `rf_sram_dac_sts` reader - "]
pub type RfSramDacStsR = crate::FieldReader<u32>;
#[doc = "Field `rf_sram_dac_sts` writer - "]
pub type RfSramDacStsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_dac_sts(&self) -> RfSramDacStsR {
        RfSramDacStsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_dac_sts(&mut self) -> RfSramDacStsW<'_, RfSramCtrl6Spec> {
        RfSramDacStsW::new(self, 0)
    }
}
#[doc = "rf_sram_ctrl6.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl6Spec;
impl crate::RegisterSpec for RfSramCtrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl6::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl6::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl6 to value 0"]
impl crate::Resettable for RfSramCtrl6Spec {}
