#[doc = "Register `se_sha_0_hash_l_4` reader"]
pub type R = crate::R<SeSha0HashL4Spec>;
#[doc = "Field `se_sha_0_hash_l_4` reader - "]
pub type SeSha0HashL4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_4(&self) -> SeSha0HashL4R {
        SeSha0HashL4R::new(self.bits)
    }
}
#[doc = "se_sha_0_hash_l_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL4Spec;
impl crate::RegisterSpec for SeSha0HashL4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_4::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL4Spec {}
#[doc = "`reset()` method sets se_sha_0_hash_l_4 to value 0"]
impl crate::Resettable for SeSha0HashL4Spec {}
