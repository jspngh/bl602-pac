#[doc = "Register `tbb_gain_index2` reader"]
pub type R = crate::R<TbbGainIndex2Spec>;
#[doc = "Register `tbb_gain_index2` writer"]
pub type W = crate::W<TbbGainIndex2Spec>;
#[doc = "Field `gain_ctrl2_gc_tbb` reader - "]
pub type GainCtrl2GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_tbb` writer - "]
pub type GainCtrl2GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl2_gc_tmx` reader - "]
pub type GainCtrl2GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_tmx` writer - "]
pub type GainCtrl2GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl2_dac_bias_sel` reader - "]
pub type GainCtrl2DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_dac_bias_sel` writer - "]
pub type GainCtrl2DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl2_gc_tbb_boost` reader - "]
pub type GainCtrl2GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_tbb_boost` writer - "]
pub type GainCtrl2GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl3_gc_tbb` reader - "]
pub type GainCtrl3GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_tbb` writer - "]
pub type GainCtrl3GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl3_gc_tmx` reader - "]
pub type GainCtrl3GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_tmx` writer - "]
pub type GainCtrl3GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl3_dac_bias_sel` reader - "]
pub type GainCtrl3DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_dac_bias_sel` writer - "]
pub type GainCtrl3DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl3_gc_tbb_boost` reader - "]
pub type GainCtrl3GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_tbb_boost` writer - "]
pub type GainCtrl3GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb(&self) -> GainCtrl2GcTbbR {
        GainCtrl2GcTbbR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tmx(&self) -> GainCtrl2GcTmxR {
        GainCtrl2GcTmxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl2_dac_bias_sel(&self) -> GainCtrl2DacBiasSelR {
        GainCtrl2DacBiasSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb_boost(&self) -> GainCtrl2GcTbbBoostR {
        GainCtrl2GcTbbBoostR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb(&self) -> GainCtrl3GcTbbR {
        GainCtrl3GcTbbR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tmx(&self) -> GainCtrl3GcTmxR {
        GainCtrl3GcTmxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl3_dac_bias_sel(&self) -> GainCtrl3DacBiasSelR {
        GainCtrl3DacBiasSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb_boost(&self) -> GainCtrl3GcTbbBoostR {
        GainCtrl3GcTbbBoostR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb(&mut self) -> GainCtrl2GcTbbW<'_, TbbGainIndex2Spec> {
        GainCtrl2GcTbbW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tmx(&mut self) -> GainCtrl2GcTmxW<'_, TbbGainIndex2Spec> {
        GainCtrl2GcTmxW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl2_dac_bias_sel(&mut self) -> GainCtrl2DacBiasSelW<'_, TbbGainIndex2Spec> {
        GainCtrl2DacBiasSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb_boost(&mut self) -> GainCtrl2GcTbbBoostW<'_, TbbGainIndex2Spec> {
        GainCtrl2GcTbbBoostW::new(self, 14)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb(&mut self) -> GainCtrl3GcTbbW<'_, TbbGainIndex2Spec> {
        GainCtrl3GcTbbW::new(self, 16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tmx(&mut self) -> GainCtrl3GcTmxW<'_, TbbGainIndex2Spec> {
        GainCtrl3GcTmxW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl3_dac_bias_sel(&mut self) -> GainCtrl3DacBiasSelW<'_, TbbGainIndex2Spec> {
        GainCtrl3DacBiasSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb_boost(&mut self) -> GainCtrl3GcTbbBoostW<'_, TbbGainIndex2Spec> {
        GainCtrl3GcTbbBoostW::new(self, 30)
    }
}
#[doc = "tbb_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbGainIndex2Spec;
impl crate::RegisterSpec for TbbGainIndex2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbb_gain_index2::R`](R) reader structure"]
impl crate::Readable for TbbGainIndex2Spec {}
#[doc = "`write(|w| ..)` method takes [`tbb_gain_index2::W`](W) writer structure"]
impl crate::Writable for TbbGainIndex2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tbb_gain_index2 to value 0"]
impl crate::Resettable for TbbGainIndex2Spec {}
