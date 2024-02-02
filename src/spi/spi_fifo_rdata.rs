#[doc = "Register `spi_fifo_rdata` reader"]
pub type R = crate::R<SpiFifoRdataSpec>;
#[doc = "Field `spi_fifo_rdata` reader - "]
pub type SpiFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SpiFifoRdataR {
        SpiFifoRdataR::new(self.bits)
    }
}
#[doc = "spi_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoRdataSpec;
impl crate::RegisterSpec for SpiFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for SpiFifoRdataSpec {}
#[doc = "`reset()` method sets spi_fifo_rdata to value 0"]
impl crate::Resettable for SpiFifoRdataSpec {}
