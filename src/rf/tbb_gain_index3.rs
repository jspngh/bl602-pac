#[doc = "Register `tbb_gain_index3` reader"]
pub type R = crate::R<TbbGainIndex3Spec>;
#[doc = "Register `tbb_gain_index3` writer"]
pub type W = crate::W<TbbGainIndex3Spec>;
#[doc = "Field `gain_ctrl4_gc_tbb` reader - "]
pub type GainCtrl4GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_tbb` writer - "]
pub type GainCtrl4GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl4_gc_tmx` reader - "]
pub type GainCtrl4GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_tmx` writer - "]
pub type GainCtrl4GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl4_dac_bias_sel` reader - "]
pub type GainCtrl4DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_dac_bias_sel` writer - "]
pub type GainCtrl4DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl4_gc_tbb_boost` reader - "]
pub type GainCtrl4GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_tbb_boost` writer - "]
pub type GainCtrl4GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl5_gc_tbb` reader - "]
pub type GainCtrl5GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_tbb` writer - "]
pub type GainCtrl5GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl5_gc_tmx` reader - "]
pub type GainCtrl5GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_tmx` writer - "]
pub type GainCtrl5GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl5_dac_bias_sel` reader - "]
pub type GainCtrl5DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_dac_bias_sel` writer - "]
pub type GainCtrl5DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl5_gc_tbb_boost` reader - "]
pub type GainCtrl5GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_tbb_boost` writer - "]
pub type GainCtrl5GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tbb(&self) -> GainCtrl4GcTbbR {
        GainCtrl4GcTbbR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tmx(&self) -> GainCtrl4GcTmxR {
        GainCtrl4GcTmxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl4_dac_bias_sel(&self) -> GainCtrl4DacBiasSelR {
        GainCtrl4DacBiasSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tbb_boost(&self) -> GainCtrl4GcTbbBoostR {
        GainCtrl4GcTbbBoostR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tbb(&self) -> GainCtrl5GcTbbR {
        GainCtrl5GcTbbR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tmx(&self) -> GainCtrl5GcTmxR {
        GainCtrl5GcTmxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl5_dac_bias_sel(&self) -> GainCtrl5DacBiasSelR {
        GainCtrl5DacBiasSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tbb_boost(&self) -> GainCtrl5GcTbbBoostR {
        GainCtrl5GcTbbBoostR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tbb(&mut self) -> GainCtrl4GcTbbW<'_, TbbGainIndex3Spec> {
        GainCtrl4GcTbbW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tmx(&mut self) -> GainCtrl4GcTmxW<'_, TbbGainIndex3Spec> {
        GainCtrl4GcTmxW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl4_dac_bias_sel(&mut self) -> GainCtrl4DacBiasSelW<'_, TbbGainIndex3Spec> {
        GainCtrl4DacBiasSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_tbb_boost(&mut self) -> GainCtrl4GcTbbBoostW<'_, TbbGainIndex3Spec> {
        GainCtrl4GcTbbBoostW::new(self, 14)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tbb(&mut self) -> GainCtrl5GcTbbW<'_, TbbGainIndex3Spec> {
        GainCtrl5GcTbbW::new(self, 16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tmx(&mut self) -> GainCtrl5GcTmxW<'_, TbbGainIndex3Spec> {
        GainCtrl5GcTmxW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl5_dac_bias_sel(&mut self) -> GainCtrl5DacBiasSelW<'_, TbbGainIndex3Spec> {
        GainCtrl5DacBiasSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_tbb_boost(&mut self) -> GainCtrl5GcTbbBoostW<'_, TbbGainIndex3Spec> {
        GainCtrl5GcTbbBoostW::new(self, 30)
    }
}
#[doc = "tbb_gain_index3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbGainIndex3Spec;
impl crate::RegisterSpec for TbbGainIndex3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbb_gain_index3::R`](R) reader structure"]
impl crate::Readable for TbbGainIndex3Spec {}
#[doc = "`write(|w| ..)` method takes [`tbb_gain_index3::W`](W) writer structure"]
impl crate::Writable for TbbGainIndex3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tbb_gain_index3 to value 0"]
impl crate::Resettable for TbbGainIndex3Spec {}
