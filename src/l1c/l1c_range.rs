#[doc = "Register `l1c_range` reader"]
pub type R = crate::R<L1cRangeSpec>;
#[doc = "Register `l1c_range` writer"]
pub type W = crate::W<L1cRangeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "l1c_range.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_range::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_range::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cRangeSpec;
impl crate::RegisterSpec for L1cRangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1c_range::R`](R) reader structure"]
impl crate::Readable for L1cRangeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1c_range::W`](W) writer structure"]
impl crate::Writable for L1cRangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets l1c_range to value 0"]
impl crate::Resettable for L1cRangeSpec {}
