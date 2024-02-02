#[doc = "Register `rf_sram_ctrl2` reader"]
pub type R = crate::R<RfSramCtrl2Spec>;
#[doc = "Register `rf_sram_ctrl2` writer"]
pub type W = crate::W<RfSramCtrl2Spec>;
#[doc = "Field `rf_sram_adc_addr_end` reader - "]
pub type RfSramAdcAddrEndR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_adc_addr_end` writer - "]
pub type RfSramAdcAddrEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rf_sram_adc_addr_start` reader - "]
pub type RfSramAdcAddrStartR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_adc_addr_start` writer - "]
pub type RfSramAdcAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_end(&self) -> RfSramAdcAddrEndR {
        RfSramAdcAddrEndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_start(&self) -> RfSramAdcAddrStartR {
        RfSramAdcAddrStartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_end(&mut self) -> RfSramAdcAddrEndW<'_, RfSramCtrl2Spec> {
        RfSramAdcAddrEndW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_start(&mut self) -> RfSramAdcAddrStartW<'_, RfSramCtrl2Spec> {
        RfSramAdcAddrStartW::new(self, 16)
    }
}
#[doc = "rf_sram_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl2Spec;
impl crate::RegisterSpec for RfSramCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl2::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl2::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl2 to value 0"]
impl crate::Resettable for RfSramCtrl2Spec {}
