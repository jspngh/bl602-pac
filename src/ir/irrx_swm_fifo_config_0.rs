#[doc = "Register `irrx_swm_fifo_config_0` reader"]
pub type R = crate::R<IrrxSwmFifoConfig0Spec>;
#[doc = "Register `irrx_swm_fifo_config_0` writer"]
pub type W = crate::W<IrrxSwmFifoConfig0Spec>;
#[doc = "Field `rx_fifo_clr` reader - "]
pub type RxFifoClrR = crate::BitReader;
#[doc = "Field `rx_fifo_clr` writer - "]
pub type RxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RxFifoCntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RxFifoClrR {
        RxFifoClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RxFifoUnderflowR {
        RxFifoUnderflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, IrrxSwmFifoConfig0Spec> {
        RxFifoClrW::new(self, 0)
    }
}
#[doc = "irrx_swm_fifo_config_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_swm_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_swm_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxSwmFifoConfig0Spec;
impl crate::RegisterSpec for IrrxSwmFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_swm_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for IrrxSwmFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`irrx_swm_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for IrrxSwmFifoConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_swm_fifo_config_0 to value 0"]
impl crate::Resettable for IrrxSwmFifoConfig0Spec {}
