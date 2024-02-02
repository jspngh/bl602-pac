#[doc = "Register `se_aes_0_key_3` reader"]
pub type R = crate::R<SeAes0Key3Spec>;
#[doc = "Register `se_aes_0_key_3` writer"]
pub type W = crate::W<SeAes0Key3Spec>;
#[doc = "Field `se_aes_0_key_3` reader - "]
pub type SeAes0Key3R = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_key_3` writer - "]
pub type SeAes0Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_3(&self) -> SeAes0Key3R {
        SeAes0Key3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_3(&mut self) -> SeAes0Key3W<'_, SeAes0Key3Spec> {
        SeAes0Key3W::new(self, 0)
    }
}
#[doc = "se_aes_0_key_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0Key3Spec;
impl crate::RegisterSpec for SeAes0Key3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_key_3::R`](R) reader structure"]
impl crate::Readable for SeAes0Key3Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_key_3::W`](W) writer structure"]
impl crate::Writable for SeAes0Key3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_key_3 to value 0"]
impl crate::Resettable for SeAes0Key3Spec {}
