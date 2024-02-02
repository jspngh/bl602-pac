#[doc = "Register `se_sha_0_hash_l_2` reader"]
pub type R = crate::R<SeSha0HashL2Spec>;
#[doc = "Field `se_sha_0_hash_l_2` reader - "]
pub type SeSha0HashL2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_2(&self) -> SeSha0HashL2R {
        SeSha0HashL2R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_l_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL2Spec;
impl crate::RegisterSpec for SeSha0HashL2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_2::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL2Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_l_2 to value 0"]
impl crate::Resettable for SeSha0HashL2Spec {}
