#[doc = "Register `sf_aes_key_r0_5` reader"]
pub type R = crate::R<SfAesKeyR0_5Spec>;
#[doc = "Register `sf_aes_key_r0_5` writer"]
pub type W = crate::W<SfAesKeyR0_5Spec>;
#[doc = "Field `sf_aes_key_r0_5` reader - "]
pub type SfAesKeyR0_5R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r0_5` writer - "]
pub type SfAesKeyR0_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_5(&self) -> SfAesKeyR0_5R {
        SfAesKeyR0_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_5(&mut self) -> SfAesKeyR0_5W<'_, SfAesKeyR0_5Spec> {
        SfAesKeyR0_5W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r0_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR0_5Spec;
impl crate::RegisterSpec for SfAesKeyR0_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r0_5::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR0_5Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r0_5::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR0_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r0_5 to value 0"]
impl crate::Resettable for SfAesKeyR0_5Spec {}
