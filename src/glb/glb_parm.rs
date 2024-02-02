#[doc = "Register `glb_parm` reader"]
pub type R = crate::R<GlbParmSpec>;
#[doc = "Register `glb_parm` writer"]
pub type W = crate::W<GlbParmSpec>;
#[doc = "Field `reg_bd_en` reader - "]
pub type RegBdEnR = crate::BitReader;
#[doc = "Field `reg_bd_en` writer - "]
pub type RegBdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ext_rst_smt` reader - "]
pub type RegExtRstSmtR = crate::BitReader;
#[doc = "Field `reg_ext_rst_smt` writer - "]
pub type RegExtRstSmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jtag_swap_set` reader - "]
pub type JtagSwapSetR = crate::FieldReader;
#[doc = "Field `jtag_swap_set` writer - "]
pub type JtagSwapSetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `swap_sflash_io_3_io_0` reader - "]
pub type SwapSflashIo3Io0R = crate::BitReader;
#[doc = "Field `swap_sflash_io_3_io_0` writer - "]
pub type SwapSflashIo3Io0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_embedded_sflash` reader - "]
pub type SelEmbeddedSflashR = crate::BitReader;
#[doc = "Field `sel_embedded_sflash` writer - "]
pub type SelEmbeddedSflashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub type RegSpi0MasterModeR = crate::BitReader;
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub type RegSpi0MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_swap` reader - "]
pub type RegSpi0SwapR = crate::BitReader;
#[doc = "Field `reg_spi_0_swap` writer - "]
pub type RegSpi0SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cci_use_jtag_pin` reader - "]
pub type RegCciUseJtagPinR = crate::BitReader;
#[doc = "Field `reg_cci_use_jtag_pin` writer - "]
pub type RegCciUseJtagPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cci_use_sdio_pin` reader - "]
pub type RegCciUseSdioPinR = crate::BitReader;
#[doc = "Field `reg_cci_use_sdio_pin` writer - "]
pub type RegCciUseSdioPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p1_adc_test_with_cci` reader - "]
pub type P1AdcTestWithCciR = crate::BitReader;
#[doc = "Field `p1_adc_test_with_cci` writer - "]
pub type P1AdcTestWithCciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p2_dac_test_with_cci` reader - "]
pub type P2DacTestWithCciR = crate::BitReader;
#[doc = "Field `p2_dac_test_with_cci` writer - "]
pub type P2DacTestWithCciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p3_cci_use_io_2_5` reader - "]
pub type P3CciUseIo2_5R = crate::BitReader;
#[doc = "Field `p3_cci_use_io_2_5` writer - "]
pub type P3CciUseIo2_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p4_adc_test_with_jtag` reader - "]
pub type P4AdcTestWithJtagR = crate::BitReader;
#[doc = "Field `p4_adc_test_with_jtag` writer - "]
pub type P4AdcTestWithJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p5_dac_test_with_jtag` reader - "]
pub type P5DacTestWithJtagR = crate::BitReader;
#[doc = "Field `p5_dac_test_with_jtag` writer - "]
pub type P5DacTestWithJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p6_sdio_use_io_0_5` reader - "]
pub type P6SdioUseIo0_5R = crate::BitReader;
#[doc = "Field `p6_sdio_use_io_0_5` writer - "]
pub type P6SdioUseIo0_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p7_jtag_use_io_2_5` reader - "]
pub type P7JtagUseIo2_5R = crate::BitReader;
#[doc = "Field `p7_jtag_use_io_2_5` writer - "]
pub type P7JtagUseIo2_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart_swap_set` reader - "]
pub type UartSwapSetR = crate::FieldReader;
#[doc = "Field `uart_swap_set` writer - "]
pub type UartSwapSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&self) -> RegBdEnR {
        RegBdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&self) -> RegExtRstSmtR {
        RegExtRstSmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&self) -> JtagSwapSetR {
        JtagSwapSetR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SwapSflashIo3Io0R {
        SwapSflashIo3Io0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SelEmbeddedSflashR {
        SelEmbeddedSflashR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> RegSpi0MasterModeR {
        RegSpi0MasterModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> RegSpi0SwapR {
        RegSpi0SwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&self) -> RegCciUseJtagPinR {
        RegCciUseJtagPinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&self) -> RegCciUseSdioPinR {
        RegCciUseSdioPinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1AdcTestWithCciR {
        P1AdcTestWithCciR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2DacTestWithCciR {
        P2DacTestWithCciR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&self) -> P3CciUseIo2_5R {
        P3CciUseIo2_5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4AdcTestWithJtagR {
        P4AdcTestWithJtagR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5DacTestWithJtagR {
        P5DacTestWithJtagR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&self) -> P6SdioUseIo0_5R {
        P6SdioUseIo0_5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&self) -> P7JtagUseIo2_5R {
        P7JtagUseIo2_5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UartSwapSetR {
        UartSwapSetR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&mut self) -> RegBdEnW<'_, GlbParmSpec> {
        RegBdEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&mut self) -> RegExtRstSmtW<'_, GlbParmSpec> {
        RegExtRstSmtW::new(self, 1)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&mut self) -> JtagSwapSetW<'_, GlbParmSpec> {
        JtagSwapSetW::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SwapSflashIo3Io0W<'_, GlbParmSpec> {
        SwapSflashIo3Io0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&mut self) -> SelEmbeddedSflashW<'_, GlbParmSpec> {
        SelEmbeddedSflashW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&mut self) -> RegSpi0MasterModeW<'_, GlbParmSpec> {
        RegSpi0MasterModeW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&mut self) -> RegSpi0SwapW<'_, GlbParmSpec> {
        RegSpi0SwapW::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&mut self) -> RegCciUseJtagPinW<'_, GlbParmSpec> {
        RegCciUseJtagPinW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&mut self) -> RegCciUseSdioPinW<'_, GlbParmSpec> {
        RegCciUseSdioPinW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&mut self) -> P1AdcTestWithCciW<'_, GlbParmSpec> {
        P1AdcTestWithCciW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&mut self) -> P2DacTestWithCciW<'_, GlbParmSpec> {
        P2DacTestWithCciW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&mut self) -> P3CciUseIo2_5W<'_, GlbParmSpec> {
        P3CciUseIo2_5W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4AdcTestWithJtagW<'_, GlbParmSpec> {
        P4AdcTestWithJtagW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5DacTestWithJtagW<'_, GlbParmSpec> {
        P5DacTestWithJtagW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&mut self) -> P6SdioUseIo0_5W<'_, GlbParmSpec> {
        P6SdioUseIo0_5W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&mut self) -> P7JtagUseIo2_5W<'_, GlbParmSpec> {
        P7JtagUseIo2_5W::new(self, 23)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&mut self) -> UartSwapSetW<'_, GlbParmSpec> {
        UartSwapSetW::new(self, 24)
    }
}
#[doc = "glb_parm.\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_parm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_parm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbParmSpec;
impl crate::RegisterSpec for GlbParmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_parm::R`](R) reader structure"]
impl crate::Readable for GlbParmSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_parm::W`](W) writer structure"]
impl crate::Writable for GlbParmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets glb_parm to value 0x0001_8300"]
impl crate::Resettable for GlbParmSpec {
    const RESET_VALUE: u32 = 0x0001_8300;
}
