#[doc = "Register `sf_aes_key_r0_2` reader"]
pub type R = crate::R<SfAesKeyR0_2Spec>;
#[doc = "Register `sf_aes_key_r0_2` writer"]
pub type W = crate::W<SfAesKeyR0_2Spec>;
#[doc = "Field `sf_aes_key_r0_2` reader - "]
pub type SfAesKeyR0_2R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r0_2` writer - "]
pub type SfAesKeyR0_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_2(&self) -> SfAesKeyR0_2R {
        SfAesKeyR0_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_2(&mut self) -> SfAesKeyR0_2W<'_, SfAesKeyR0_2Spec> {
        SfAesKeyR0_2W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r0_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR0_2Spec;
impl crate::RegisterSpec for SfAesKeyR0_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r0_2::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR0_2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r0_2::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR0_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r0_2 to value 0"]
impl crate::Resettable for SfAesKeyR0_2Spec {}
