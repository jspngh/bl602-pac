#[doc = "Register `se_aes_0_key_1` reader"]
pub type R = crate::R<SeAes0Key1Spec>;
#[doc = "Register `se_aes_0_key_1` writer"]
pub type W = crate::W<SeAes0Key1Spec>;
#[doc = "Field `se_aes_0_key_1` reader - "]
pub type SeAes0Key1R = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_key_1` writer - "]
pub type SeAes0Key1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_1(&self) -> SeAes0Key1R {
        SeAes0Key1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_1(&mut self) -> SeAes0Key1W<'_, SeAes0Key1Spec> {
        SeAes0Key1W::new(self, 0)
    }
}
#[doc = "se_aes_0_key_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0Key1Spec;
impl crate::RegisterSpec for SeAes0Key1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_key_1::R`](R) reader structure"]
impl crate::Readable for SeAes0Key1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_key_1::W`](W) writer structure"]
impl crate::Writable for SeAes0Key1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_key_1 to value 0"]
impl crate::Resettable for SeAes0Key1Spec {}
