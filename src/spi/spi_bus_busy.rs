#[doc = "Register `spi_bus_busy` reader"]
pub type R = crate::R<SpiBusBusySpec>;
#[doc = "Field `sts_spi_bus_busy` reader - "]
pub type StsSpiBusBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&self) -> StsSpiBusBusyR {
        StsSpiBusBusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "spi_bus_busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_bus_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiBusBusySpec;
impl crate::RegisterSpec for SpiBusBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_bus_busy::R`](R) reader structure"]
impl crate::Readable for SpiBusBusySpec {}
#[doc = "`reset()` method sets spi_bus_busy to value 0"]
impl crate::Resettable for SpiBusBusySpec {}
