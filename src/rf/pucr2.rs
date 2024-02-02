#[doc = "Register `pucr2` reader"]
pub type R = crate::R<Pucr2Spec>;
#[doc = "Register `pucr2` writer"]
pub type W = crate::W<Pucr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "pucr2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pucr2Spec;
impl crate::RegisterSpec for Pucr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucr2::R`](R) reader structure"]
impl crate::Readable for Pucr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pucr2::W`](W) writer structure"]
impl crate::Writable for Pucr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pucr2 to value 0"]
impl crate::Resettable for Pucr2Spec {}
