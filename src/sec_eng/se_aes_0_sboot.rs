#[doc = "Register `se_aes_0_sboot` reader"]
pub type R = crate::R<SeAes0SbootSpec>;
#[doc = "Register `se_aes_0_sboot` writer"]
pub type W = crate::W<SeAes0SbootSpec>;
#[doc = "Field `se_aes_0_sboot_key_sel` reader - "]
pub type SeAes0SbootKeySelR = crate::BitReader;
#[doc = "Field `se_aes_0_sboot_key_sel` writer - "]
pub type SeAes0SbootKeySelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_sboot_key_sel(&self) -> SeAes0SbootKeySelR {
        SeAes0SbootKeySelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_sboot_key_sel(&mut self) -> SeAes0SbootKeySelW<'_, SeAes0SbootSpec> {
        SeAes0SbootKeySelW::new(self, 0)
    }
}
#[doc = "se_aes_0_sboot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_sboot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_sboot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0SbootSpec;
impl crate::RegisterSpec for SeAes0SbootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_sboot::R`](R) reader structure"]
impl crate::Readable for SeAes0SbootSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_sboot::W`](W) writer structure"]
impl crate::Writable for SeAes0SbootSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_sboot to value 0"]
impl crate::Resettable for SeAes0SbootSpec {}
