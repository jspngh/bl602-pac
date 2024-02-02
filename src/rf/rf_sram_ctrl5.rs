#[doc = "Register `rf_sram_ctrl5` reader"]
pub type R = crate::R<RfSramCtrl5Spec>;
#[doc = "Register `rf_sram_ctrl5` writer"]
pub type W = crate::W<RfSramCtrl5Spec>;
#[doc = "Field `rf_sram_dac_addr_end` reader - "]
pub type RfSramDacAddrEndR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_dac_addr_end` writer - "]
pub type RfSramDacAddrEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rf_sram_dac_addr_start` reader - "]
pub type RfSramDacAddrStartR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_dac_addr_start` writer - "]
pub type RfSramDacAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_dac_addr_end(&self) -> RfSramDacAddrEndR {
        RfSramDacAddrEndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_dac_addr_start(&self) -> RfSramDacAddrStartR {
        RfSramDacAddrStartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_dac_addr_end(&mut self) -> RfSramDacAddrEndW<'_, RfSramCtrl5Spec> {
        RfSramDacAddrEndW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_dac_addr_start(&mut self) -> RfSramDacAddrStartW<'_, RfSramCtrl5Spec> {
        RfSramDacAddrStartW::new(self, 16)
    }
}
#[doc = "rf_sram_ctrl5.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl5Spec;
impl crate::RegisterSpec for RfSramCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl5::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl5::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl5 to value 0"]
impl crate::Resettable for RfSramCtrl5Spec {}
