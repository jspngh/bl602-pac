#[doc = "Register `se_aes_0_key_sel_1` reader"]
pub type R = crate::R<SeAes0KeySel1Spec>;
#[doc = "Register `se_aes_0_key_sel_1` writer"]
pub type W = crate::W<SeAes0KeySel1Spec>;
#[doc = "Field `se_aes_0_key_sel_1` reader - "]
pub type SeAes0KeySel1R = crate::FieldReader;
#[doc = "Field `se_aes_0_key_sel_1` writer - "]
pub type SeAes0KeySel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_1(&self) -> SeAes0KeySel1R {
        SeAes0KeySel1R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_1(&mut self) -> SeAes0KeySel1W<'_, SeAes0KeySel1Spec> {
        SeAes0KeySel1W::new(self, 0)
    }
}
#[doc = "se_aes_0_key_sel_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_sel_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_sel_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0KeySel1Spec;
impl crate::RegisterSpec for SeAes0KeySel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_key_sel_1::R`](R) reader structure"]
impl crate::Readable for SeAes0KeySel1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_key_sel_1::W`](W) writer structure"]
impl crate::Writable for SeAes0KeySel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_aes_0_key_sel_1 to value 0"]
impl crate::Resettable for SeAes0KeySel1Spec {}
