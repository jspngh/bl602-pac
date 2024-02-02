#[doc = "Register `rfif_dfe_ctrl0` reader"]
pub type R = crate::R<RfifDfeCtrl0Spec>;
#[doc = "Register `rfif_dfe_ctrl0` writer"]
pub type W = crate::W<RfifDfeCtrl0Spec>;
#[doc = "Field `rfckg_rxclk_4s_on` reader - "]
pub type RfckgRxclk4sOnR = crate::BitReader;
#[doc = "Field `rfckg_rxclk_4s_on` writer - "]
pub type RfckgRxclk4sOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfckg_txclk_4s_on` reader - "]
pub type RfckgTxclk4sOnR = crate::BitReader;
#[doc = "Field `rfckg_txclk_4s_on` writer - "]
pub type RfckgTxclk4sOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfckg_adc_afifo_inv` reader - "]
pub type RfckgAdcAfifoInvR = crate::BitReader;
#[doc = "Field `rfckg_adc_afifo_inv` writer - "]
pub type RfckgAdcAfifoInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfckg_adc_clkout_sel` reader - "]
pub type RfckgAdcClkoutSelR = crate::BitReader;
#[doc = "Field `rfckg_adc_clkout_sel` writer - "]
pub type RfckgAdcClkoutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfckg_dac_afifo_inv` reader - "]
pub type RfckgDacAfifoInvR = crate::BitReader;
#[doc = "Field `rfckg_dac_afifo_inv` writer - "]
pub type RfckgDacAfifoInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_dfe_en_4s` reader - "]
pub type RxDfeEn4sR = crate::BitReader;
#[doc = "Field `rx_dfe_en_4s` writer - "]
pub type RxDfeEn4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_dfe_en_4s_en` reader - "]
pub type RxDfeEn4sEnR = crate::BitReader;
#[doc = "Field `rx_dfe_en_4s_en` writer - "]
pub type RxDfeEn4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_dfe_en_4s` reader - "]
pub type TxDfeEn4sR = crate::BitReader;
#[doc = "Field `tx_dfe_en_4s` writer - "]
pub type TxDfeEn4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_dfe_en_4s_en` reader - "]
pub type TxDfeEn4sEnR = crate::BitReader;
#[doc = "Field `tx_dfe_en_4s_en` writer - "]
pub type TxDfeEn4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_test_sel` reader - "]
pub type RxTestSelR = crate::FieldReader;
#[doc = "Field `rx_test_sel` writer - "]
pub type RxTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tx_test_sel` reader - "]
pub type TxTestSelR = crate::FieldReader;
#[doc = "Field `tx_test_sel` writer - "]
pub type TxTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pad_adc_clkout_inv_en` reader - "]
pub type PadAdcClkoutInvEnR = crate::BitReader;
#[doc = "Field `pad_adc_clkout_inv_en` writer - "]
pub type PadAdcClkoutInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pad_dac_clkout_inv_en` reader - "]
pub type PadDacClkoutInvEnR = crate::BitReader;
#[doc = "Field `pad_dac_clkout_inv_en` writer - "]
pub type PadDacClkoutInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_ch_ind_ble_4s` reader - "]
pub type RfChIndBle4sR = crate::FieldReader;
#[doc = "Field `rf_ch_ind_ble_4s` writer - "]
pub type RfChIndBle4sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `rf_ch_ind_ble_4s_en` reader - "]
pub type RfChIndBle4sEnR = crate::BitReader;
#[doc = "Field `rf_ch_ind_ble_4s_en` writer - "]
pub type RfChIndBle4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifimode_4s` reader - "]
pub type Wifimode4sR = crate::FieldReader;
#[doc = "Field `wifimode_4s` writer - "]
pub type Wifimode4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifimode_4s_en` reader - "]
pub type Wifimode4sEnR = crate::BitReader;
#[doc = "Field `wifimode_4s_en` writer - "]
pub type Wifimode4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bbmode_4s` reader - "]
pub type Bbmode4sR = crate::BitReader;
#[doc = "Field `bbmode_4s` writer - "]
pub type Bbmode4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bbmode_4s_en` reader - "]
pub type Bbmode4sEnR = crate::BitReader;
#[doc = "Field `bbmode_4s_en` writer - "]
pub type Bbmode4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_sel` reader - "]
pub type TestSelR = crate::FieldReader;
#[doc = "Field `test_sel` writer - "]
pub type TestSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&self) -> RfckgRxclk4sOnR {
        RfckgRxclk4sOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&self) -> RfckgTxclk4sOnR {
        RfckgTxclk4sOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&self) -> RfckgAdcAfifoInvR {
        RfckgAdcAfifoInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&self) -> RfckgAdcClkoutSelR {
        RfckgAdcClkoutSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&self) -> RfckgDacAfifoInvR {
        RfckgDacAfifoInvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&self) -> RxDfeEn4sR {
        RxDfeEn4sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&self) -> RxDfeEn4sEnR {
        RxDfeEn4sEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&self) -> TxDfeEn4sR {
        TxDfeEn4sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&self) -> TxDfeEn4sEnR {
        TxDfeEn4sEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&self) -> RxTestSelR {
        RxTestSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&self) -> TxTestSelR {
        TxTestSelR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&self) -> PadAdcClkoutInvEnR {
        PadAdcClkoutInvEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&self) -> PadDacClkoutInvEnR {
        PadDacClkoutInvEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&self) -> RfChIndBle4sR {
        RfChIndBle4sR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&self) -> RfChIndBle4sEnR {
        RfChIndBle4sEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&self) -> Wifimode4sR {
        Wifimode4sR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&self) -> Wifimode4sEnR {
        Wifimode4sEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&self) -> Bbmode4sR {
        Bbmode4sR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&self) -> Bbmode4sEnR {
        Bbmode4sEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&self) -> TestSelR {
        TestSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&mut self) -> RfckgRxclk4sOnW<'_, RfifDfeCtrl0Spec> {
        RfckgRxclk4sOnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&mut self) -> RfckgTxclk4sOnW<'_, RfifDfeCtrl0Spec> {
        RfckgTxclk4sOnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&mut self) -> RfckgAdcAfifoInvW<'_, RfifDfeCtrl0Spec> {
        RfckgAdcAfifoInvW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&mut self) -> RfckgAdcClkoutSelW<'_, RfifDfeCtrl0Spec> {
        RfckgAdcClkoutSelW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&mut self) -> RfckgDacAfifoInvW<'_, RfifDfeCtrl0Spec> {
        RfckgDacAfifoInvW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&mut self) -> RxDfeEn4sW<'_, RfifDfeCtrl0Spec> {
        RxDfeEn4sW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&mut self) -> RxDfeEn4sEnW<'_, RfifDfeCtrl0Spec> {
        RxDfeEn4sEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&mut self) -> TxDfeEn4sW<'_, RfifDfeCtrl0Spec> {
        TxDfeEn4sW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&mut self) -> TxDfeEn4sEnW<'_, RfifDfeCtrl0Spec> {
        TxDfeEn4sEnW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&mut self) -> RxTestSelW<'_, RfifDfeCtrl0Spec> {
        RxTestSelW::new(self, 9)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&mut self) -> TxTestSelW<'_, RfifDfeCtrl0Spec> {
        TxTestSelW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&mut self) -> PadAdcClkoutInvEnW<'_, RfifDfeCtrl0Spec> {
        PadAdcClkoutInvEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&mut self) -> PadDacClkoutInvEnW<'_, RfifDfeCtrl0Spec> {
        PadDacClkoutInvEnW::new(self, 14)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&mut self) -> RfChIndBle4sW<'_, RfifDfeCtrl0Spec> {
        RfChIndBle4sW::new(self, 15)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&mut self) -> RfChIndBle4sEnW<'_, RfifDfeCtrl0Spec> {
        RfChIndBle4sEnW::new(self, 22)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&mut self) -> Wifimode4sW<'_, RfifDfeCtrl0Spec> {
        Wifimode4sW::new(self, 23)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&mut self) -> Wifimode4sEnW<'_, RfifDfeCtrl0Spec> {
        Wifimode4sEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&mut self) -> Bbmode4sW<'_, RfifDfeCtrl0Spec> {
        Bbmode4sW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&mut self) -> Bbmode4sEnW<'_, RfifDfeCtrl0Spec> {
        Bbmode4sEnW::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&mut self) -> TestSelW<'_, RfifDfeCtrl0Spec> {
        TestSelW::new(self, 28)
    }
}
#[doc = "rfif_dfe_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_dfe_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_dfe_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifDfeCtrl0Spec;
impl crate::RegisterSpec for RfifDfeCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfif_dfe_ctrl0::R`](R) reader structure"]
impl crate::Readable for RfifDfeCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfif_dfe_ctrl0::W`](W) writer structure"]
impl crate::Writable for RfifDfeCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfif_dfe_ctrl0 to value 0"]
impl crate::Resettable for RfifDfeCtrl0Spec {}
