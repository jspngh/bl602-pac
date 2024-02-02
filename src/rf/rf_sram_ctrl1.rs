#[doc = "Register `rf_sram_ctrl1` reader"]
pub type R = crate::R<RfSramCtrl1Spec>;
#[doc = "Register `rf_sram_ctrl1` writer"]
pub type W = crate::W<RfSramCtrl1Spec>;
#[doc = "Field `rf_sram_adc_done` reader - "]
pub type RfSramAdcDoneR = crate::BitReader;
#[doc = "Field `rf_sram_adc_done` writer - "]
pub type RfSramAdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_adc_en` reader - "]
pub type RfSramAdcEnR = crate::BitReader;
#[doc = "Field `rf_sram_adc_en` writer - "]
pub type RfSramAdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_adc_loop_en` reader - "]
pub type RfSramAdcLoopEnR = crate::BitReader;
#[doc = "Field `rf_sram_adc_loop_en` writer - "]
pub type RfSramAdcLoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_adc_sts_clr` reader - "]
pub type RfSramAdcStsClrR = crate::BitReader;
#[doc = "Field `rf_sram_adc_sts_clr` writer - "]
pub type RfSramAdcStsClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_sram_adc_done_cnt` reader - "]
pub type RfSramAdcDoneCntR = crate::FieldReader<u16>;
#[doc = "Field `rf_sram_adc_done_cnt` writer - "]
pub type RfSramAdcDoneCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&self) -> RfSramAdcDoneR {
        RfSramAdcDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&self) -> RfSramAdcEnR {
        RfSramAdcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&self) -> RfSramAdcLoopEnR {
        RfSramAdcLoopEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&self) -> RfSramAdcStsClrR {
        RfSramAdcStsClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&self) -> RfSramAdcDoneCntR {
        RfSramAdcDoneCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&mut self) -> RfSramAdcDoneW<'_, RfSramCtrl1Spec> {
        RfSramAdcDoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&mut self) -> RfSramAdcEnW<'_, RfSramCtrl1Spec> {
        RfSramAdcEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&mut self) -> RfSramAdcLoopEnW<'_, RfSramCtrl1Spec> {
        RfSramAdcLoopEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&mut self) -> RfSramAdcStsClrW<'_, RfSramCtrl1Spec> {
        RfSramAdcStsClrW::new(self, 3)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&mut self) -> RfSramAdcDoneCntW<'_, RfSramCtrl1Spec> {
        RfSramAdcDoneCntW::new(self, 16)
    }
}
#[doc = "rf_sram_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSramCtrl1Spec;
impl crate::RegisterSpec for RfSramCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_sram_ctrl1::R`](R) reader structure"]
impl crate::Readable for RfSramCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_sram_ctrl1::W`](W) writer structure"]
impl crate::Writable for RfSramCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_sram_ctrl1 to value 0"]
impl crate::Resettable for RfSramCtrl1Spec {}
