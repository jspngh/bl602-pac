#[doc = "Register `rrf_gain_index2` reader"]
pub type R = crate::R<RrfGainIndex2Spec>;
#[doc = "Register `rrf_gain_index2` writer"]
pub type W = crate::W<RrfGainIndex2Spec>;
#[doc = "Field `gain_ctrl8_gc_rmxgm` reader - "]
pub type GainCtrl8GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl8_gc_rmxgm` writer - "]
pub type GainCtrl8GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl8_gc_lna` reader - "]
pub type GainCtrl8GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl8_gc_lna` writer - "]
pub type GainCtrl8GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl7_gc_rmxgm` reader - "]
pub type GainCtrl7GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_rmxgm` writer - "]
pub type GainCtrl7GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl7_gc_lna` reader - "]
pub type GainCtrl7GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_lna` writer - "]
pub type GainCtrl7GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl6_gc_rmxgm` reader - "]
pub type GainCtrl6GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_rmxgm` writer - "]
pub type GainCtrl6GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl6_gc_lna` reader - "]
pub type GainCtrl6GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_lna` writer - "]
pub type GainCtrl6GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rmxgm(&self) -> GainCtrl8GcRmxgmR {
        GainCtrl8GcRmxgmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_lna(&self) -> GainCtrl8GcLnaR {
        GainCtrl8GcLnaR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rmxgm(&self) -> GainCtrl7GcRmxgmR {
        GainCtrl7GcRmxgmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_lna(&self) -> GainCtrl7GcLnaR {
        GainCtrl7GcLnaR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rmxgm(&self) -> GainCtrl6GcRmxgmR {
        GainCtrl6GcRmxgmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_lna(&self) -> GainCtrl6GcLnaR {
        GainCtrl6GcLnaR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rmxgm(&mut self) -> GainCtrl8GcRmxgmW<'_, RrfGainIndex2Spec> {
        GainCtrl8GcRmxgmW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_lna(&mut self) -> GainCtrl8GcLnaW<'_, RrfGainIndex2Spec> {
        GainCtrl8GcLnaW::new(self, 2)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rmxgm(&mut self) -> GainCtrl7GcRmxgmW<'_, RrfGainIndex2Spec> {
        GainCtrl7GcRmxgmW::new(self, 5)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_lna(&mut self) -> GainCtrl7GcLnaW<'_, RrfGainIndex2Spec> {
        GainCtrl7GcLnaW::new(self, 7)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rmxgm(&mut self) -> GainCtrl6GcRmxgmW<'_, RrfGainIndex2Spec> {
        GainCtrl6GcRmxgmW::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_lna(&mut self) -> GainCtrl6GcLnaW<'_, RrfGainIndex2Spec> {
        GainCtrl6GcLnaW::new(self, 12)
    }
}
#[doc = "rrf_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrf_gain_index2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrf_gain_index2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrfGainIndex2Spec;
impl crate::RegisterSpec for RrfGainIndex2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrf_gain_index2::R`](R) reader structure"]
impl crate::Readable for RrfGainIndex2Spec {}
#[doc = "`write(|w| ..)` method takes [`rrf_gain_index2::W`](W) writer structure"]
impl crate::Writable for RrfGainIndex2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rrf_gain_index2 to value 0"]
impl crate::Resettable for RrfGainIndex2Spec {}
