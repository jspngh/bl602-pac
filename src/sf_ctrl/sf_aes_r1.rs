#[doc = "Register `sf_aes_r1` reader"]
pub type R = crate::R<SfAesR1Spec>;
#[doc = "Register `sf_aes_r1` writer"]
pub type W = crate::W<SfAesR1Spec>;
#[doc = "Field `sf_aes_r1_end` reader - "]
pub type SfAesR1EndR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_r1_end` writer - "]
pub type SfAesR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_r1_start` reader - "]
pub type SfAesR1StartR = crate::FieldReader<u16>;
#[doc = "Field `sf_aes_r1_start` writer - "]
pub type SfAesR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `sf_aes_r1_hw_key_en` reader - "]
pub type SfAesR1HwKeyEnR = crate::BitReader;
#[doc = "Field `sf_aes_r1_hw_key_en` writer - "]
pub type SfAesR1HwKeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_r1_en` reader - "]
pub type SfAesR1EnR = crate::BitReader;
#[doc = "Field `sf_aes_r1_en` writer - "]
pub type SfAesR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_r1_lock` reader - "]
pub type SfAesR1LockR = crate::BitReader;
#[doc = "Field `sf_aes_r1_lock` writer - "]
pub type SfAesR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r1_end(&self) -> SfAesR1EndR {
        SfAesR1EndR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r1_start(&self) -> SfAesR1StartR {
        SfAesR1StartR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r1_hw_key_en(&self) -> SfAesR1HwKeyEnR {
        SfAesR1HwKeyEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r1_en(&self) -> SfAesR1EnR {
        SfAesR1EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r1_lock(&self) -> SfAesR1LockR {
        SfAesR1LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r1_end(&mut self) -> SfAesR1EndW<'_, SfAesR1Spec> {
        SfAesR1EndW::new(self, 0)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r1_start(&mut self) -> SfAesR1StartW<'_, SfAesR1Spec> {
        SfAesR1StartW::new(self, 14)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r1_hw_key_en(&mut self) -> SfAesR1HwKeyEnW<'_, SfAesR1Spec> {
        SfAesR1HwKeyEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r1_en(&mut self) -> SfAesR1EnW<'_, SfAesR1Spec> {
        SfAesR1EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r1_lock(&mut self) -> SfAesR1LockW<'_, SfAesR1Spec> {
        SfAesR1LockW::new(self, 31)
    }
}
#[doc = "sf_aes_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesR1Spec;
impl crate::RegisterSpec for SfAesR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_r1::R`](R) reader structure"]
impl crate::Readable for SfAesR1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_r1::W`](W) writer structure"]
impl crate::Writable for SfAesR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_r1 to value 0x3fff"]
impl crate::Resettable for SfAesR1Spec {
    const RESET_VALUE: u32 = 0x3fff;
}
