#[doc = "Register `TMR3_2` reader"]
pub type R = crate::R<Tmr3_2Spec>;
#[doc = "Register `TMR3_2` writer"]
pub type W = crate::W<Tmr3_2Spec>;
#[doc = "Field `tmr` reader - "]
pub type TmrR = crate::FieldReader<u32>;
#[doc = "Field `tmr` writer - "]
pub type TmrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TmrW<'_, Tmr3_2Spec> {
        TmrW::new(self, 0)
    }
}
#[doc = "TMR3_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr3_2Spec;
impl crate::RegisterSpec for Tmr3_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr3_2::R`](R) reader structure"]
impl crate::Readable for Tmr3_2Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr3_2::W`](W) writer structure"]
impl crate::Writable for Tmr3_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR3_2 to value 0xffff_ffff"]
impl crate::Resettable for Tmr3_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
