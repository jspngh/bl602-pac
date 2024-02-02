#[doc = "Register `trx_gain_hw` reader"]
pub type R = crate::R<TrxGainHwSpec>;
#[doc = "Register `trx_gain_hw` writer"]
pub type W = crate::W<TrxGainHwSpec>;
#[doc = "Field `gc_lna_hw` reader - "]
pub type GcLnaHwR = crate::FieldReader;
#[doc = "Field `gc_lna_hw` writer - "]
pub type GcLnaHwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_rmxgm_hw` reader - "]
pub type GcRmxgmHwR = crate::FieldReader;
#[doc = "Field `gc_rmxgm_hw` writer - "]
pub type GcRmxgmHwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gc_rbb1_hw` reader - "]
pub type GcRbb1HwR = crate::FieldReader;
#[doc = "Field `gc_rbb1_hw` writer - "]
pub type GcRbb1HwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gc_rbb2_hw` reader - "]
pub type GcRbb2HwR = crate::FieldReader;
#[doc = "Field `gc_rbb2_hw` writer - "]
pub type GcRbb2HwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_tmx_hw` reader - "]
pub type GcTmxHwR = crate::FieldReader;
#[doc = "Field `gc_tmx_hw` writer - "]
pub type GcTmxHwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_tbb_hw` reader - "]
pub type GcTbbHwR = crate::FieldReader;
#[doc = "Field `gc_tbb_hw` writer - "]
pub type GcTbbHwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gc_tbb_boost_hw` reader - "]
pub type GcTbbBoostHwR = crate::FieldReader;
#[doc = "Field `gc_tbb_boost_hw` writer - "]
pub type GcTbbBoostHwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&self) -> GcLnaHwR {
        GcLnaHwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm_hw(&self) -> GcRmxgmHwR {
        GcRmxgmHwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&self) -> GcRbb1HwR {
        GcRbb1HwR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&self) -> GcRbb2HwR {
        GcRbb2HwR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx_hw(&self) -> GcTmxHwR {
        GcTmxHwR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb_hw(&self) -> GcTbbHwR {
        GcTbbHwR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost_hw(&self) -> GcTbbBoostHwR {
        GcTbbBoostHwR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&mut self) -> GcLnaHwW<'_, TrxGainHwSpec> {
        GcLnaHwW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm_hw(&mut self) -> GcRmxgmHwW<'_, TrxGainHwSpec> {
        GcRmxgmHwW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&mut self) -> GcRbb1HwW<'_, TrxGainHwSpec> {
        GcRbb1HwW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&mut self) -> GcRbb2HwW<'_, TrxGainHwSpec> {
        GcRbb2HwW::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx_hw(&mut self) -> GcTmxHwW<'_, TrxGainHwSpec> {
        GcTmxHwW::new(self, 16)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb_hw(&mut self) -> GcTbbHwW<'_, TrxGainHwSpec> {
        GcTbbHwW::new(self, 20)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost_hw(&mut self) -> GcTbbBoostHwW<'_, TrxGainHwSpec> {
        GcTbbBoostHwW::new(self, 28)
    }
}
#[doc = "trx gain hardware readback\n\nYou can [`read`](crate::Reg::read) this register and get [`trx_gain_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trx_gain_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrxGainHwSpec;
impl crate::RegisterSpec for TrxGainHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trx_gain_hw::R`](R) reader structure"]
impl crate::Readable for TrxGainHwSpec {}
#[doc = "`write(|w| ..)` method takes [`trx_gain_hw::W`](W) writer structure"]
impl crate::Writable for TrxGainHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets trx_gain_hw to value 0"]
impl crate::Resettable for TrxGainHwSpec {}
