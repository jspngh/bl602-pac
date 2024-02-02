#[doc = "Register `rf_sram_ctrl4` reader"]
pub type R = crate::R<RfSramCtrl4Spec>;
#[doc = "Register `rf_sram_ctrl4` writer"]
pub type W = crate::W<RfSramCtrl4Spec>;
#[doc = "Field `rf_sram_dac_done` reader - "]
pub type RfSramDacDoneR = crate::BitReader;
#[doc = "Field `rf_sram_dac_done` writer - "]
pub type RfSramDacDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_dac_en` reader - "]
pub type RfSramDacEnR = crate::BitReader;
#[doc = "Field `rf_sram_dac_en` writer - "]
pub type RfSramDacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_dac_loop_en` reader - "]
pub type RfSramDacLoopEnR = crate::BitReader;
#[doc = "Field `rf_sram_dac_loop_en` writer - "]
pub type RfSramDacLoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_dac_sts_clr` reader - "]
pub type RfSramDacStsClrR = crate::BitReader;
#[doc = "Field `rf_sram_dac_sts_clr` writer - "]
pub type RfSramDacStsClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_dac_done_cnt` reader - "]
pub type RfSramDacDoneCntR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_dac_done_cnt` writer - "]
pub type RfSramDacDoneCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_dac_done(&self) -> RfSramDacDoneR {
        RfSramDacDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_dac_en(&self) -> RfSramDacEnR {
        RfSramDacEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_dac_loop_en(&self) -> RfSramDacLoopEnR {
        RfSramDacLoopEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_dac_sts_clr(&self) -> RfSramDacStsClrR {
        RfSramDacStsClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_dac_done_cnt(&self) -> RfSramDacDoneCntR {
        RfSramDacDoneCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_dac_done(&mut self) -> RfSramDacDoneW<'_, RfSramCtrl4Spec> {
        RfSramDacDoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_dac_en(&mut self) -> RfSramDacEnW<'_, RfSramCtrl4Spec> {
        RfSramDacEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_dac_loop_en(&mut self) -> RfSramDacLoopEnW<'_, RfSramCtrl4Spec> {
        RfSramDacLoopEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_dac_sts_clr(&mut self) -> RfSramDacStsClrW<'_, RfSramCtrl4Spec> {
        RfSramDacStsClrW::new(self, 3)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_dac_done_cnt(&mut self) -> RfSramDacDoneCntW<'_, RfSramCtrl4Spec> {
        RfSramDacDoneCntW::new(self, 16)
    }
}
#[doc = "rf_sram_ctrl4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl4Spec;
impl crate::RegisterSpec for RfSramCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl4::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl4::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl4 to value 0"]
impl crate::Resettable for RfSramCtrl4Spec {}
