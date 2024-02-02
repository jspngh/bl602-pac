#[doc = "Register `se_trng_0_dout_4` reader"]
pub type R = crate::R<SeTrng0Dout4Spec>;
#[doc = "Field `se_trng_0_dout_4` reader - "]
pub type SeTrng0Dout4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_4(&self) -> SeTrng0Dout4R {
        SeTrng0Dout4R::new(self.bits)
    }
}
#[doc = "se_trng_0_dout_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Dout4Spec;
impl crate::RegisterSpec for SeTrng0Dout4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_dout_4::R`](R) reader structure"]
impl crate::Readable for SeTrng0Dout4Spec {}
#[doc = "`reset()` method sets se_trng_0_dout_4 to value 0"]
impl crate::Resettable for SeTrng0Dout4Spec {}
