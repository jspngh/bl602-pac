#[doc = "Register `tbb_gain_index1` reader"]
pub type R = crate::R<TbbGainIndex1Spec>;
#[doc = "Register `tbb_gain_index1` writer"]
pub type W = crate::W<TbbGainIndex1Spec>;
#[doc = "Field `gain_ctrl0_gc_tbb` reader - "]
pub type GainCtrl0GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_tbb` writer - "]
pub type GainCtrl0GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl0_gc_tmx` reader - "]
pub type GainCtrl0GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_tmx` writer - "]
pub type GainCtrl0GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl0_dac_bias_sel` reader - "]
pub type GainCtrl0DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_dac_bias_sel` writer - "]
pub type GainCtrl0DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl0_gc_tbb_boost` reader - "]
pub type GainCtrl0GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_tbb_boost` writer - "]
pub type GainCtrl0GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl1_gc_tbb` reader - "]
pub type GainCtrl1GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_tbb` writer - "]
pub type GainCtrl1GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl1_gc_tmx` reader - "]
pub type GainCtrl1GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_tmx` writer - "]
pub type GainCtrl1GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl1_dac_bias_sel` reader - "]
pub type GainCtrl1DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_dac_bias_sel` writer - "]
pub type GainCtrl1DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl1_gc_tbb_boost` reader - "]
pub type GainCtrl1GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_tbb_boost` writer - "]
pub type GainCtrl1GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&self) -> GainCtrl0GcTbbR {
        GainCtrl0GcTbbR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&self) -> GainCtrl0GcTmxR {
        GainCtrl0GcTmxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&self) -> GainCtrl0DacBiasSelR {
        GainCtrl0DacBiasSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&self) -> GainCtrl0GcTbbBoostR {
        GainCtrl0GcTbbBoostR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&self) -> GainCtrl1GcTbbR {
        GainCtrl1GcTbbR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&self) -> GainCtrl1GcTmxR {
        GainCtrl1GcTmxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&self) -> GainCtrl1DacBiasSelR {
        GainCtrl1DacBiasSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&self) -> GainCtrl1GcTbbBoostR {
        GainCtrl1GcTbbBoostR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&mut self) -> GainCtrl0GcTbbW<'_, TbbGainIndex1Spec> {
        GainCtrl0GcTbbW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&mut self) -> GainCtrl0GcTmxW<'_, TbbGainIndex1Spec> {
        GainCtrl0GcTmxW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&mut self) -> GainCtrl0DacBiasSelW<'_, TbbGainIndex1Spec> {
        GainCtrl0DacBiasSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&mut self) -> GainCtrl0GcTbbBoostW<'_, TbbGainIndex1Spec> {
        GainCtrl0GcTbbBoostW::new(self, 14)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&mut self) -> GainCtrl1GcTbbW<'_, TbbGainIndex1Spec> {
        GainCtrl1GcTbbW::new(self, 16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&mut self) -> GainCtrl1GcTmxW<'_, TbbGainIndex1Spec> {
        GainCtrl1GcTmxW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&mut self) -> GainCtrl1DacBiasSelW<'_, TbbGainIndex1Spec> {
        GainCtrl1DacBiasSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&mut self) -> GainCtrl1GcTbbBoostW<'_, TbbGainIndex1Spec> {
        GainCtrl1GcTbbBoostW::new(self, 30)
    }
}
#[doc = "tbb_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbGainIndex1Spec;
impl crate::RegisterSpec for TbbGainIndex1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbb_gain_index1::R`](R) reader structure"]
impl crate::Readable for TbbGainIndex1Spec {}
#[doc = "`write(|w| ..)` method takes [`tbb_gain_index1::W`](W) writer structure"]
impl crate::Writable for TbbGainIndex1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tbb_gain_index1 to value 0"]
impl crate::Resettable for TbbGainIndex1Spec {}
