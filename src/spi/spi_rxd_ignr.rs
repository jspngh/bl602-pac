#[doc = "Register `spi_rxd_ignr` reader"]
pub type R = crate::R<SpiRxdIgnrSpec>;
#[doc = "Register `spi_rxd_ignr` writer"]
pub type W = crate::W<SpiRxdIgnrSpec>;
#[doc = "Field `cr_spi_rxd_ignr_p` reader - "]
pub type CrSpiRxdIgnrPR = crate::FieldReader;
#[doc = "Field `cr_spi_rxd_ignr_p` writer - "]
pub type CrSpiRxdIgnrPW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cr_spi_rxd_ignr_s` reader - "]
pub type CrSpiRxdIgnrSR = crate::FieldReader;
#[doc = "Field `cr_spi_rxd_ignr_s` writer - "]
pub type CrSpiRxdIgnrSW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&self) -> CrSpiRxdIgnrPR {
        CrSpiRxdIgnrPR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&self) -> CrSpiRxdIgnrSR {
        CrSpiRxdIgnrSR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&mut self) -> CrSpiRxdIgnrPW<'_, SpiRxdIgnrSpec> {
        CrSpiRxdIgnrPW::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&mut self) -> CrSpiRxdIgnrSW<'_, SpiRxdIgnrSpec> {
        CrSpiRxdIgnrSW::new(self, 16)
    }
}
#[doc = "spi_rxd_ignr.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_rxd_ignr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_rxd_ignr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiRxdIgnrSpec;
impl crate::RegisterSpec for SpiRxdIgnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rxd_ignr::R`](R) reader structure"]
impl crate::Readable for SpiRxdIgnrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_rxd_ignr::W`](W) writer structure"]
impl crate::Writable for SpiRxdIgnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_rxd_ignr to value 0"]
impl crate::Resettable for SpiRxdIgnrSpec {}
