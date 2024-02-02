#[doc = "Register `se_trng_0_dout_3` reader"]
pub type R = crate::R<SeTrng0Dout3Spec>;
#[doc = "Field `se_trng_0_dout_3` reader - "]
pub type SeTrng0Dout3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_3(&self) -> SeTrng0Dout3R {
        SeTrng0Dout3R::new(self.bits)
    }
}
#[doc = "se_trng_0_dout_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Dout3Spec;
impl crate::RegisterSpec for SeTrng0Dout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_dout_3::R`](R) reader structure"]
impl crate::Readable for SeTrng0Dout3Spec {}
#[doc = "`reset()` method sets se_trng_0_dout_3 to value 0"]
impl crate::Resettable for SeTrng0Dout3Spec {}
