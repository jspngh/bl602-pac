#[doc = "Register `sf_aes_key_r2_7` reader"]
pub type R = crate::R<SfAesKeyR2_7Spec>;
#[doc = "Register `sf_aes_key_r2_7` writer"]
pub type W = crate::W<SfAesKeyR2_7Spec>;
#[doc = "Field `sf_aes_key_r2_7` reader - "]
pub type SfAesKeyR2_7R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r2_7` writer - "]
pub type SfAesKeyR2_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_7(&self) -> SfAesKeyR2_7R {
        SfAesKeyR2_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_7(&mut self) -> SfAesKeyR2_7W<'_, SfAesKeyR2_7Spec> {
        SfAesKeyR2_7W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r2_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR2_7Spec;
impl crate::RegisterSpec for SfAesKeyR2_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r2_7::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR2_7Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r2_7::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR2_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r2_7 to value 0"]
impl crate::Resettable for SfAesKeyR2_7Spec {}
