#[doc = "Register `sf_aes_key_r0_1` reader"]
pub type R = crate::R<SfAesKeyR0_1Spec>;
#[doc = "Register `sf_aes_key_r0_1` writer"]
pub type W = crate::W<SfAesKeyR0_1Spec>;
#[doc = "Field `sf_aes_key_r0_1` reader - "]
pub type SfAesKeyR0_1R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r0_1` writer - "]
pub type SfAesKeyR0_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_1(&self) -> SfAesKeyR0_1R {
        SfAesKeyR0_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_1(&mut self) -> SfAesKeyR0_1W<'_, SfAesKeyR0_1Spec> {
        SfAesKeyR0_1W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r0_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR0_1Spec;
impl crate::RegisterSpec for SfAesKeyR0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r0_1::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r0_1::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r0_1 to value 0"]
impl crate::Resettable for SfAesKeyR0_1Spec {}
