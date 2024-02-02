#[doc = "Register `pucr2_hw` reader"]
pub type R = crate::R<Pucr2HwSpec>;
#[doc = "Register `pucr2_hw` writer"]
pub type W = crate::W<Pucr2HwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "pucr2_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr2_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr2_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pucr2HwSpec;
impl crate::RegisterSpec for Pucr2HwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucr2_hw::R`](R) reader structure"]
impl crate::Readable for Pucr2HwSpec {}
#[doc = "`write(|w| ..)` method takes [`pucr2_hw::W`](W) writer structure"]
impl crate::Writable for Pucr2HwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pucr2_hw to value 0"]
impl crate::Resettable for Pucr2HwSpec {}
