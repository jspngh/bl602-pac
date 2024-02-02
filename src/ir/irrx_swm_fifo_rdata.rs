#[doc = "Register `irrx_swm_fifo_rdata` reader"]
pub type R = crate::R<IrrxSwmFifoRdataSpec>;
#[doc = "Field `rx_fifo_rdata` reader - "]
pub type RxFifoRdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RxFifoRdataR {
        RxFifoRdataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "irrx_swm_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_swm_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxSwmFifoRdataSpec;
impl crate::RegisterSpec for IrrxSwmFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_swm_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for IrrxSwmFifoRdataSpec {}
#[doc = "`reset()` method sets irrx_swm_fifo_rdata to value 0"]
impl crate::Resettable for IrrxSwmFifoRdataSpec {}
