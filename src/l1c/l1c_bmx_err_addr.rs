#[doc = "Register `l1c_bmx_err_addr` reader"]
pub type R = crate::R<L1cBmxErrAddrSpec>;
#[doc = "Field `l1c_bmx_err_addr` reader - "]
pub type L1cBmxErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&self) -> L1cBmxErrAddrR {
        L1cBmxErrAddrR::new(self.bits)
    }
}
#[doc = "l1c_bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_bmx_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cBmxErrAddrSpec;
impl crate::RegisterSpec for L1cBmxErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1c_bmx_err_addr::R`](R) reader structure"]
impl crate::Readable for L1cBmxErrAddrSpec {}
#[doc = "`reset()` method sets l1c_bmx_err_addr to value 0"]
impl crate::Resettable for L1cBmxErrAddrSpec {}
