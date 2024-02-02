#[doc = "Register `rbb_gain_index5` reader"]
pub type R = crate::R<RbbGainIndex5Spec>;
#[doc = "Register `rbb_gain_index5` writer"]
pub type W = crate::W<RbbGainIndex5Spec>;
#[doc = "Field `gain_ctrl16_gc_rbb1` reader - "]
pub type GainCtrl16GcRbb1R = crate::FieldReader;
#[doc = "Field `gain_ctrl16_gc_rbb1` writer - "]
pub type GainCtrl16GcRbb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl16_gc_rbb2` reader - "]
pub type GainCtrl16GcRbb2R = crate::FieldReader;
#[doc = "Field `gain_ctrl16_gc_rbb2` writer - "]
pub type GainCtrl16GcRbb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl16_gc_rbb1(&self) -> GainCtrl16GcRbb1R {
        GainCtrl16GcRbb1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl16_gc_rbb2(&self) -> GainCtrl16GcRbb2R {
        GainCtrl16GcRbb2R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl16_gc_rbb1(&mut self) -> GainCtrl16GcRbb1W<'_, RbbGainIndex5Spec> {
        GainCtrl16GcRbb1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl16_gc_rbb2(&mut self) -> GainCtrl16GcRbb2W<'_, RbbGainIndex5Spec> {
        GainCtrl16GcRbb2W::new(self, 4)
    }
}
#[doc = "rbb_gain_index5.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbGainIndex5Spec;
impl crate::RegisterSpec for RbbGainIndex5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_gain_index5::R`](R) reader structure"]
impl crate::Readable for RbbGainIndex5Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb_gain_index5::W`](W) writer structure"]
impl crate::Writable for RbbGainIndex5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_gain_index5 to value 0"]
impl crate::Resettable for RbbGainIndex5Spec {}
