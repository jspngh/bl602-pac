#[doc = "Register `tbb_gain_index4` reader"]
pub type R = crate::R<TbbGainIndex4Spec>;
#[doc = "Register `tbb_gain_index4` writer"]
pub type W = crate::W<TbbGainIndex4Spec>;
#[doc = "Field `gain_ctrl6_gc_tbb` reader - "]
pub type GainCtrl6GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_tbb` writer - "]
pub type GainCtrl6GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl6_gc_tmx` reader - "]
pub type GainCtrl6GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_tmx` writer - "]
pub type GainCtrl6GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl6_dac_bias_sel` reader - "]
pub type GainCtrl6DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_dac_bias_sel` writer - "]
pub type GainCtrl6DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl6_gc_tbb_boost` reader - "]
pub type GainCtrl6GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_tbb_boost` writer - "]
pub type GainCtrl6GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl7_gc_tbb` reader - "]
pub type GainCtrl7GcTbbR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_tbb` writer - "]
pub type GainCtrl7GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gain_ctrl7_gc_tmx` reader - "]
pub type GainCtrl7GcTmxR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_tmx` writer - "]
pub type GainCtrl7GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl7_dac_bias_sel` reader - "]
pub type GainCtrl7DacBiasSelR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_dac_bias_sel` writer - "]
pub type GainCtrl7DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl7_gc_tbb_boost` reader - "]
pub type GainCtrl7GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_tbb_boost` writer - "]
pub type GainCtrl7GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tbb(&self) -> GainCtrl6GcTbbR {
        GainCtrl6GcTbbR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tmx(&self) -> GainCtrl6GcTmxR {
        GainCtrl6GcTmxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl6_dac_bias_sel(&self) -> GainCtrl6DacBiasSelR {
        GainCtrl6DacBiasSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tbb_boost(&self) -> GainCtrl6GcTbbBoostR {
        GainCtrl6GcTbbBoostR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tbb(&self) -> GainCtrl7GcTbbR {
        GainCtrl7GcTbbR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tmx(&self) -> GainCtrl7GcTmxR {
        GainCtrl7GcTmxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl7_dac_bias_sel(&self) -> GainCtrl7DacBiasSelR {
        GainCtrl7DacBiasSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tbb_boost(&self) -> GainCtrl7GcTbbBoostR {
        GainCtrl7GcTbbBoostR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tbb(&mut self) -> GainCtrl6GcTbbW<'_, TbbGainIndex4Spec> {
        GainCtrl6GcTbbW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tmx(&mut self) -> GainCtrl6GcTmxW<'_, TbbGainIndex4Spec> {
        GainCtrl6GcTmxW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl6_dac_bias_sel(&mut self) -> GainCtrl6DacBiasSelW<'_, TbbGainIndex4Spec> {
        GainCtrl6DacBiasSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_tbb_boost(&mut self) -> GainCtrl6GcTbbBoostW<'_, TbbGainIndex4Spec> {
        GainCtrl6GcTbbBoostW::new(self, 14)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tbb(&mut self) -> GainCtrl7GcTbbW<'_, TbbGainIndex4Spec> {
        GainCtrl7GcTbbW::new(self, 16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tmx(&mut self) -> GainCtrl7GcTmxW<'_, TbbGainIndex4Spec> {
        GainCtrl7GcTmxW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl7_dac_bias_sel(&mut self) -> GainCtrl7DacBiasSelW<'_, TbbGainIndex4Spec> {
        GainCtrl7DacBiasSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_tbb_boost(&mut self) -> GainCtrl7GcTbbBoostW<'_, TbbGainIndex4Spec> {
        GainCtrl7GcTbbBoostW::new(self, 30)
    }
}
#[doc = "tbb_gain_index4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbGainIndex4Spec;
impl crate::RegisterSpec for TbbGainIndex4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbb_gain_index4::R`](R) reader structure"]
impl crate::Readable for TbbGainIndex4Spec {}
#[doc = "`write(|w| ..)` method takes [`tbb_gain_index4::W`](W) writer structure"]
impl crate::Writable for TbbGainIndex4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tbb_gain_index4 to value 0"]
impl crate::Resettable for TbbGainIndex4Spec {}
