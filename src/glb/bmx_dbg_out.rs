#[doc = "Register `bmx_dbg_out` reader"]
pub type R = crate::R<BmxDbgOutSpec>;
#[doc = "Field `bmx_dbg_out` reader - "]
pub type BmxDbgOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&self) -> BmxDbgOutR {
        BmxDbgOutR::new(self.bits)
    }
}
#[doc = "bmx_dbg_out.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_dbg_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxDbgOutSpec;
impl crate::RegisterSpec for BmxDbgOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_dbg_out::R`](R) reader structure"]
impl crate::Readable for BmxDbgOutSpec {}
#[doc = "`reset()` method sets bmx_dbg_out to value 0"]
impl crate::Resettable for BmxDbgOutSpec {}
