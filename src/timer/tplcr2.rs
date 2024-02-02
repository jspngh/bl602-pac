#[doc = "Register `TPLCR2` reader"]
pub type R = crate::R<Tplcr2Spec>;
#[doc = "Register `TPLCR2` writer"]
pub type W = crate::W<Tplcr2Spec>;
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
    pub fn tplcr(&mut self) -> TplcrW<'_, Tplcr2Spec> {
        TplcrW::new(self, 0)
    }
}
#[doc = "TPLCR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tplcr2Spec;
impl crate::RegisterSpec for Tplcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tplcr2::R`](R) reader structure"]
impl crate::Readable for Tplcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tplcr2::W`](W) writer structure"]
impl crate::Writable for Tplcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPLCR2 to value 0"]
impl crate::Resettable for Tplcr2Spec {}
