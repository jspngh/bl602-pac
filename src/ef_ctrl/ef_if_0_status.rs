#[doc = "Register `ef_if_0_status` reader"]
pub type R = crate::R<EfIf0StatusSpec>;
#[doc = "Register `ef_if_0_status` writer"]
pub type W = crate::W<EfIf0StatusSpec>;
#[doc = "Field `ef_if_0_status` reader - "]
pub type EfIf0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_0_status(&self) -> EfIf0StatusR {
        EfIf0StatusR::new(self.bits)
    }
}
impl W {}
#[doc = "ef_if_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIf0StatusSpec;
impl crate::RegisterSpec for EfIf0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_0_status::R`](R) reader structure"]
impl crate::Readable for EfIf0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_0_status::W`](W) writer structure"]
impl crate::Writable for EfIf0StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_0_status to value 0xe400"]
impl crate::Resettable for EfIf0StatusSpec {
    const RESET_VALUE: u32 = 0xe400;
}
