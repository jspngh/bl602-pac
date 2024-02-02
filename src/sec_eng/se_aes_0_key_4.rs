#[doc = "Register `se_aes_0_key_4` reader"]
pub type R = crate::R<SeAes0Key4Spec>;
#[doc = "Register `se_aes_0_key_4` writer"]
pub type W = crate::W<SeAes0Key4Spec>;
#[doc = "Field `se_aes_0_key_4` reader - "]
pub type SeAes0Key4R = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_key_4` writer - "]
pub type SeAes0Key4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_4(&self) -> SeAes0Key4R {
        SeAes0Key4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_key_4(&mut self) -> SeAes0Key4W<'_, SeAes0Key4Spec> {
        SeAes0Key4W::new(self, 0)
    }
}
#[doc = "se_aes_0_key_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0Key4Spec;
impl crate::RegisterSpec for SeAes0Key4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_key_4::R`](R) reader structure"]
impl crate::Readable for SeAes0Key4Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_key_4::W`](W) writer structure"]
impl crate::Writable for SeAes0Key4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_key_4 to value 0"]
impl crate::Resettable for SeAes0Key4Spec {}
