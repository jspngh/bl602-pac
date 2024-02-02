#[doc = "Register `sf_aes_iv_r2_w0` reader"]
pub type R = crate::R<SfAesIvR2W0Spec>;
#[doc = "Register `sf_aes_iv_r2_w0` writer"]
pub type W = crate::W<SfAesIvR2W0Spec>;
#[doc = "Field `sf_aes_iv_r2_w0` reader - "]
pub type SfAesIvR2W0R = crate::FieldReader<u32>;
#[doc = "Field `sf_aes_iv_r2_w0` writer - "]
pub type SfAesIvR2W0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_iv_r2_w0(&self) -> SfAesIvR2W0R {
        SfAesIvR2W0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_iv_r2_w0(&mut self) -> SfAesIvR2W0W<'_, SfAesIvR2W0Spec> {
        SfAesIvR2W0W::new(self, 0)
    }
}
#[doc = "sf_aes_iv_r2_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r2_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r2_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesIvR2W0Spec;
impl crate::RegisterSpec for SfAesIvR2W0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes_iv_r2_w0::R`](R) reader structure"]
impl crate::Readable for SfAesIvR2W0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes_iv_r2_w0::W`](W) writer structure"]
impl crate::Writable for SfAesIvR2W0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_aes_iv_r2_w0 to value 0"]
impl crate::Resettable for SfAesIvR2W0Spec {}
