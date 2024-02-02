#[doc = "Register `sf_aes_key_r1_6` reader"]
pub type R = crate::R<SfAesKeyR1_6Spec>;
#[doc = "Register `sf_aes_key_r1_6` writer"]
pub type W = crate::W<SfAesKeyR1_6Spec>;
#[doc = "Field `sf_aes_key_r1_6` reader - "]
pub type SfAesKeyR1_6R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r1_6` writer - "]
pub type SfAesKeyR1_6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_6(&self) -> SfAesKeyR1_6R {
        SfAesKeyR1_6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_6(&mut self) -> SfAesKeyR1_6W<'_, SfAesKeyR1_6Spec> {
        SfAesKeyR1_6W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r1_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR1_6Spec;
impl crate::RegisterSpec for SfAesKeyR1_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r1_6::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR1_6Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r1_6::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR1_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r1_6 to value 0"]
impl crate::Resettable for SfAesKeyR1_6Spec {}
