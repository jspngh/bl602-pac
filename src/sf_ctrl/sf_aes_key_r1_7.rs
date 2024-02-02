#[doc = "Register `sf_aes_key_r1_7` reader"]
pub type R = crate::R<SfAesKeyR1_7Spec>;
#[doc = "Register `sf_aes_key_r1_7` writer"]
pub type W = crate::W<SfAesKeyR1_7Spec>;
#[doc = "Field `sf_aes_key_r1_7` reader - "]
pub type SfAesKeyR1_7R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r1_7` writer - "]
pub type SfAesKeyR1_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_7(&self) -> SfAesKeyR1_7R {
        SfAesKeyR1_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_7(&mut self) -> SfAesKeyR1_7W<'_, SfAesKeyR1_7Spec> {
        SfAesKeyR1_7W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r1_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR1_7Spec;
impl crate::RegisterSpec for SfAesKeyR1_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r1_7::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR1_7Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r1_7::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR1_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r1_7 to value 0"]
impl crate::Resettable for SfAesKeyR1_7Spec {}
