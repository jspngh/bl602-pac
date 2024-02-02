#[doc = "Register `se_trng_0_dout_6` reader"]
pub type R = crate::R<SeTrng0Dout6Spec>;
#[doc = "Field `se_trng_0_dout_6` reader - "]
pub type SeTrng0Dout6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_6(&self) -> SeTrng0Dout6R {
        SeTrng0Dout6R::new(self.bits)
    }
}
#[doc = "se_trng_0_dout_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Dout6Spec;
impl crate::RegisterSpec for SeTrng0Dout6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_dout_6::R`](R) reader structure"]
impl crate::Readable for SeTrng0Dout6Spec {}
#[doc = "`reset()` method sets se_trng_0_dout_6 to value 0"]
impl crate::Resettable for SeTrng0Dout6Spec {}
