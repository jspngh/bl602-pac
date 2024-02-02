#[doc = "Register `hit_cnt_msb` reader"]
pub type R = crate::R<HitCntMsbSpec>;
#[doc = "Field `hit_cnt_msb` reader - "]
pub type HitCntMsbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&self) -> HitCntMsbR {
        HitCntMsbR::new(self.bits)
    }
}
#[doc = "hit_cnt_msb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_msb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HitCntMsbSpec;
impl crate::RegisterSpec for HitCntMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hit_cnt_msb::R`](R) reader structure"]
impl crate::Readable for HitCntMsbSpec {}
#[doc = "`reset()` method sets hit_cnt_msb to value 0"]
impl crate::Resettable for HitCntMsbSpec {}
