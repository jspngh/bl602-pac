#[doc = "Register `clk_cfg2` reader"]
pub type R = crate::R<ClkCfg2Spec>;
#[doc = "Register `clk_cfg2` writer"]
pub type W = crate::W<ClkCfg2Spec>;
#[doc = "Field `uart_clk_div` reader - "]
pub type UartClkDivR = crate::FieldReader;
#[doc = "Field `uart_clk_div` writer - "]
pub type UartClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uart_clk_en` reader - "]
pub type UartClkEnR = crate::BitReader;
#[doc = "Field `uart_clk_en` writer - "]
pub type UartClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HbnUartClkSelR = crate::BitReader;
#[doc = "Field `sf_clk_div` reader - "]
pub type SfClkDivR = crate::FieldReader;
#[doc = "Field `sf_clk_div` writer - "]
pub type SfClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_clk_en` reader - "]
pub type SfClkEnR = crate::BitReader;
#[doc = "Field `sf_clk_en` writer - "]
pub type SfClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_clk_sel` reader - "]
pub type SfClkSelR = crate::FieldReader;
#[doc = "Field `sf_clk_sel` writer - "]
pub type SfClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_clk_sel2` reader - "]
pub type SfClkSel2R = crate::FieldReader;
#[doc = "Field `sf_clk_sel2` writer - "]
pub type SfClkSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ir_clk_div` reader - "]
pub type IrClkDivR = crate::FieldReader;
#[doc = "Field `ir_clk_div` writer - "]
pub type IrClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ir_clk_en` reader - "]
pub type IrClkEnR = crate::BitReader;
#[doc = "Field `ir_clk_en` writer - "]
pub type IrClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_clk_en` reader - "]
pub type DmaClkEnR = crate::FieldReader;
#[doc = "Field `dma_clk_en` writer - "]
pub type DmaClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UartClkDivR {
        UartClkDivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UartClkEnR {
        UartClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HbnUartClkSelR {
        HbnUartClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SfClkDivR {
        SfClkDivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SfClkEnR {
        SfClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SfClkSelR {
        SfClkSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SfClkSel2R {
        SfClkSel2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IrClkDivR {
        IrClkDivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IrClkEnR {
        IrClkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&mut self) -> UartClkDivW<'_, ClkCfg2Spec> {
        UartClkDivW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UartClkEnW<'_, ClkCfg2Spec> {
        UartClkEnW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&mut self) -> SfClkDivW<'_, ClkCfg2Spec> {
        SfClkDivW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&mut self) -> SfClkEnW<'_, ClkCfg2Spec> {
        SfClkEnW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&mut self) -> SfClkSelW<'_, ClkCfg2Spec> {
        SfClkSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&mut self) -> SfClkSel2W<'_, ClkCfg2Spec> {
        SfClkSel2W::new(self, 14)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&mut self) -> IrClkDivW<'_, ClkCfg2Spec> {
        IrClkDivW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&mut self) -> IrClkEnW<'_, ClkCfg2Spec> {
        IrClkEnW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<'_, ClkCfg2Spec> {
        DmaClkEnW::new(self, 24)
    }
}
#[doc = "Clock configuration for UART and Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCfg2Spec;
impl crate::RegisterSpec for ClkCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg2::R`](R) reader structure"]
impl crate::Readable for ClkCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg2::W`](W) writer structure"]
impl crate::Writable for ClkCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clk_cfg2 to value 0xff8f_2b17"]
impl crate::Resettable for ClkCfg2Spec {
    const RESET_VALUE: u32 = 0xff8f_2b17;
}
