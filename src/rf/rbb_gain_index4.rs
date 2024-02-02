#[doc = "Register `rbb_gain_index4` reader"]
pub type R = crate::R<RbbGainIndex4Spec>;
#[doc = "Register `rbb_gain_index4` writer"]
pub type W = crate::W<RbbGainIndex4Spec>;
#[doc = "Field `gain_ctrl12_gc_rbb1` reader - "]
pub type GainCtrl12GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl12_gc_rbb1` writer - "]
pub type GainCtrl12GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl12_gc_rbb2` reader - "]
pub type GainCtrl12GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl12_gc_rbb2` writer - "]
pub type GainCtrl12GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl13_gc_rbb1` reader - "]
pub type GainCtrl13GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl13_gc_rbb1` writer - "]
pub type GainCtrl13GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl13_gc_rbb2` reader - "]
pub type GainCtrl13GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl13_gc_rbb2` writer - "]
pub type GainCtrl13GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl14_gc_rbb1` reader - "]
pub type GainCtrl14GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl14_gc_rbb1` writer - "]
pub type GainCtrl14GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl14_gc_rbb2` reader - "]
pub type GainCtrl14GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl14_gc_rbb2` writer - "]
pub type GainCtrl14GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl15_gc_rbb1` reader - "]
pub type GainCtrl15GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl15_gc_rbb1` writer - "]
pub type GainCtrl15GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl15_gc_rbb2` reader - "]
pub type GainCtrl15GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl15_gc_rbb2` writer - "]
pub type GainCtrl15GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb1(&self) -> GainCtrl12GcRbb1R {
        GainCtrl12GcRbb1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb2(&self) -> GainCtrl12GcRbb2R {
        GainCtrl12GcRbb2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb1(&self) -> GainCtrl13GcRbb1R {
        GainCtrl13GcRbb1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb2(&self) -> GainCtrl13GcRbb2R {
        GainCtrl13GcRbb2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb1(&self) -> GainCtrl14GcRbb1R {
        GainCtrl14GcRbb1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb2(&self) -> GainCtrl14GcRbb2R {
        GainCtrl14GcRbb2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb1(&self) -> GainCtrl15GcRbb1R {
        GainCtrl15GcRbb1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb2(&self) -> GainCtrl15GcRbb2R {
        GainCtrl15GcRbb2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb1(&mut self) -> GainCtrl12GcRbb1W<'_, RbbGainIndex4Spec> {
        GainCtrl12GcRbb1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb2(&mut self) -> GainCtrl12GcRbb2W<'_, RbbGainIndex4Spec> {
        GainCtrl12GcRbb2W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb1(&mut self) -> GainCtrl13GcRbb1W<'_, RbbGainIndex4Spec> {
        GainCtrl13GcRbb1W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb2(&mut self) -> GainCtrl13GcRbb2W<'_, RbbGainIndex4Spec> {
        GainCtrl13GcRbb2W::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb1(&mut self) -> GainCtrl14GcRbb1W<'_, RbbGainIndex4Spec> {
        GainCtrl14GcRbb1W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb2(&mut self) -> GainCtrl14GcRbb2W<'_, RbbGainIndex4Spec> {
        GainCtrl14GcRbb2W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb1(&mut self) -> GainCtrl15GcRbb1W<'_, RbbGainIndex4Spec> {
        GainCtrl15GcRbb1W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb2(&mut self) -> GainCtrl15GcRbb2W<'_, RbbGainIndex4Spec> {
        GainCtrl15GcRbb2W::new(self, 28)
    }
}
#[doc = "rbb_gain_index4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbGainIndex4Spec;
impl crate::RegisterSpec for RbbGainIndex4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_gain_index4::R`](R) reader structure"]
impl crate::Readable for RbbGainIndex4Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb_gain_index4::W`](W) writer structure"]
impl crate::Writable for RbbGainIndex4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_gain_index4 to value 0"]
impl crate::Resettable for RbbGainIndex4Spec {}
