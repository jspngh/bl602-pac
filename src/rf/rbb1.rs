#[doc = "Register `rbb1` reader"]
pub type R = crate::R<Rbb1Spec>;
#[doc = "Register `rbb1` writer"]
pub type W = crate::W<Rbb1Spec>;
#[doc = "Field `rosdac_q` reader - "]
pub type RosdacQR = crate::FieldReader;
#[doc = "Field `rosdac_q` writer - "]
pub type RosdacQW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_i` reader - "]
pub type RosdacIR = crate::FieldReader;
#[doc = "Field `rosdac_i` writer - "]
pub type RosdacIW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_q_hw` reader - "]
pub type RosdacQHwR = crate::FieldReader;
#[doc = "Field `rosdac_q_hw` writer - "]
pub type RosdacQHwW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_i_hw` reader - "]
pub type RosdacIHwR = crate::FieldReader;
#[doc = "Field `rosdac_i_hw` writer - "]
pub type RosdacIHwW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_range` reader - "]
pub type RosdacRangeR = crate::BitReader;
#[doc = "Field `rosdac_range` writer - "]
pub type RosdacRangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q(&self) -> RosdacQR {
        RosdacQR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i(&self) -> RosdacIR {
        RosdacIR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q_hw(&self) -> RosdacQHwR {
        RosdacQHwR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i_hw(&self) -> RosdacIHwR {
        RosdacIHwR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rosdac_range(&self) -> RosdacRangeR {
        RosdacRangeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q(&mut self) -> RosdacQW<'_, Rbb1Spec> {
        RosdacQW::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i(&mut self) -> RosdacIW<'_, Rbb1Spec> {
        RosdacIW::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q_hw(&mut self) -> RosdacQHwW<'_, Rbb1Spec> {
        RosdacQHwW::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i_hw(&mut self) -> RosdacIHwW<'_, Rbb1Spec> {
        RosdacIHwW::new(self, 24)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rosdac_range(&mut self) -> RosdacRangeW<'_, Rbb1Spec> {
        RosdacRangeW::new(self, 31)
    }
}
#[doc = "rbb1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbb1Spec;
impl crate::RegisterSpec for Rbb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb1::R`](R) reader structure"]
impl crate::Readable for Rbb1Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb1::W`](W) writer structure"]
impl crate::Writable for Rbb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb1 to value 0"]
impl crate::Resettable for Rbb1Spec {}
