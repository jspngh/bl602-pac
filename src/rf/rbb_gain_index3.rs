#[doc = "Register `rbb_gain_index3` reader"]
pub type R = crate::R<RbbGainIndex3Spec>;
#[doc = "Register `rbb_gain_index3` writer"]
pub type W = crate::W<RbbGainIndex3Spec>;
#[doc = "Field `gain_ctrl8_gc_rbb1` reader - "]
pub type GainCtrl8GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl8_gc_rbb1` writer - "]
pub type GainCtrl8GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl8_gc_rbb2` reader - "]
pub type GainCtrl8GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl8_gc_rbb2` writer - "]
pub type GainCtrl8GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl9_gc_rbb1` reader - "]
pub type GainCtrl9GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl9_gc_rbb1` writer - "]
pub type GainCtrl9GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl9_gc_rbb2` reader - "]
pub type GainCtrl9GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl9_gc_rbb2` writer - "]
pub type GainCtrl9GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl10_gc_rbb1` reader - "]
pub type GainCtrl10GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl10_gc_rbb1` writer - "]
pub type GainCtrl10GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl10_gc_rbb2` reader - "]
pub type GainCtrl10GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl10_gc_rbb2` writer - "]
pub type GainCtrl10GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl11_gc_rbb1` reader - "]
pub type GainCtrl11GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl11_gc_rbb1` writer - "]
pub type GainCtrl11GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl11_gc_rbb2` reader - "]
pub type GainCtrl11GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl11_gc_rbb2` writer - "]
pub type GainCtrl11GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rbb1(&self) -> GainCtrl8GcRbb1R {
        GainCtrl8GcRbb1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rbb2(&self) -> GainCtrl8GcRbb2R {
        GainCtrl8GcRbb2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl9_gc_rbb1(&self) -> GainCtrl9GcRbb1R {
        GainCtrl9GcRbb1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl9_gc_rbb2(&self) -> GainCtrl9GcRbb2R {
        GainCtrl9GcRbb2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl10_gc_rbb1(&self) -> GainCtrl10GcRbb1R {
        GainCtrl10GcRbb1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl10_gc_rbb2(&self) -> GainCtrl10GcRbb2R {
        GainCtrl10GcRbb2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl11_gc_rbb1(&self) -> GainCtrl11GcRbb1R {
        GainCtrl11GcRbb1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl11_gc_rbb2(&self) -> GainCtrl11GcRbb2R {
        GainCtrl11GcRbb2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rbb1(&mut self) -> GainCtrl8GcRbb1W<'_, RbbGainIndex3Spec> {
        GainCtrl8GcRbb1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rbb2(&mut self) -> GainCtrl8GcRbb2W<'_, RbbGainIndex3Spec> {
        GainCtrl8GcRbb2W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl9_gc_rbb1(&mut self) -> GainCtrl9GcRbb1W<'_, RbbGainIndex3Spec> {
        GainCtrl9GcRbb1W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl9_gc_rbb2(&mut self) -> GainCtrl9GcRbb2W<'_, RbbGainIndex3Spec> {
        GainCtrl9GcRbb2W::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl10_gc_rbb1(&mut self) -> GainCtrl10GcRbb1W<'_, RbbGainIndex3Spec> {
        GainCtrl10GcRbb1W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl10_gc_rbb2(&mut self) -> GainCtrl10GcRbb2W<'_, RbbGainIndex3Spec> {
        GainCtrl10GcRbb2W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl11_gc_rbb1(&mut self) -> GainCtrl11GcRbb1W<'_, RbbGainIndex3Spec> {
        GainCtrl11GcRbb1W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl11_gc_rbb2(&mut self) -> GainCtrl11GcRbb2W<'_, RbbGainIndex3Spec> {
        GainCtrl11GcRbb2W::new(self, 28)
    }
}
#[doc = "rbb_gain_index3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbGainIndex3Spec;
impl crate::RegisterSpec for RbbGainIndex3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_gain_index3::R`](R) reader structure"]
impl crate::Readable for RbbGainIndex3Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb_gain_index3::W`](W) writer structure"]
impl crate::Writable for RbbGainIndex3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_gain_index3 to value 0"]
impl crate::Resettable for RbbGainIndex3Spec {}
