#[doc = "Register `se_sha_0_hash_l_3` reader"]
pub type R = crate::R<SeSha0HashL3Spec>;
#[doc = "Field `se_sha_0_hash_l_3` reader - "]
pub type SeSha0HashL3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_3(&self) -> SeSha0HashL3R {
        SeSha0HashL3R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_l_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL3Spec;
impl crate::RegisterSpec for SeSha0HashL3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_3::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL3Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_l_3 to value 0"]
impl crate::Resettable for SeSha0HashL3Spec {}
