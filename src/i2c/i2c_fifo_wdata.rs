#[doc = "Register `i2c_fifo_wdata` writer"]
pub type W = crate::W<I2cFifoWdataSpec>;
#[doc = "Field `i2c_fifo_wdata` writer - "]
pub type I2cFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_wdata(&mut self) -> I2cFifoWdataW<'_, I2cFifoWdataSpec> {
        I2cFifoWdataW::new(self, 0)
    }
}
#[doc = "i2c_fifo_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFifoWdataSpec;
impl crate::RegisterSpec for I2cFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for I2cFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2c_fifo_wdata to value 0"]
impl crate::Resettable for I2cFifoWdataSpec {}
