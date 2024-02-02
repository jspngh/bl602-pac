#[doc = "Register `bmx_err_addr` reader"]
pub type R = crate::R<BmxErrAddrSpec>;
#[doc = "Field `bmx_err_addr` reader - "]
pub type BmxErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_err_addr(&self) -> BmxErrAddrR {
        BmxErrAddrR::new(self.bits)
    }
}
#[doc = "bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxErrAddrSpec;
impl crate::RegisterSpec for BmxErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_err_addr::R`](R) reader structure"]
impl crate::Readable for BmxErrAddrSpec {}
#[doc = "`reset()` method sets bmx_err_addr to value 0"]
impl crate::Resettable for BmxErrAddrSpec {}
