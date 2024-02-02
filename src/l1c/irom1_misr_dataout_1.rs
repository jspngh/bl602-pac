#[doc = "Register `irom1_misr_dataout_1` reader"]
pub type R = crate::R<Irom1MisrDataout1Spec>;
#[doc = "Register `irom1_misr_dataout_1` writer"]
pub type W = crate::W<Irom1MisrDataout1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "irom1_misr_dataout_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irom1_misr_dataout_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irom1MisrDataout1Spec;
impl crate::RegisterSpec for Irom1MisrDataout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irom1_misr_dataout_1::R`](R) reader structure"]
impl crate::Readable for Irom1MisrDataout1Spec {}
#[doc = "`write(|w| ..)` method takes [`irom1_misr_dataout_1::W`](W) writer structure"]
impl crate::Writable for Irom1MisrDataout1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irom1_misr_dataout_1 to value 0"]
impl crate::Resettable for Irom1MisrDataout1Spec {}
