#[doc = "Register `sf_aes_key_r1_3` reader"]
pub type R = crate::R<SfAesKeyR1_3Spec>;
#[doc = "Register `sf_aes_key_r1_3` writer"]
pub type W = crate::W<SfAesKeyR1_3Spec>;
#[doc = "Field `sf_aes_key_r1_3` reader - "]
pub type SfAesKeyR1_3R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r1_3` writer - "]
pub type SfAesKeyR1_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_3(&self) -> SfAesKeyR1_3R {
        SfAesKeyR1_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r1_3(&mut self) -> SfAesKeyR1_3W<'_, SfAesKeyR1_3Spec> {
        SfAesKeyR1_3W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r1_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR1_3Spec;
impl crate::RegisterSpec for SfAesKeyR1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r1_3::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR1_3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r1_3::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR1_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r1_3 to value 0"]
impl crate::Resettable for SfAesKeyR1_3Spec {}
