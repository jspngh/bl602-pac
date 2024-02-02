#[doc = "Register `rf_sram_ctrl0` reader"]
pub type R = crate::R<RfSramCtrl0Spec>;
#[doc = "Register `rf_sram_ctrl0` writer"]
pub type W = crate::W<RfSramCtrl0Spec>;
#[doc = "Field `rf_sram_link_dly` reader - "]
pub type RfSramLinkDlyR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_link_dly` writer - "]
pub type RfSramLinkDlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rf_sram_link_mode` reader - "]
pub type RfSramLinkModeR = crate::FieldReader;
#[doc = "Field `rf_sram_link_mode` writer - "]
pub type RfSramLinkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rf_sram_swap` reader - "]
pub type RfSramSwapR = crate::BitReader;
#[doc = "Field `rf_sram_swap` writer - "]
pub type RfSramSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_ext_clr` reader - "]
pub type RfSramExtClrR = crate::BitReader;
#[doc = "Field `rf_sram_ext_clr` writer - "]
pub type RfSramExtClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_link_dly(&self) -> RfSramLinkDlyR {
        RfSramLinkDlyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rf_sram_link_mode(&self) -> RfSramLinkModeR {
        RfSramLinkModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_sram_swap(&self) -> RfSramSwapR {
        RfSramSwapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_sram_ext_clr(&self) -> RfSramExtClrR {
        RfSramExtClrR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_link_dly(&mut self) -> RfSramLinkDlyW<'_, RfSramCtrl0Spec> {
        RfSramLinkDlyW::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rf_sram_link_mode(&mut self) -> RfSramLinkModeW<'_, RfSramCtrl0Spec> {
        RfSramLinkModeW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_sram_swap(&mut self) -> RfSramSwapW<'_, RfSramCtrl0Spec> {
        RfSramSwapW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_sram_ext_clr(&mut self) -> RfSramExtClrW<'_, RfSramCtrl0Spec> {
        RfSramExtClrW::new(self, 19)
    }
}
#[doc = "rf_sram_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl0Spec;
impl crate::RegisterSpec for RfSramCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl0::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl0::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl0 to value 0"]
impl crate::Resettable for RfSramCtrl0Spec {}
