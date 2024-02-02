#[doc = "Register `uart_int_en` reader"]
pub type R = crate::R<UartIntEnSpec>;
#[doc = "Register `uart_int_en` writer"]
pub type W = crate::W<UartIntEnSpec>;
#[doc = "Field `cr_utx_end_en` reader - "]
pub type CrUtxEndEnR = crate::BitReader;
#[doc = "Field `cr_utx_end_en` writer - "]
pub type CrUtxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_end_en` reader - "]
pub type CrUrxEndEnR = crate::BitReader;
#[doc = "Field `cr_urx_end_en` writer - "]
pub type CrUrxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_fifo_en` reader - "]
pub type CrUtxFifoEnR = crate::BitReader;
#[doc = "Field `cr_utx_fifo_en` writer - "]
pub type CrUtxFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_fifo_en` reader - "]
pub type CrUrxFifoEnR = crate::BitReader;
#[doc = "Field `cr_urx_fifo_en` writer - "]
pub type CrUrxFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rto_en` reader - "]
pub type CrUrxRtoEnR = crate::BitReader;
#[doc = "Field `cr_urx_rto_en` writer - "]
pub type CrUrxRtoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_pce_en` reader - "]
pub type CrUrxPceEnR = crate::BitReader;
#[doc = "Field `cr_urx_pce_en` writer - "]
pub type CrUrxPceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_fer_en` reader - "]
pub type CrUtxFerEnR = crate::BitReader;
#[doc = "Field `cr_utx_fer_en` writer - "]
pub type CrUtxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_fer_en` reader - "]
pub type CrUrxFerEnR = crate::BitReader;
#[doc = "Field `cr_urx_fer_en` writer - "]
pub type CrUrxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&self) -> CrUtxEndEnR {
        CrUtxEndEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&self) -> CrUrxEndEnR {
        CrUrxEndEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_en(&self) -> CrUtxFifoEnR {
        CrUtxFifoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_en(&self) -> CrUrxFifoEnR {
        CrUrxFifoEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&self) -> CrUrxRtoEnR {
        CrUrxRtoEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&self) -> CrUrxPceEnR {
        CrUrxPceEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&self) -> CrUtxFerEnR {
        CrUtxFerEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_en(&self) -> CrUrxFerEnR {
        CrUrxFerEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&mut self) -> CrUtxEndEnW<'_, UartIntEnSpec> {
        CrUtxEndEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&mut self) -> CrUrxEndEnW<'_, UartIntEnSpec> {
        CrUrxEndEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_en(&mut self) -> CrUtxFifoEnW<'_, UartIntEnSpec> {
        CrUtxFifoEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_en(&mut self) -> CrUrxFifoEnW<'_, UartIntEnSpec> {
        CrUrxFifoEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&mut self) -> CrUrxRtoEnW<'_, UartIntEnSpec> {
        CrUrxRtoEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&mut self) -> CrUrxPceEnW<'_, UartIntEnSpec> {
        CrUrxPceEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&mut self) -> CrUtxFerEnW<'_, UartIntEnSpec> {
        CrUtxFerEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_en(&mut self) -> CrUrxFerEnW<'_, UartIntEnSpec> {
        CrUrxFerEnW::new(self, 7)
    }
}
#[doc = "UART interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntEnSpec;
impl crate::RegisterSpec for UartIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_en::R`](R) reader structure"]
impl crate::Readable for UartIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_int_en::W`](W) writer structure"]
impl crate::Writable for UartIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_int_en to value 0xff"]
impl crate::Resettable for UartIntEnSpec {
    const RESET_VALUE: u32 = 0xff;
}
