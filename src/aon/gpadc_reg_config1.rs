#[doc = "Register `gpadc_reg_config1` reader"]
pub type R = crate::R<GpadcRegConfig1Spec>;
#[doc = "Register `gpadc_reg_config1` writer"]
pub type W = crate::W<GpadcRegConfig1Spec>;
#[doc = "Field `gpadc_cal_os_en` reader - "]
pub type GpadcCalOsEnR = crate::BitReader;
#[doc = "Field `gpadc_cal_os_en` writer - "]
pub type GpadcCalOsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_cont_conv_en` reader - "]
pub type GpadcContConvEnR = crate::BitReader;
#[doc = "Field `gpadc_cont_conv_en` writer - "]
pub type GpadcContConvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_res_sel` reader - "]
pub type GpadcResSelR = crate::FieldReader;
#[doc = "Field `gpadc_res_sel` writer - "]
pub type GpadcResSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_clk_ana_inv` reader - "]
pub type GpadcClkAnaInvR = crate::BitReader;
#[doc = "Field `gpadc_clk_ana_inv` writer - "]
pub type GpadcClkAnaInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_clk_div_ratio` reader - "]
pub type GpadcClkDivRatioR = crate::FieldReader;
#[doc = "Field `gpadc_clk_div_ratio` writer - "]
pub type GpadcClkDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_scan_length` reader - "]
pub type GpadcScanLengthR = crate::FieldReader;
#[doc = "Field `gpadc_scan_length` writer - "]
pub type GpadcScanLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpadc_scan_en` reader - "]
pub type GpadcScanEnR = crate::BitReader;
#[doc = "Field `gpadc_scan_en` writer - "]
pub type GpadcScanEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_dither_en` reader - "]
pub type GpadcDitherEnR = crate::BitReader;
#[doc = "Field `gpadc_dither_en` writer - "]
pub type GpadcDitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_v11_sel` reader - "]
pub type GpadcV11SelR = crate::FieldReader;
#[doc = "Field `gpadc_v11_sel` writer - "]
pub type GpadcV11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_v18_sel` reader - "]
pub type GpadcV18SelR = crate::FieldReader;
#[doc = "Field `gpadc_v18_sel` writer - "]
pub type GpadcV18SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&self) -> GpadcCalOsEnR {
        GpadcCalOsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&self) -> GpadcContConvEnR {
        GpadcContConvEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&self) -> GpadcResSelR {
        GpadcResSelR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&self) -> GpadcClkAnaInvR {
        GpadcClkAnaInvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&self) -> GpadcClkDivRatioR {
        GpadcClkDivRatioR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&self) -> GpadcScanLengthR {
        GpadcScanLengthR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&self) -> GpadcScanEnR {
        GpadcScanEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&self) -> GpadcDitherEnR {
        GpadcDitherEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&self) -> GpadcV11SelR {
        GpadcV11SelR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&self) -> GpadcV18SelR {
        GpadcV18SelR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&mut self) -> GpadcCalOsEnW<'_, GpadcRegConfig1Spec> {
        GpadcCalOsEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&mut self) -> GpadcContConvEnW<'_, GpadcRegConfig1Spec> {
        GpadcContConvEnW::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&mut self) -> GpadcResSelW<'_, GpadcRegConfig1Spec> {
        GpadcResSelW::new(self, 2)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&mut self) -> GpadcClkAnaInvW<'_, GpadcRegConfig1Spec> {
        GpadcClkAnaInvW::new(self, 17)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&mut self) -> GpadcClkDivRatioW<'_, GpadcRegConfig1Spec> {
        GpadcClkDivRatioW::new(self, 18)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&mut self) -> GpadcScanLengthW<'_, GpadcRegConfig1Spec> {
        GpadcScanLengthW::new(self, 21)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&mut self) -> GpadcScanEnW<'_, GpadcRegConfig1Spec> {
        GpadcScanEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&mut self) -> GpadcDitherEnW<'_, GpadcRegConfig1Spec> {
        GpadcDitherEnW::new(self, 26)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&mut self) -> GpadcV11SelW<'_, GpadcRegConfig1Spec> {
        GpadcV11SelW::new(self, 27)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&mut self) -> GpadcV18SelW<'_, GpadcRegConfig1Spec> {
        GpadcV18SelW::new(self, 29)
    }
}
#[doc = "gpadc_reg_config1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegConfig1Spec;
impl crate::RegisterSpec for GpadcRegConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_config1::R`](R) reader structure"]
impl crate::Readable for GpadcRegConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_config1::W`](W) writer structure"]
impl crate::Writable for GpadcRegConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpadc_reg_config1 to value 0x000c_0002"]
impl crate::Resettable for GpadcRegConfig1Spec {
    const RESET_VALUE: u32 = 0x000c_0002;
}
