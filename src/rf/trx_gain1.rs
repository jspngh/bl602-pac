#[doc = "Register `trx_gain1` reader"]
pub type R = crate::R<TrxGain1Spec>;
#[doc = "Register `trx_gain1` writer"]
pub type W = crate::W<TrxGain1Spec>;
#[doc = "Field `gc_lna` reader - "]
pub type GcLnaR = crate::FieldReader;
#[doc = "Field `gc_lna` writer - "]
pub type GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_rmxgm` reader - "]
pub type GcRmxgmR = crate::FieldReader;
#[doc = "Field `gc_rmxgm` writer - "]
pub type GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gc_rbb1` reader - "]
pub type GcRbb1R = crate::FieldReader;
#[doc = "Field `gc_rbb1` writer - "]
pub type GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gc_rbb2` reader - "]
pub type GcRbb2R = crate::FieldReader;
#[doc = "Field `gc_rbb2` writer - "]
pub type GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_tmx` reader - "]
pub type GcTmxR = crate::FieldReader;
#[doc = "Field `gc_tmx` writer - "]
pub type GcTmxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gc_tbb` reader - "]
pub type GcTbbR = crate::FieldReader;
#[doc = "Field `gc_tbb` writer - "]
pub type GcTbbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gc_tbb_boost` reader - "]
pub type GcTbbBoostR = crate::FieldReader;
#[doc = "Field `gc_tbb_boost` writer - "]
pub type GcTbbBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&self) -> GcLnaR {
        GcLnaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm(&self) -> GcRmxgmR {
        GcRmxgmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1(&self) -> GcRbb1R {
        GcRbb1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2(&self) -> GcRbb2R {
        GcRbb2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx(&self) -> GcTmxR {
        GcTmxR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb(&self) -> GcTbbR {
        GcTbbR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost(&self) -> GcTbbBoostR {
        GcTbbBoostR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&mut self) -> GcLnaW<'_, TrxGain1Spec> {
        GcLnaW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm(&mut self) -> GcRmxgmW<'_, TrxGain1Spec> {
        GcRmxgmW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1(&mut self) -> GcRbb1W<'_, TrxGain1Spec> {
        GcRbb1W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2(&mut self) -> GcRbb2W<'_, TrxGain1Spec> {
        GcRbb2W::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx(&mut self) -> GcTmxW<'_, TrxGain1Spec> {
        GcTmxW::new(self, 16)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb(&mut self) -> GcTbbW<'_, TrxGain1Spec> {
        GcTbbW::new(self, 20)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost(&mut self) -> GcTbbBoostW<'_, TrxGain1Spec> {
        GcTbbBoostW::new(self, 28)
    }
}
#[doc = "gain control1\n\nYou can [`read`](crate::Reg::read) this register and get [`trx_gain1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trx_gain1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrxGain1Spec;
impl crate::RegisterSpec for TrxGain1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trx_gain1::R`](R) reader structure"]
impl crate::Readable for TrxGain1Spec {}
#[doc = "`write(|w| ..)` method takes [`trx_gain1::W`](W) writer structure"]
impl crate::Writable for TrxGain1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets trx_gain1 to value 0"]
impl crate::Resettable for TrxGain1Spec {}
