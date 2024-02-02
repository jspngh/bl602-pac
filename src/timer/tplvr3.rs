#[doc = "Register `TPLVR3` reader"]
pub type R = crate::R<Tplvr3Spec>;
#[doc = "Register `TPLVR3` writer"]
pub type W = crate::W<Tplvr3Spec>;
#[doc = "Field `tplvr` reader - "]
pub type TplvrR = crate::FieldReader<u32>;
#[doc = "Field `tplvr` writer - "]
pub type TplvrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr(&self) -> TplvrR {
        TplvrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr(&mut self) -> TplvrW<'_, Tplvr3Spec> {
        TplvrW::new(self, 0)
    }
}
#[doc = "TPLVR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tplvr3Spec;
impl crate::RegisterSpec for Tplvr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tplvr3::R`](R) reader structure"]
impl crate::Readable for Tplvr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tplvr3::W`](W) writer structure"]
impl crate::Writable for Tplvr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPLVR3 to value 0"]
impl crate::Resettable for Tplvr3Spec {}
