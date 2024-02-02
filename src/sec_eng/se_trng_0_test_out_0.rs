#[doc = "Register `se_trng_0_test_out_0` reader"]
pub type R = crate::R<SeTrng0TestOut0Spec>;
#[doc = "Field `se_trng_0_test_out_0` reader - "]
pub type SeTrng0TestOut0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_test_out_0(&self) -> SeTrng0TestOut0R {
        SeTrng0TestOut0R::new(self.bits)
    }
}
#[doc = "se_trng_0_test_out_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0TestOut0Spec;
impl crate::RegisterSpec for SeTrng0TestOut0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_test_out_0::R`](R) reader structure"]
impl crate::Readable for SeTrng0TestOut0Spec {}
#[doc = "`reset()` method sets se_trng_0_test_out_0 to value 0"]
impl crate::Resettable for SeTrng0TestOut0Spec {}
