#[doc = "Register `rf_fsm_ctrl2` reader"]
pub type R = crate::R<RfFsmCtrl2Spec>;
#[doc = "Register `rf_fsm_ctrl2` writer"]
pub type W = crate::W<RfFsmCtrl2Spec>;
#[doc = "Field `rf_fsm_st_dbg` reader - "]
pub type RfFsmStDbgR = crate::FieldReader;
#[doc = "Field `rf_fsm_st_dbg` writer - "]
pub type RfFsmStDbgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_fsm_st_dbg_en` reader - "]
pub type RfFsmStDbgEnR = crate::BitReader;
#[doc = "Field `rf_fsm_st_dbg_en` writer - "]
pub type RfFsmStDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_trx_en_ble_4s` reader - "]
pub type RfTrxEnBle4sR = crate::BitReader;
#[doc = "Field `rf_trx_en_ble_4s` writer - "]
pub type RfTrxEnBle4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_trx_sw_ble_4s` reader - "]
pub type RfTrxSwBle4sR = crate::BitReader;
#[doc = "Field `rf_trx_sw_ble_4s` writer - "]
pub type RfTrxSwBle4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_trx_ble_4s_en` reader - "]
pub type RfTrxBle4sEnR = crate::BitReader;
#[doc = "Field `rf_trx_ble_4s_en` writer - "]
pub type RfTrxBle4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_dfe_tx_dly_n` reader - "]
pub type RfFsmDfeTxDlyNR = crate::FieldReader<u16>;
#[doc = "Field `rf_fsm_dfe_tx_dly_n` writer - "]
pub type RfFsmDfeTxDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rf_fsm_dfe_rx_dly_n` reader - "]
pub type RfFsmDfeRxDlyNR = crate::FieldReader<u16>;
#[doc = "Field `rf_fsm_dfe_rx_dly_n` writer - "]
pub type RfFsmDfeRxDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&self) -> RfFsmStDbgR {
        RfFsmStDbgR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&self) -> RfFsmStDbgEnR {
        RfFsmStDbgEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&self) -> RfTrxEnBle4sR {
        RfTrxEnBle4sR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&self) -> RfTrxSwBle4sR {
        RfTrxSwBle4sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&self) -> RfTrxBle4sEnR {
        RfTrxBle4sEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&self) -> RfFsmDfeTxDlyNR {
        RfFsmDfeTxDlyNR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&self) -> RfFsmDfeRxDlyNR {
        RfFsmDfeRxDlyNR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&mut self) -> RfFsmStDbgW<'_, RfFsmCtrl2Spec> {
        RfFsmStDbgW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&mut self) -> RfFsmStDbgEnW<'_, RfFsmCtrl2Spec> {
        RfFsmStDbgEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&mut self) -> RfTrxEnBle4sW<'_, RfFsmCtrl2Spec> {
        RfTrxEnBle4sW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&mut self) -> RfTrxSwBle4sW<'_, RfFsmCtrl2Spec> {
        RfTrxSwBle4sW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&mut self) -> RfTrxBle4sEnW<'_, RfFsmCtrl2Spec> {
        RfTrxBle4sEnW::new(self, 6)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&mut self) -> RfFsmDfeTxDlyNW<'_, RfFsmCtrl2Spec> {
        RfFsmDfeTxDlyNW::new(self, 10)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&mut self) -> RfFsmDfeRxDlyNW<'_, RfFsmCtrl2Spec> {
        RfFsmDfeRxDlyNW::new(self, 20)
    }
}
#[doc = "rf_fsm_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsmCtrl2Spec;
impl crate::RegisterSpec for RfFsmCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm_ctrl2::R`](R) reader structure"]
impl crate::Readable for RfFsmCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm_ctrl2::W`](W) writer structure"]
impl crate::Writable for RfFsmCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_fsm_ctrl2 to value 0"]
impl crate::Resettable for RfFsmCtrl2Spec {}
