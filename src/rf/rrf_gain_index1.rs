#[doc = "Register `rrf_gain_index1` reader"]
pub type R = crate::R<RrfGainIndex1Spec>;
#[doc = "Register `rrf_gain_index1` writer"]
pub type W = crate::W<RrfGainIndex1Spec>;
#[doc = "Field `gain_ctrl0_gc_rmxgm` reader - "]
pub type GainCtrl0GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_rmxgm` writer - "]
pub type GainCtrl0GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl0_gc_lna` reader - "]
pub type GainCtrl0GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl0_gc_lna` writer - "]
pub type GainCtrl0GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl1_gc_rmxgm` reader - "]
pub type GainCtrl1GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_rmxgm` writer - "]
pub type GainCtrl1GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl1_gc_lna` reader - "]
pub type GainCtrl1GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl1_gc_lna` writer - "]
pub type GainCtrl1GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl2_gc_rmxgm` reader - "]
pub type GainCtrl2GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_rmxgm` writer - "]
pub type GainCtrl2GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl2_gc_lna` reader - "]
pub type GainCtrl2GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl2_gc_lna` writer - "]
pub type GainCtrl2GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl3_gc_rmxgm` reader - "]
pub type GainCtrl3GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_rmxgm` writer - "]
pub type GainCtrl3GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl3_gc_lna` reader - "]
pub type GainCtrl3GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl3_gc_lna` writer - "]
pub type GainCtrl3GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl4_gc_rmxgm` reader - "]
pub type GainCtrl4GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_rmxgm` writer - "]
pub type GainCtrl4GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl4_gc_lna` reader - "]
pub type GainCtrl4GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl4_gc_lna` writer - "]
pub type GainCtrl4GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gain_ctrl5_gc_rmxgm` reader - "]
pub type GainCtrl5GcRmxgmR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_rmxgm` writer - "]
pub type GainCtrl5GcRmxgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gain_ctrl5_gc_lna` reader - "]
pub type GainCtrl5GcLnaR = crate::FieldReader;
#[doc = "Field `gain_ctrl5_gc_lna` writer - "]
pub type GainCtrl5GcLnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&self) -> GainCtrl0GcRmxgmR {
        GainCtrl0GcRmxgmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&self) -> GainCtrl0GcLnaR {
        GainCtrl0GcLnaR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&self) -> GainCtrl1GcRmxgmR {
        GainCtrl1GcRmxgmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&self) -> GainCtrl1GcLnaR {
        GainCtrl1GcLnaR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&self) -> GainCtrl2GcRmxgmR {
        GainCtrl2GcRmxgmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&self) -> GainCtrl2GcLnaR {
        GainCtrl2GcLnaR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&self) -> GainCtrl3GcRmxgmR {
        GainCtrl3GcRmxgmR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&self) -> GainCtrl3GcLnaR {
        GainCtrl3GcLnaR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&self) -> GainCtrl4GcRmxgmR {
        GainCtrl4GcRmxgmR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&self) -> GainCtrl4GcLnaR {
        GainCtrl4GcLnaR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&self) -> GainCtrl5GcRmxgmR {
        GainCtrl5GcRmxgmR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&self) -> GainCtrl5GcLnaR {
        GainCtrl5GcLnaR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&mut self) -> GainCtrl0GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl0GcRmxgmW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&mut self) -> GainCtrl0GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl0GcLnaW::new(self, 2)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&mut self) -> GainCtrl1GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl1GcRmxgmW::new(self, 5)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&mut self) -> GainCtrl1GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl1GcLnaW::new(self, 7)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&mut self) -> GainCtrl2GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl2GcRmxgmW::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&mut self) -> GainCtrl2GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl2GcLnaW::new(self, 12)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&mut self) -> GainCtrl3GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl3GcRmxgmW::new(self, 15)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&mut self) -> GainCtrl3GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl3GcLnaW::new(self, 17)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&mut self) -> GainCtrl4GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl4GcRmxgmW::new(self, 20)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&mut self) -> GainCtrl4GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl4GcLnaW::new(self, 22)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&mut self) -> GainCtrl5GcRmxgmW<'_, RrfGainIndex1Spec> {
        GainCtrl5GcRmxgmW::new(self, 25)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&mut self) -> GainCtrl5GcLnaW<'_, RrfGainIndex1Spec> {
        GainCtrl5GcLnaW::new(self, 27)
    }
}
#[doc = "rrf_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrf_gain_index1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrf_gain_index1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrfGainIndex1Spec;
impl crate::RegisterSpec for RrfGainIndex1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrf_gain_index1::R`](R) reader structure"]
impl crate::Readable for RrfGainIndex1Spec {}
#[doc = "`write(|w| ..)` method takes [`rrf_gain_index1::W`](W) writer structure"]
impl crate::Writable for RrfGainIndex1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rrf_gain_index1 to value 0"]
impl crate::Resettable for RrfGainIndex1Spec {}
