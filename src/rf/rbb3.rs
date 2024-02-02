#[doc = "Register `rbb3` reader"]
pub type R = crate::R<Rbb3Spec>;
#[doc = "Register `rbb3` writer"]
pub type W = crate::W<Rbb3Spec>;
#[doc = "Field `rbb_bt_mode_hw` reader - "]
pub type RbbBtModeHwR = crate::BitReader;
#[doc = "Field `rbb_bt_mode_hw` writer - "]
pub type RbbBtModeHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_bt_mode` reader - "]
pub type RbbBtModeR = crate::BitReader;
#[doc = "Field `rbb_bt_mode` writer - "]
pub type RbbBtModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_bt_fif_tune` reader - "]
pub type RbbBtFifTuneR = crate::FieldReader;
#[doc = "Field `rbb_bt_fif_tune` writer - "]
pub type RbbBtFifTuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rbb_deq` reader - "]
pub type RbbDeqR = crate::FieldReader;
#[doc = "Field `rbb_deq` writer - "]
pub type RbbDeqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rbb_bm_op` reader - "]
pub type RbbBmOpR = crate::FieldReader;
#[doc = "Field `rbb_bm_op` writer - "]
pub type RbbBmOpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rbb_vcm` reader - "]
pub type RbbVcmR = crate::FieldReader;
#[doc = "Field `rbb_vcm` writer - "]
pub type RbbVcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rbb_bq_iqbias_short` reader - "]
pub type RbbBqIqbiasShortR = crate::BitReader;
#[doc = "Field `rbb_bq_iqbias_short` writer - "]
pub type RbbBqIqbiasShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_tia_iqbias_short` reader - "]
pub type RbbTiaIqbiasShortR = crate::BitReader;
#[doc = "Field `rbb_tia_iqbias_short` writer - "]
pub type RbbTiaIqbiasShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_bw` reader - "]
pub type RbbBwR = crate::FieldReader;
#[doc = "Field `rbb_bw` writer - "]
pub type RbbBwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rxiqcal_en` reader - "]
pub type RxiqcalEnR = crate::BitReader;
#[doc = "Field `rxiqcal_en` writer - "]
pub type RxiqcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwr_det_en` reader - "]
pub type PwrDetEnR = crate::BitReader;
#[doc = "Field `pwr_det_en` writer - "]
pub type PwrDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&self) -> RbbBtModeHwR {
        RbbBtModeHwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&self) -> RbbBtModeR {
        RbbBtModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&self) -> RbbBtFifTuneR {
        RbbBtFifTuneR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RbbDeqR {
        RbbDeqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RbbBmOpR {
        RbbBmOpR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RbbVcmR {
        RbbVcmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&self) -> RbbBqIqbiasShortR {
        RbbBqIqbiasShortR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&self) -> RbbTiaIqbiasShortR {
        RbbTiaIqbiasShortR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RbbBwR {
        RbbBwR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&self) -> RxiqcalEnR {
        RxiqcalEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&self) -> PwrDetEnR {
        PwrDetEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&mut self) -> RbbBtModeHwW<'_, Rbb3Spec> {
        RbbBtModeHwW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&mut self) -> RbbBtModeW<'_, Rbb3Spec> {
        RbbBtModeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&mut self) -> RbbBtFifTuneW<'_, Rbb3Spec> {
        RbbBtFifTuneW::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&mut self) -> RbbDeqW<'_, Rbb3Spec> {
        RbbDeqW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&mut self) -> RbbBmOpW<'_, Rbb3Spec> {
        RbbBmOpW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&mut self) -> RbbVcmW<'_, Rbb3Spec> {
        RbbVcmW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&mut self) -> RbbBqIqbiasShortW<'_, Rbb3Spec> {
        RbbBqIqbiasShortW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&mut self) -> RbbTiaIqbiasShortW<'_, Rbb3Spec> {
        RbbTiaIqbiasShortW::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&mut self) -> RbbBwW<'_, Rbb3Spec> {
        RbbBwW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&mut self) -> RxiqcalEnW<'_, Rbb3Spec> {
        RxiqcalEnW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&mut self) -> PwrDetEnW<'_, Rbb3Spec> {
        PwrDetEnW::new(self, 31)
    }
}
#[doc = "rbb3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbb3Spec;
impl crate::RegisterSpec for Rbb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb3::R`](R) reader structure"]
impl crate::Readable for Rbb3Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb3::W`](W) writer structure"]
impl crate::Writable for Rbb3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb3 to value 0"]
impl crate::Resettable for Rbb3Spec {}
