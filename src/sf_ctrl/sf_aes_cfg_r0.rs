#[doc = "Register `sf_aes_cfg_r0` reader"]
pub type R = crate::R<SfAesCfgR0Spec>;
#[doc = "Register `sf_aes_cfg_r0` writer"]
pub type W = crate::W<SfAesCfgR0Spec>;
#[doc = "Field `sf_aes_region_r0_end` reader - "]
pub type SfAesRegionR0EndR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_region_r0_end` writer - "]
pub type SfAesRegionR0EndW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_region_r0_start` reader - "]
pub type SfAesRegionR0StartR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_region_r0_start` writer - "]
pub type SfAesRegionR0StartW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_region_r0_hw_key_en` reader - "]
pub type SfAesRegionR0HwKeyEnR = crate::BitReader;
#[doc = "Field `sf_aes_region_r0_hw_key_en` writer - "]
pub type SfAesRegionR0HwKeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_region_r0_en` reader - "]
pub type SfAesRegionR0EnR = crate::BitReader;
#[doc = "Field `sf_aes_region_r0_en` writer - "]
pub type SfAesRegionR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_region_r0_lock` reader - "]
pub type SfAesRegionR0LockR = crate::BitReader;
#[doc = "Field `sf_aes_region_r0_lock` writer - "]
pub type SfAesRegionR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_region_r0_end(&self) -> SfAesRegionR0EndR {
        SfAesRegionR0EndR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_region_r0_start(&self) -> SfAesRegionR0StartR {
        SfAesRegionR0StartR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_region_r0_hw_key_en(&self) -> SfAesRegionR0HwKeyEnR {
        SfAesRegionR0HwKeyEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_region_r0_en(&self) -> SfAesRegionR0EnR {
        SfAesRegionR0EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_region_r0_lock(&self) -> SfAesRegionR0LockR {
        SfAesRegionR0LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_region_r0_end(&mut self) -> SfAesRegionR0EndW<'_, SfAesCfgR0Spec> {
        SfAesRegionR0EndW::new(self, 0)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_region_r0_start(&mut self) -> SfAesRegionR0StartW<'_, SfAesCfgR0Spec> {
        SfAesRegionR0StartW::new(self, 14)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_region_r0_hw_key_en(&mut self) -> SfAesRegionR0HwKeyEnW<'_, SfAesCfgR0Spec> {
        SfAesRegionR0HwKeyEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_region_r0_en(&mut self) -> SfAesRegionR0EnW<'_, SfAesCfgR0Spec> {
        SfAesRegionR0EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_region_r0_lock(&mut self) -> SfAesRegionR0LockW<'_, SfAesCfgR0Spec> {
        SfAesRegionR0LockW::new(self, 31)
    }
}
#[doc = "sf_aes_cfg_r0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_cfg_r0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_cfg_r0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesCfgR0Spec;
impl crate::RegisterSpec for SfAesCfgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_cfg_r0::R`](R) reader structure"]
impl crate::Readable for SfAesCfgR0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_cfg_r0::W`](W) writer structure"]
impl crate::Writable for SfAesCfgR0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_cfg_r0 to value 0x3fff"]
impl crate::Resettable for SfAesCfgR0Spec {
    const RESET_VALUE: u32 = 0x3fff;
}
