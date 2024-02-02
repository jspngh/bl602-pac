#[doc = "Register `sf_aes_key_r2_0` reader"]
pub type R = crate::R<SfAesKeyR2_0Spec>;
#[doc = "Register `sf_aes_key_r2_0` writer"]
pub type W = crate::W<SfAesKeyR2_0Spec>;
#[doc = "Field `sf_aes_key_r2_0` reader - "]
pub type SfAesKeyR2_0R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_key_r2_0` writer - "]
pub type SfAesKeyR2_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_0(&self) -> SfAesKeyR2_0R {
        SfAesKeyR2_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_0(&mut self) -> SfAesKeyR2_0W<'_, SfAesKeyR2_0Spec> {
        SfAesKeyR2_0W::new(self, 0)
    }
}
#[doc = "sf_aes_key_r2_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesKeyR2_0Spec;
impl crate::RegisterSpec for SfAesKeyR2_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_key_r2_0::R`](R) reader structure"]
impl crate::Readable for SfAesKeyR2_0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_key_r2_0::W`](W) writer structure"]
impl crate::Writable for SfAesKeyR2_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_key_r2_0 to value 0"]
impl crate::Resettable for SfAesKeyR2_0Spec {}
