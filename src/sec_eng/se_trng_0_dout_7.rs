#[doc = "Register `se_trng_0_dout_7` reader"]
pub type R = crate::R<SeTrng0Dout7Spec>;
#[doc = "Field `se_trng_0_dout_7` reader - "]
pub type SeTrng0Dout7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_7(&self) -> SeTrng0Dout7R {
        SeTrng0Dout7R::new(self.bits)
    }
}
#[doc = "se_trng_0_dout_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Dout7Spec;
impl crate::RegisterSpec for SeTrng0Dout7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_dout_7::R`](R) reader structure"]
impl crate::Readable for SeTrng0Dout7Spec {}
#[doc = "`reset()` method sets se_trng_0_dout_7 to value 0"]
impl crate::Resettable for SeTrng0Dout7Spec {}
