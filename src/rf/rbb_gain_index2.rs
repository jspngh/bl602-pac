#[doc = "Register `rbb_gain_index2` reader"]
pub type R = crate::R<RbbGainIndex2Spec>;
#[doc = "Register `rbb_gain_index2` writer"]
pub type W = crate::W<RbbGainIndex2Spec>;
#[doc = "Field `gain_ctrl4_gc_rbb1` reader - "]
pub type GainCtrl4GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_rbb1` writer - "]
pub type GainCtrl4GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl4_gc_rbb2` reader - "]
pub type GainCtrl4GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_rbb2` writer - "]
pub type GainCtrl4GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl5_gc_rbb1` reader - "]
pub type GainCtrl5GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_rbb1` writer - "]
pub type GainCtrl5GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl5_gc_rbb2` reader - "]
pub type GainCtrl5GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_rbb2` writer - "]
pub type GainCtrl5GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl6_gc_rbb1` reader - "]
pub type GainCtrl6GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_rbb1` writer - "]
pub type GainCtrl6GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl6_gc_rbb2` reader - "]
pub type GainCtrl6GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl6_gc_rbb2` writer - "]
pub type GainCtrl6GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl7_gc_rbb1` reader - "]
pub type GainCtrl7GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_rbb1` writer - "]
pub type GainCtrl7GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl7_gc_rbb2` reader - "]
pub type GainCtrl7GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl7_gc_rbb2` writer - "]
pub type GainCtrl7GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb1(&self) -> GainCtrl4GcRbb1R {
        GainCtrl4GcRbb1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb2(&self) -> GainCtrl4GcRbb2R {
        GainCtrl4GcRbb2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb1(&self) -> GainCtrl5GcRbb1R {
        GainCtrl5GcRbb1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb2(&self) -> GainCtrl5GcRbb2R {
        GainCtrl5GcRbb2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb1(&self) -> GainCtrl6GcRbb1R {
        GainCtrl6GcRbb1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb2(&self) -> GainCtrl6GcRbb2R {
        GainCtrl6GcRbb2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb1(&self) -> GainCtrl7GcRbb1R {
        GainCtrl7GcRbb1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb2(&self) -> GainCtrl7GcRbb2R {
        GainCtrl7GcRbb2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb1(&mut self) -> GainCtrl4GcRbb1W<'_, RbbGainIndex2Spec> {
        GainCtrl4GcRbb1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb2(&mut self) -> GainCtrl4GcRbb2W<'_, RbbGainIndex2Spec> {
        GainCtrl4GcRbb2W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb1(&mut self) -> GainCtrl5GcRbb1W<'_, RbbGainIndex2Spec> {
        GainCtrl5GcRbb1W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb2(&mut self) -> GainCtrl5GcRbb2W<'_, RbbGainIndex2Spec> {
        GainCtrl5GcRbb2W::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb1(&mut self) -> GainCtrl6GcRbb1W<'_, RbbGainIndex2Spec> {
        GainCtrl6GcRbb1W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb2(&mut self) -> GainCtrl6GcRbb2W<'_, RbbGainIndex2Spec> {
        GainCtrl6GcRbb2W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb1(&mut self) -> GainCtrl7GcRbb1W<'_, RbbGainIndex2Spec> {
        GainCtrl7GcRbb1W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb2(&mut self) -> GainCtrl7GcRbb2W<'_, RbbGainIndex2Spec> {
        GainCtrl7GcRbb2W::new(self, 28)
    }
}
#[doc = "rbb_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbGainIndex2Spec;
impl crate::RegisterSpec for RbbGainIndex2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_gain_index2::R`](R) reader structure"]
impl crate::Readable for RbbGainIndex2Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb_gain_index2::W`](W) writer structure"]
impl crate::Writable for RbbGainIndex2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_gain_index2 to value 0"]
impl crate::Resettable for RbbGainIndex2Spec {}
