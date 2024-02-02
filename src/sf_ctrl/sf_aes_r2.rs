#[doc = "Register `sf_aes_r2` reader"]
pub type R = crate::R<SfAesR2Spec>;
#[doc = "Register `sf_aes_r2` writer"]
pub type W = crate::W<SfAesR2Spec>;
#[doc = "Field `sf_aes_r2_end` reader - "]
pub type SfAesR2EndR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_r2_end` writer - "]
pub type SfAesR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_r2_start` reader - "]
pub type SfAesR2StartR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_r2_start` writer - "]
pub type SfAesR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_r2_hw_key_en` reader - "]
pub type SfAesR2HwKeyEnR = crate::BitReader;
#[doc = "Field `sf_aes_r2_hw_key_en` writer - "]
pub type SfAesR2HwKeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_r2_en` reader - "]
pub type SfAesR2EnR = crate::BitReader;
#[doc = "Field `sf_aes_r2_en` writer - "]
pub type SfAesR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_r2_lock` reader - "]
pub type SfAesR2LockR = crate::BitReader;
#[doc = "Field `sf_aes_r2_lock` writer - "]
pub type SfAesR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&self) -> SfAesR2EndR {
        SfAesR2EndR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&self) -> SfAesR2StartR {
        SfAesR2StartR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&self) -> SfAesR2HwKeyEnR {
        SfAesR2HwKeyEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&self) -> SfAesR2EnR {
        SfAesR2EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&self) -> SfAesR2LockR {
        SfAesR2LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&mut self) -> SfAesR2EndW<'_, SfAesR2Spec> {
        SfAesR2EndW::new(self, 0)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&mut self) -> SfAesR2StartW<'_, SfAesR2Spec> {
        SfAesR2StartW::new(self, 14)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&mut self) -> SfAesR2HwKeyEnW<'_, SfAesR2Spec> {
        SfAesR2HwKeyEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&mut self) -> SfAesR2EnW<'_, SfAesR2Spec> {
        SfAesR2EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&mut self) -> SfAesR2LockW<'_, SfAesR2Spec> {
        SfAesR2LockW::new(self, 31)
    }
}
#[doc = "sf_aes_r2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesR2Spec;
impl crate::RegisterSpec for SfAesR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_r2::R`](R) reader structure"]
impl crate::Readable for SfAesR2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_r2::W`](W) writer structure"]
impl crate::Writable for SfAesR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_r2 to value 0x3fff"]
impl crate::Resettable for SfAesR2Spec {
    const RESET_VALUE: u32 = 0x3fff;
}
