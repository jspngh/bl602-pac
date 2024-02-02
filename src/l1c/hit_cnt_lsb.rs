#[doc = "Register `hit_cnt_lsb` reader"]
pub type R = crate::R<HitCntLsbSpec>;
#[doc = "Field `hit_cnt_lsb` reader - "]
pub type HitCntLsbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&self) -> HitCntLsbR {
        HitCntLsbR::new(self.bits)
    }
}
#[doc = "hit_cnt_lsb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_lsb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HitCntLsbSpec;
impl crate::RegisterSpec for HitCntLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hit_cnt_lsb::R`](R) reader structure"]
impl crate::Readable for HitCntLsbSpec {}
#[doc = "`reset()` method sets hit_cnt_lsb to value 0"]
impl crate::Resettable for HitCntLsbSpec {}
