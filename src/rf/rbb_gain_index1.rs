#[doc = "Register `rbb_gain_index1` reader"]
pub type R = crate::R<RbbGainIndex1Spec>;
#[doc = "Register `rbb_gain_index1` writer"]
pub type W = crate::W<RbbGainIndex1Spec>;
#[doc = "Field `gain_ctrl0_gc_rbb1` reader - "]
pub type GainCtrl0GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_rbb1` writer - "]
pub type GainCtrl0GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl0_gc_rbb2` reader - "]
pub type GainCtrl0GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_rbb2` writer - "]
pub type GainCtrl0GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl1_gc_rbb1` reader - "]
pub type GainCtrl1GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_rbb1` writer - "]
pub type GainCtrl1GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl1_gc_rbb2` reader - "]
pub type GainCtrl1GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_rbb2` writer - "]
pub type GainCtrl1GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl2_gc_rbb1` reader - "]
pub type GainCtrl2GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_rbb1` writer - "]
pub type GainCtrl2GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl2_gc_rbb2` reader - "]
pub type GainCtrl2GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_rbb2` writer - "]
pub type GainCtrl2GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl3_gc_rbb1` reader - "]
pub type GainCtrl3GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_rbb1` writer - "]
pub type GainCtrl3GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl3_gc_rbb2` reader - "]
pub type GainCtrl3GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_rbb2` writer - "]
pub type GainCtrl3GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb1(&self) -> GainCtrl0GcRbb1R {
        GainCtrl0GcRbb1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb2(&self) -> GainCtrl0GcRbb2R {
        GainCtrl0GcRbb2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb1(&self) -> GainCtrl1GcRbb1R {
        GainCtrl1GcRbb1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb2(&self) -> GainCtrl1GcRbb2R {
        GainCtrl1GcRbb2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb1(&self) -> GainCtrl2GcRbb1R {
        GainCtrl2GcRbb1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb2(&self) -> GainCtrl2GcRbb2R {
        GainCtrl2GcRbb2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb1(&self) -> GainCtrl3GcRbb1R {
        GainCtrl3GcRbb1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb2(&self) -> GainCtrl3GcRbb2R {
        GainCtrl3GcRbb2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb1(&mut self) -> GainCtrl0GcRbb1W<'_, RbbGainIndex1Spec> {
        GainCtrl0GcRbb1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb2(&mut self) -> GainCtrl0GcRbb2W<'_, RbbGainIndex1Spec> {
        GainCtrl0GcRbb2W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb1(&mut self) -> GainCtrl1GcRbb1W<'_, RbbGainIndex1Spec> {
        GainCtrl1GcRbb1W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb2(&mut self) -> GainCtrl1GcRbb2W<'_, RbbGainIndex1Spec> {
        GainCtrl1GcRbb2W::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb1(&mut self) -> GainCtrl2GcRbb1W<'_, RbbGainIndex1Spec> {
        GainCtrl2GcRbb1W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb2(&mut self) -> GainCtrl2GcRbb2W<'_, RbbGainIndex1Spec> {
        GainCtrl2GcRbb2W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb1(&mut self) -> GainCtrl3GcRbb1W<'_, RbbGainIndex1Spec> {
        GainCtrl3GcRbb1W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb2(&mut self) -> GainCtrl3GcRbb2W<'_, RbbGainIndex1Spec> {
        GainCtrl3GcRbb2W::new(self, 28)
    }
}
#[doc = "rbb_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbGainIndex1Spec;
impl crate::RegisterSpec for RbbGainIndex1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_gain_index1::R`](R) reader structure"]
impl crate::Readable for RbbGainIndex1Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb_gain_index1::W`](W) writer structure"]
impl crate::Writable for RbbGainIndex1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_gain_index1 to value 0"]
impl crate::Resettable for RbbGainIndex1Spec {}
