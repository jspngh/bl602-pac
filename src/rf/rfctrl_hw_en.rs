#[doc = "Register `rfctrl_hw_en` reader"]
pub type R = crate::R<RfctrlHwEnSpec>;
#[doc = "Register `rfctrl_hw_en` writer"]
pub type W = crate::W<RfctrlHwEnSpec>;
#[doc = "Field `pu_ctrl_hw` reader - "]
pub type PuCtrlHwR = crate::BitReader;
#[doc = "Field `pu_ctrl_hw` writer - "]
pub type PuCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_gain_ctrl_hw` reader - "]
pub type RxGainCtrlHwR = crate::BitReader;
#[doc = "Field `rx_gain_ctrl_hw` writer - "]
pub type RxGainCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_gain_ctrl_hw` reader - "]
pub type TxGainCtrlHwR = crate::BitReader;
#[doc = "Field `tx_gain_ctrl_hw` writer - "]
pub type TxGainCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lna_ctrl_hw` reader - "]
pub type LnaCtrlHwR = crate::BitReader;
#[doc = "Field `lna_ctrl_hw` writer - "]
pub type LnaCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_bw_ctrl_hw` reader - "]
pub type RbbBwCtrlHwR = crate::BitReader;
#[doc = "Field `rbb_bw_ctrl_hw` writer - "]
pub type RbbBwCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trxcal_ctrl_hw` reader - "]
pub type TrxcalCtrlHwR = crate::BitReader;
#[doc = "Field `trxcal_ctrl_hw` writer - "]
pub type TrxcalCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_ctrl_hw` reader - "]
pub type LoCtrlHwR = crate::BitReader;
#[doc = "Field `lo_ctrl_hw` writer - "]
pub type LoCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inc_acal_ctrl_en_hw` reader - "]
pub type IncAcalCtrlEnHwR = crate::BitReader;
#[doc = "Field `inc_acal_ctrl_en_hw` writer - "]
pub type IncAcalCtrlEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inc_fcal_ctrl_en_hw` reader - "]
pub type IncFcalCtrlEnHwR = crate::BitReader;
#[doc = "Field `inc_fcal_ctrl_en_hw` writer - "]
pub type IncFcalCtrlEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdm_ctrl_hw` reader - "]
pub type SdmCtrlHwR = crate::BitReader;
#[doc = "Field `sdm_ctrl_hw` writer - "]
pub type SdmCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` reader - "]
pub type RbbPkdetEnCtrlHwR = crate::BitReader;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` writer - "]
pub type RbbPkdetEnCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` reader - "]
pub type RbbPkdetOutRstnCtrlHwR = crate::BitReader;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` writer - "]
pub type RbbPkdetOutRstnCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adda_ctrl_hw` reader - "]
pub type AddaCtrlHwR = crate::BitReader;
#[doc = "Field `adda_ctrl_hw` writer - "]
pub type AddaCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PuCtrlHwR {
        PuCtrlHwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&self) -> RxGainCtrlHwR {
        RxGainCtrlHwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&self) -> TxGainCtrlHwR {
        TxGainCtrlHwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&self) -> LnaCtrlHwR {
        LnaCtrlHwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RbbBwCtrlHwR {
        RbbBwCtrlHwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&self) -> TrxcalCtrlHwR {
        TrxcalCtrlHwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&self) -> LoCtrlHwR {
        LoCtrlHwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&self) -> IncAcalCtrlEnHwR {
        IncAcalCtrlEnHwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&self) -> IncFcalCtrlEnHwR {
        IncFcalCtrlEnHwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&self) -> SdmCtrlHwR {
        SdmCtrlHwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RbbPkdetEnCtrlHwR {
        RbbPkdetEnCtrlHwR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RbbPkdetOutRstnCtrlHwR {
        RbbPkdetOutRstnCtrlHwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&self) -> AddaCtrlHwR {
        AddaCtrlHwR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&mut self) -> PuCtrlHwW<'_, RfctrlHwEnSpec> {
        PuCtrlHwW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&mut self) -> RxGainCtrlHwW<'_, RfctrlHwEnSpec> {
        RxGainCtrlHwW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&mut self) -> TxGainCtrlHwW<'_, RfctrlHwEnSpec> {
        TxGainCtrlHwW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&mut self) -> LnaCtrlHwW<'_, RfctrlHwEnSpec> {
        LnaCtrlHwW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RbbBwCtrlHwW<'_, RfctrlHwEnSpec> {
        RbbBwCtrlHwW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&mut self) -> TrxcalCtrlHwW<'_, RfctrlHwEnSpec> {
        TrxcalCtrlHwW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&mut self) -> LoCtrlHwW<'_, RfctrlHwEnSpec> {
        LoCtrlHwW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&mut self) -> IncAcalCtrlEnHwW<'_, RfctrlHwEnSpec> {
        IncAcalCtrlEnHwW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&mut self) -> IncFcalCtrlEnHwW<'_, RfctrlHwEnSpec> {
        IncFcalCtrlEnHwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&mut self) -> SdmCtrlHwW<'_, RfctrlHwEnSpec> {
        SdmCtrlHwW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RbbPkdetEnCtrlHwW<'_, RfctrlHwEnSpec> {
        RbbPkdetEnCtrlHwW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RbbPkdetOutRstnCtrlHwW<'_, RfctrlHwEnSpec> {
        RbbPkdetOutRstnCtrlHwW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&mut self) -> AddaCtrlHwW<'_, RfctrlHwEnSpec> {
        AddaCtrlHwW::new(self, 12)
    }
}
#[doc = "Control logic switch\n\nYou can [`read`](crate::Reg::read) this register and get [`rfctrl_hw_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfctrl_hw_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfctrlHwEnSpec;
impl crate::RegisterSpec for RfctrlHwEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfctrl_hw_en::R`](R) reader structure"]
impl crate::Readable for RfctrlHwEnSpec {}
#[doc = "`write(|w| ..)` method takes [`rfctrl_hw_en::W`](W) writer structure"]
impl crate::Writable for RfctrlHwEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfctrl_hw_en to value 0"]
impl crate::Resettable for RfctrlHwEnSpec {}
