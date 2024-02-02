#[doc = "Register `se_sha_0_hash_l_7` reader"]
pub type R = crate::R<SeSha0HashL7Spec>;
#[doc = "Field `se_sha_0_hash_l_7` reader - "]
pub type SeSha0HashL7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_7(&self) -> SeSha0HashL7R {
        SeSha0HashL7R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_l_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL7Spec;
impl crate::RegisterSpec for SeSha0HashL7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_7::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL7Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_l_7 to value 0"]
impl crate::Resettable for SeSha0HashL7Spec {}
