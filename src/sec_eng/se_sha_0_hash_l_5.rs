#[doc = "Register `se_sha_0_hash_l_5` reader"]
pub type R = crate::R<SeSha0HashL5Spec>;
#[doc = "Field `se_sha_0_hash_l_5` reader - "]
pub type SeSha0HashL5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_5(&self) -> SeSha0HashL5R {
        SeSha0HashL5R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_l_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL5Spec;
impl crate::RegisterSpec for SeSha0HashL5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_5::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL5Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_l_5 to value 0"]
impl crate::Resettable for SeSha0HashL5Spec {}
