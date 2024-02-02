#[doc = "Register `i2c_fifo_rdata` reader"]
pub type R = crate::R<I2cFifoRdataSpec>;
#[doc = "Field `i2c_fifo_rdata` reader - "]
pub type I2cFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2cFifoRdataR {
        I2cFifoRdataR::new(self.bits)
    }
}
#[doc = "i2c_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFifoRdataSpec;
impl crate::RegisterSpec for I2cFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for I2cFifoRdataSpec {}
#[doc = "`reset()` method sets i2c_fifo_rdata to value 0"]
impl crate::Resettable for I2cFifoRdataSpec {}
