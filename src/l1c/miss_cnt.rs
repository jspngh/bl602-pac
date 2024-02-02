#[doc = "Register `miss_cnt` reader"]
pub type R = crate::R<MissCntSpec>;
#[doc = "Field `miss_cnt` reader - "]
pub type MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&self) -> MissCntR {
        MissCntR::new(self.bits)
    }
}
#[doc = "miss_cnt.\n\nYou can [`read`](crate::Reg::read) this register and get [`miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MissCntSpec;
impl crate::RegisterSpec for MissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miss_cnt::R`](R) reader structure"]
impl crate::Readable for MissCntSpec {}
#[doc = "`reset()` method sets miss_cnt to value 0"]
impl crate::Resettable for MissCntSpec {}
