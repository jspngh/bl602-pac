#[doc = "Register `i2c_fifo_config_0` reader"]
pub type R = crate::R<I2cFifoConfig0Spec>;
#[doc = "Register `i2c_fifo_config_0` writer"]
pub type W = crate::W<I2cFifoConfig0Spec>;
#[doc = "Field `i2c_dma_tx_en` reader - "]
pub type I2cDmaTxEnR = crate::BitReader;
#[doc = "Field `i2c_dma_tx_en` writer - "]
pub type I2cDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c_dma_rx_en` reader - "]
pub type I2cDmaRxEnR = crate::BitReader;
#[doc = "Field `i2c_dma_rx_en` writer - "]
pub type I2cDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn i2c_dma_tx_en(&self) -> I2cDmaTxEnR {
        I2cDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_dma_rx_en(&self) -> I2cDmaRxEnR {
        I2cDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
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
    pub fn i2c_dma_tx_en(&mut self) -> I2cDmaTxEnW<'_, I2cFifoConfig0Spec> {
        I2cDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_dma_rx_en(&mut self) -> I2cDmaRxEnW<'_, I2cFifoConfig0Spec> {
        I2cDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, I2cFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, I2cFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "i2c_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFifoConfig0Spec;
impl crate::RegisterSpec for I2cFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for I2cFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for I2cFifoConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2c_fifo_config_0 to value 0"]
impl crate::Resettable for I2cFifoConfig0Spec {}
