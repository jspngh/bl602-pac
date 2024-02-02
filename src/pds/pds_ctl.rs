#[doc = "Register `PDS_CTL` reader"]
pub type R = crate::R<PdsCtlSpec>;
#[doc = "Register `PDS_CTL` writer"]
pub type W = crate::W<PdsCtlSpec>;
#[doc = "Field `pds_start_ps` reader - "]
pub type PdsStartPsR = crate::BitReader;
#[doc = "Field `pds_start_ps` writer - "]
pub type PdsStartPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_sleep_forever` reader - "]
pub type CrSleepForeverR = crate::BitReader;
#[doc = "Field `cr_sleep_forever` writer - "]
pub type CrSleepForeverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_xtal_force_off` reader - "]
pub type CrXtalForceOffR = crate::BitReader;
#[doc = "Field `cr_xtal_force_off` writer - "]
pub type CrXtalForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wifi_pds_save_state` reader - "]
pub type CrWifiPdsSaveStateR = crate::BitReader;
#[doc = "Field `cr_wifi_pds_save_state` writer - "]
pub type CrWifiPdsSaveStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_dcdc18` reader - "]
pub type CrPdsPdDcdc18R = crate::BitReader;
#[doc = "Field `cr_pds_pd_dcdc18` writer - "]
pub type CrPdsPdDcdc18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub type CrPdsPdBgSysR = crate::BitReader;
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub type CrPdsPdBgSysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gate_clk` reader - "]
pub type CrPdsGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_gate_clk` writer - "]
pub type CrPdsGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mem_stby` reader - "]
pub type CrPdsMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_mem_stby` writer - "]
pub type CrPdsMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_iso_en` reader - "]
pub type CrPdsIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_iso_en` writer - "]
pub type CrPdsIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wait_xtal_rdy` reader - "]
pub type CrPdsWaitXtalRdyR = crate::BitReader;
#[doc = "Field `cr_pds_wait_xtal_rdy` writer - "]
pub type CrPdsWaitXtalRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pwr_off` reader - "]
pub type CrPdsPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_pwr_off` writer - "]
pub type CrPdsPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_xtal` reader - "]
pub type CrPdsPdXtalR = crate::BitReader;
#[doc = "Field `cr_pds_pd_xtal` writer - "]
pub type CrPdsPdXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_soc_enb_force_on` reader - "]
pub type CrPdsSocEnbForceOnR = crate::BitReader;
#[doc = "Field `cr_pds_soc_enb_force_on` writer - "]
pub type CrPdsSocEnbForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rst_soc_en` reader - "]
pub type CrPdsRstSocEnR = crate::BitReader;
#[doc = "Field `cr_pds_rst_soc_en` writer - "]
pub type CrPdsRstSocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rc32m_off_dis` reader - "]
pub type CrPdsRc32mOffDisR = crate::BitReader;
#[doc = "Field `cr_pds_rc32m_off_dis` writer - "]
pub type CrPdsRc32mOffDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ldo_vsel_en` reader - "]
pub type CrPdsLdoVselEnR = crate::BitReader;
#[doc = "Field `cr_pds_ldo_vsel_en` writer - "]
pub type CrPdsLdoVselEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub type CrNpWfiMaskR = crate::BitReader;
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub type CrNpWfiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_ldo11` reader - "]
pub type CrPdsPdLdo11R = crate::BitReader;
#[doc = "Field `cr_pds_pd_ldo11` writer - "]
pub type CrPdsPdLdo11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ldo_vol` reader - "]
pub type CrPdsLdoVolR = crate::FieldReader;
#[doc = "Field `cr_pds_ldo_vol` writer - "]
pub type CrPdsLdoVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_pds_ctrl_rf` reader - "]
pub type CrPdsCtrlRfR = crate::FieldReader;
#[doc = "Field `cr_pds_ctrl_rf` writer - "]
pub type CrPdsCtrlRfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_pds_ctrl_pll` reader - "]
pub type CrPdsCtrlPllR = crate::FieldReader;
#[doc = "Field `cr_pds_ctrl_pll` writer - "]
pub type CrPdsCtrlPllW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&self) -> PdsStartPsR {
        PdsStartPsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CrSleepForeverR {
        CrSleepForeverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CrXtalForceOffR {
        CrXtalForceOffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wifi_pds_save_state(&self) -> CrWifiPdsSaveStateR {
        CrWifiPdsSaveStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&self) -> CrPdsPdDcdc18R {
        CrPdsPdDcdc18R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CrPdsPdBgSysR {
        CrPdsPdBgSysR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&self) -> CrPdsGateClkR {
        CrPdsGateClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&self) -> CrPdsMemStbyR {
        CrPdsMemStbyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&self) -> CrPdsIsoEnR {
        CrPdsIsoEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&self) -> CrPdsWaitXtalRdyR {
        CrPdsWaitXtalRdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&self) -> CrPdsPwrOffR {
        CrPdsPwrOffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&self) -> CrPdsPdXtalR {
        CrPdsPdXtalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_soc_enb_force_on(&self) -> CrPdsSocEnbForceOnR {
        CrPdsSocEnbForceOnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc_en(&self) -> CrPdsRstSocEnR {
        CrPdsRstSocEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&self) -> CrPdsRc32mOffDisR {
        CrPdsRc32mOffDisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo_vsel_en(&self) -> CrPdsLdoVselEnR {
        CrPdsLdoVselEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CrNpWfiMaskR {
        CrNpWfiMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&self) -> CrPdsPdLdo11R {
        CrPdsPdLdo11R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_pds_ldo_vol(&self) -> CrPdsLdoVolR {
        CrPdsLdoVolR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&self) -> CrPdsCtrlRfR {
        CrPdsCtrlRfR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_pll(&self) -> CrPdsCtrlPllR {
        CrPdsCtrlPllR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&mut self) -> PdsStartPsW<'_, PdsCtlSpec> {
        PdsStartPsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&mut self) -> CrSleepForeverW<'_, PdsCtlSpec> {
        CrSleepForeverW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&mut self) -> CrXtalForceOffW<'_, PdsCtlSpec> {
        CrXtalForceOffW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wifi_pds_save_state(&mut self) -> CrWifiPdsSaveStateW<'_, PdsCtlSpec> {
        CrWifiPdsSaveStateW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&mut self) -> CrPdsPdDcdc18W<'_, PdsCtlSpec> {
        CrPdsPdDcdc18W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CrPdsPdBgSysW<'_, PdsCtlSpec> {
        CrPdsPdBgSysW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&mut self) -> CrPdsGateClkW<'_, PdsCtlSpec> {
        CrPdsGateClkW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&mut self) -> CrPdsMemStbyW<'_, PdsCtlSpec> {
        CrPdsMemStbyW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&mut self) -> CrPdsIsoEnW<'_, PdsCtlSpec> {
        CrPdsIsoEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&mut self) -> CrPdsWaitXtalRdyW<'_, PdsCtlSpec> {
        CrPdsWaitXtalRdyW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&mut self) -> CrPdsPwrOffW<'_, PdsCtlSpec> {
        CrPdsPwrOffW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&mut self) -> CrPdsPdXtalW<'_, PdsCtlSpec> {
        CrPdsPdXtalW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_soc_enb_force_on(&mut self) -> CrPdsSocEnbForceOnW<'_, PdsCtlSpec> {
        CrPdsSocEnbForceOnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc_en(&mut self) -> CrPdsRstSocEnW<'_, PdsCtlSpec> {
        CrPdsRstSocEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&mut self) -> CrPdsRc32mOffDisW<'_, PdsCtlSpec> {
        CrPdsRc32mOffDisW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo_vsel_en(&mut self) -> CrPdsLdoVselEnW<'_, PdsCtlSpec> {
        CrPdsLdoVselEnW::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&mut self) -> CrNpWfiMaskW<'_, PdsCtlSpec> {
        CrNpWfiMaskW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&mut self) -> CrPdsPdLdo11W<'_, PdsCtlSpec> {
        CrPdsPdLdo11W::new(self, 22)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_pds_ldo_vol(&mut self) -> CrPdsLdoVolW<'_, PdsCtlSpec> {
        CrPdsLdoVolW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&mut self) -> CrPdsCtrlRfW<'_, PdsCtlSpec> {
        CrPdsCtrlRfW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_pll(&mut self) -> CrPdsCtrlPllW<'_, PdsCtlSpec> {
        CrPdsCtrlPllW::new(self, 30)
    }
}
#[doc = "PDS_CTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtlSpec;
impl crate::RegisterSpec for PdsCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl::R`](R) reader structure"]
impl crate::Readable for PdsCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl::W`](W) writer structure"]
impl crate::Writable for PdsCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL to value 0x1a00_6b00"]
impl crate::Resettable for PdsCtlSpec {
    const RESET_VALUE: u32 = 0x1a00_6b00;
}
