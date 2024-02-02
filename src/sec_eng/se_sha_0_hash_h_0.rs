#[doc = "Register `se_sha_0_hash_h_0` reader"]
pub type R = crate::R<SeSha0HashH0Spec>;
#[doc = "Field `se_sha_0_hash_h_0` reader - "]
pub type SeSha0HashH0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_h_0(&self) -> SeSha0HashH0R {
        SeSha0HashH0R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_h_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashH0Spec;
impl crate::RegisterSpec for SeSha0HashH0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_h_0::R`](R) reader structure"]
impl crate::Readable for SeSha0HashH0Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_h_0 to value 0"]
impl crate::Resettable for SeSha0HashH0Spec {}
