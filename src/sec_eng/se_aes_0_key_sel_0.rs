#[doc = "Register `se_aes_0_key_sel_0` reader"]
pub type R = crate::R<SeAes0KeySel0Spec>;
#[doc = "Register `se_aes_0_key_sel_0` writer"]
pub type W = crate::W<SeAes0KeySel0Spec>;
#[doc = "Field `se_aes_0_key_sel_0` reader - "]
pub type SeAes0KeySel0R = crate::FieldReader;
#[doc = "Field `se_aes_0_key_sel_0` writer - "]
pub type SeAes0KeySel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_0(&self) -> SeAes0KeySel0R {
        SeAes0KeySel0R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_0(&mut self) -> SeAes0KeySel0W<'_, SeAes0KeySel0Spec> {
        SeAes0KeySel0W::new(self, 0)
    }
}
#[doc = "se_aes_0_key_sel_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_sel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_sel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0KeySel0Spec;
impl crate::RegisterSpec for SeAes0KeySel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_key_sel_0::R`](R) reader structure"]
impl crate::Readable for SeAes0KeySel0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_key_sel_0::W`](W) writer structure"]
impl crate::Writable for SeAes0KeySel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_key_sel_0 to value 0"]
impl crate::Resettable for SeAes0KeySel0Spec {}
