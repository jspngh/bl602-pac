#[doc = "Register `spi_fifo_config_1` reader"]
pub type R = crate::R<SpiFifoConfig1Spec>;
#[doc = "Register `spi_fifo_config_1` writer"]
pub type W = crate::W<SpiFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - "]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - "]
pub type TxFifoThR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` writer - "]
pub type TxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rx_fifo_th` reader - "]
pub type RxFifoThR = crate::FieldReader;
#[doc = "Field `rx_fifo_th` writer - "]
pub type RxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RxFifoThR {
        RxFifoThR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, SpiFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RxFifoThW<'_, SpiFifoConfig1Spec> {
        RxFifoThW::new(self, 24)
    }
}
#[doc = "spi_fifo_config_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoConfig1Spec;
impl crate::RegisterSpec for SpiFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for SpiFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for SpiFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_fifo_config_1 to value 0x04"]
impl crate::Resettable for SpiFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x04;
}
