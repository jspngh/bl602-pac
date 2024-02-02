#[doc = "Register `pmip_mv2aon` reader"]
pub type R = crate::R<PmipMv2aonSpec>;
#[doc = "Register `pmip_mv2aon` writer"]
pub type W = crate::W<PmipMv2aonSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "pmip_mv2aon.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmip_mv2aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmip_mv2aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmipMv2aonSpec;
impl crate::RegisterSpec for PmipMv2aonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmip_mv2aon::R`](R) reader structure"]
impl crate::Readable for PmipMv2aonSpec {}
#[doc = "`write(|w| ..)` method takes [`pmip_mv2aon::W`](W) writer structure"]
impl crate::Writable for PmipMv2aonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pmip_mv2aon to value 0"]
impl crate::Resettable for PmipMv2aonSpec {}
