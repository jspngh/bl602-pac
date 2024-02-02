#[doc = "Register `sf_aes` reader"]
pub type R = crate::R<SfAesSpec>;
#[doc = "Register `sf_aes` writer"]
pub type W = crate::W<SfAesSpec>;
#[doc = "Field `sf_aes_en` reader - "]
pub type SfAesEnR = crate::BitReader;
#[doc = "Field `sf_aes_en` writer - "]
pub type SfAesEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_mode` reader - "]
pub type SfAesModeR = crate::FieldReader;
#[doc = "Field `sf_aes_mode` writer - "]
pub type SfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_aes_pref_trig` reader - "]
pub type SfAesPrefTrigR = crate::BitReader;
#[doc = "Field `sf_aes_pref_trig` writer - "]
pub type SfAesPrefTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_pref_busy` reader - "]
pub type SfAesPrefBusyR = crate::BitReader;
#[doc = "Field `sf_aes_status` reader - "]
pub type SfAesStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&self) -> SfAesEnR {
        SfAesEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&self) -> SfAesModeR {
        SfAesModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&self) -> SfAesPrefTrigR {
        SfAesPrefTrigR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_pref_busy(&self) -> SfAesPrefBusyR {
        SfAesPrefBusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&self) -> SfAesStatusR {
        SfAesStatusR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&mut self) -> SfAesEnW<'_, SfAesSpec> {
        SfAesEnW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&mut self) -> SfAesModeW<'_, SfAesSpec> {
        SfAesModeW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&mut self) -> SfAesPrefTrigW<'_, SfAesSpec> {
        SfAesPrefTrigW::new(self, 3)
    }
}
#[doc = "sf_aes.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesSpec;
impl crate::RegisterSpec for SfAesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes::R`](R) reader structure"]
impl crate::Readable for SfAesSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes::W`](W) writer structure"]
impl crate::Writable for SfAesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes to value 0x40"]
impl crate::Resettable for SfAesSpec {
    const RESET_VALUE: u32 = 0x40;
}
