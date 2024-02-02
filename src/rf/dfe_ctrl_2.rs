#[doc = "Register `dfe_ctrl_2` reader"]
pub type R = crate::R<DfeCtrl2Spec>;
#[doc = "Register `dfe_ctrl_2` writer"]
pub type W = crate::W<DfeCtrl2Spec>;
#[doc = "Field `rx_adc_os_i` reader - "]
pub type RxAdcOsIR = crate::FieldReader<u16>;
#[doc = "Field `rx_adc_os_i` writer - "]
pub type RxAdcOsIW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_adc_os_q` reader - "]
pub type RxAdcOsQR = crate::FieldReader<u16>;
#[doc = "Field `rx_adc_os_q` writer - "]
pub type RxAdcOsQW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_adc_dce_flt_en` reader - "]
pub type RxAdcDceFltEnR = crate::BitReader;
#[doc = "Field `rx_adc_dce_flt_en` writer - "]
pub type RxAdcDceFltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_adc_low_pow_en` reader - "]
pub type RxAdcLowPowEnR = crate::BitReader;
#[doc = "Field `rx_adc_low_pow_en` writer - "]
pub type RxAdcLowPowEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_adc_dat_format` reader - "]
pub type RxAdcDatFormatR = crate::BitReader;
#[doc = "Field `rx_adc_dat_format` writer - "]
pub type RxAdcDatFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_adc_iq_swap` reader - "]
pub type RxAdcIqSwapR = crate::BitReader;
#[doc = "Field `rx_adc_iq_swap` writer - "]
pub type RxAdcIqSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_os_i(&self) -> RxAdcOsIR {
        RxAdcOsIR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_os_q(&self) -> RxAdcOsQR {
        RxAdcOsQR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_adc_dce_flt_en(&self) -> RxAdcDceFltEnR {
        RxAdcDceFltEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_adc_low_pow_en(&self) -> RxAdcLowPowEnR {
        RxAdcLowPowEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_adc_dat_format(&self) -> RxAdcDatFormatR {
        RxAdcDatFormatR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_adc_iq_swap(&self) -> RxAdcIqSwapR {
        RxAdcIqSwapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_os_i(&mut self) -> RxAdcOsIW<'_, DfeCtrl2Spec> {
        RxAdcOsIW::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_os_q(&mut self) -> RxAdcOsQW<'_, DfeCtrl2Spec> {
        RxAdcOsQW::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_adc_dce_flt_en(&mut self) -> RxAdcDceFltEnW<'_, DfeCtrl2Spec> {
        RxAdcDceFltEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_adc_low_pow_en(&mut self) -> RxAdcLowPowEnW<'_, DfeCtrl2Spec> {
        RxAdcLowPowEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_adc_dat_format(&mut self) -> RxAdcDatFormatW<'_, DfeCtrl2Spec> {
        RxAdcDatFormatW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_adc_iq_swap(&mut self) -> RxAdcIqSwapW<'_, DfeCtrl2Spec> {
        RxAdcIqSwapW::new(self, 31)
    }
}
#[doc = "dfe_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl2Spec;
impl crate::RegisterSpec for DfeCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_2::R`](R) reader structure"]
impl crate::Readable for DfeCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_2::W`](W) writer structure"]
impl crate::Writable for DfeCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_2 to value 0"]
impl crate::Resettable for DfeCtrl2Spec {}
