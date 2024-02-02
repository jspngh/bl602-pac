#[doc = "Register `TPLCR3` reader"]
pub type R = crate::R<Tplcr3Spec>;
#[doc = "Register `TPLCR3` writer"]
pub type W = crate::W<Tplcr3Spec>;
#[doc = "Field `tplcr` reader - "]
pub type TplcrR = crate::FieldReader;
#[doc = "Field `tplcr` writer - "]
pub type TplcrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&self) -> TplcrR {
        TplcrR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&mut self) -> TplcrW<'_, Tplcr3Spec> {
        TplcrW::new(self, 0)
    }
}
#[doc = "TPLCR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tplcr3Spec;
impl crate::RegisterSpec for Tplcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tplcr3::R`](R) reader structure"]
impl crate::Readable for Tplcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tplcr3::W`](W) writer structure"]
impl crate::Writable for Tplcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPLCR3 to value 0"]
impl crate::Resettable for Tplcr3Spec {}
