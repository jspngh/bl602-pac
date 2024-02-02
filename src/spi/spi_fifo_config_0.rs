#[doc = "Register `spi_fifo_config_0` reader"]
pub type R = crate::R<SpiFifoConfig0Spec>;
#[doc = "Register `spi_fifo_config_0` writer"]
pub type W = crate::W<SpiFifoConfig0Spec>;
#[doc = "Field `spi_dma_tx_en` reader - "]
pub type SpiDmaTxEnR = crate::BitReader;
#[doc = "Field `spi_dma_tx_en` writer - "]
pub type SpiDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi_dma_rx_en` reader - "]
pub type SpiDmaRxEnR = crate::BitReader;
#[doc = "Field `spi_dma_rx_en` writer - "]
pub type SpiDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` reader - "]
pub type TxFifoClrR = crate::BitReader;
#[doc = "Field `tx_fifo_clr` writer - "]
pub type TxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_clr` reader - "]
pub type RxFifoClrR = crate::BitReader;
#[doc = "Field `rx_fifo_clr` writer - "]
pub type RxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - "]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - "]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RxFifoUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&self) -> SpiDmaTxEnR {
        SpiDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&self) -> SpiDmaRxEnR {
        SpiDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TxFifoClrR {
        TxFifoClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RxFifoClrR {
        RxFifoClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TxFifoOverflowR {
        TxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RxFifoUnderflowR {
        RxFifoUnderflowR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&mut self) -> SpiDmaTxEnW<'_, SpiFifoConfig0Spec> {
        SpiDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&mut self) -> SpiDmaRxEnW<'_, SpiFifoConfig0Spec> {
        SpiDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, SpiFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, SpiFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "spi_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoConfig0Spec;
impl crate::RegisterSpec for SpiFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for SpiFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for SpiFifoConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_fifo_config_0 to value 0"]
impl crate::Resettable for SpiFifoConfig0Spec {}
