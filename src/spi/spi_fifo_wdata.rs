#[doc = "Register `spi_fifo_wdata` writer"]
pub type W = crate::W<SpiFifoWdataSpec>;
#[doc = "Field `spi_fifo_wdata` writer - "]
pub type SpiFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_wdata(&mut self) -> SpiFifoWdataW<'_, SpiFifoWdataSpec> {
        SpiFifoWdataW::new(self, 0)
    }
}
#[doc = "spi_fifo_wdata.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoWdataSpec;
impl crate::RegisterSpec for SpiFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for SpiFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_fifo_wdata to value 0"]
impl crate::Resettable for SpiFifoWdataSpec {}
