#[doc = "Register `gpadc_config` reader"]
pub type R = crate::R<GpadcConfigSpec>;
#[doc = "Register `gpadc_config` writer"]
pub type W = crate::W<GpadcConfigSpec>;
#[doc = "Field `gpadc_dma_en` reader - "]
pub type GpadcDmaEnR = crate::BitReader;
#[doc = "Field `gpadc_dma_en` writer - "]
pub type GpadcDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_clr` reader - "]
pub type GpadcFifoClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_clr` writer - "]
pub type GpadcFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_ne` reader - "]
pub type GpadcFifoNeR = crate::BitReader;
#[doc = "Field `gpadc_fifo_full` reader - "]
pub type GpadcFifoFullR = crate::BitReader;
#[doc = "Field `gpadc_rdy` reader - "]
pub type GpadcRdyR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun` reader - "]
pub type GpadcFifoOverrunR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun` reader - "]
pub type GpadcFifoUnderrunR = crate::BitReader;
#[doc = "Field `gpadc_rdy_clr` reader - "]
pub type GpadcRdyClrR = crate::BitReader;
#[doc = "Field `gpadc_rdy_clr` writer - "]
pub type GpadcRdyClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_overrun_clr` reader - "]
pub type GpadcFifoOverrunClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun_clr` writer - "]
pub type GpadcFifoOverrunClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_underrun_clr` reader - "]
pub type GpadcFifoUnderrunClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun_clr` writer - "]
pub type GpadcFifoUnderrunClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_rdy_mask` reader - "]
pub type GpadcRdyMaskR = crate::BitReader;
#[doc = "Field `gpadc_rdy_mask` writer - "]
pub type GpadcRdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_overrun_mask` reader - "]
pub type GpadcFifoOverrunMaskR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun_mask` writer - "]
pub type GpadcFifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_underrun_mask` reader - "]
pub type GpadcFifoUnderrunMaskR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun_mask` writer - "]
pub type GpadcFifoUnderrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_data_count` reader - "]
pub type GpadcFifoDataCountR = crate::FieldReader;
#[doc = "Field `gpadc_fifo_thl` reader - "]
pub type GpadcFifoThlR = crate::FieldReader;
#[doc = "Field `gpadc_fifo_thl` writer - "]
pub type GpadcFifoThlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rsvd_31_24` reader - "]
pub type Rsvd31_24R = crate::FieldReader;
#[doc = "Field `rsvd_31_24` writer - "]
pub type Rsvd31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GpadcDmaEnR {
        GpadcDmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&self) -> GpadcFifoClrR {
        GpadcFifoClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GpadcFifoNeR {
        GpadcFifoNeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GpadcFifoFullR {
        GpadcFifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GpadcRdyR {
        GpadcRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GpadcFifoOverrunR {
        GpadcFifoOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GpadcFifoUnderrunR {
        GpadcFifoUnderrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GpadcRdyClrR {
        GpadcRdyClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GpadcFifoOverrunClrR {
        GpadcFifoOverrunClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&self) -> GpadcFifoUnderrunClrR {
        GpadcFifoUnderrunClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&self) -> GpadcRdyMaskR {
        GpadcRdyMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&self) -> GpadcFifoOverrunMaskR {
        GpadcFifoOverrunMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&self) -> GpadcFifoUnderrunMaskR {
        GpadcFifoUnderrunMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&self) -> GpadcFifoDataCountR {
        GpadcFifoDataCountR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&self) -> GpadcFifoThlR {
        GpadcFifoThlR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> Rsvd31_24R {
        Rsvd31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&mut self) -> GpadcDmaEnW<'_, GpadcConfigSpec> {
        GpadcDmaEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&mut self) -> GpadcFifoClrW<'_, GpadcConfigSpec> {
        GpadcFifoClrW::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&mut self) -> GpadcRdyClrW<'_, GpadcConfigSpec> {
        GpadcRdyClrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GpadcFifoOverrunClrW<'_, GpadcConfigSpec> {
        GpadcFifoOverrunClrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&mut self) -> GpadcFifoUnderrunClrW<'_, GpadcConfigSpec> {
        GpadcFifoUnderrunClrW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&mut self) -> GpadcRdyMaskW<'_, GpadcConfigSpec> {
        GpadcRdyMaskW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&mut self) -> GpadcFifoOverrunMaskW<'_, GpadcConfigSpec> {
        GpadcFifoOverrunMaskW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&mut self) -> GpadcFifoUnderrunMaskW<'_, GpadcConfigSpec> {
        GpadcFifoUnderrunMaskW::new(self, 14)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&mut self) -> GpadcFifoThlW<'_, GpadcConfigSpec> {
        GpadcFifoThlW::new(self, 22)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> Rsvd31_24W<'_, GpadcConfigSpec> {
        Rsvd31_24W::new(self, 24)
    }
}
#[doc = "gpadc_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcConfigSpec;
impl crate::RegisterSpec for GpadcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_config::R`](R) reader structure"]
impl crate::Readable for GpadcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_config::W`](W) writer structure"]
impl crate::Writable for GpadcConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GpadcConfigSpec {}
