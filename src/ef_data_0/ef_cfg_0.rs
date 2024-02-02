#[doc = "Register `ef_cfg_0` reader"]
pub type R = crate::R<EfCfg0Spec>;
#[doc = "Register `ef_cfg_0` writer"]
pub type W = crate::W<EfCfg0Spec>;
#[doc = "Field `ef_sf_aes_mode` reader - "]
pub type EfSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_sf_aes_mode` writer - "]
pub type EfSfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sboot_sign_mode` reader - "]
pub type EfSbootSignModeR = crate::FieldReader;
#[doc = "Field `ef_sboot_sign_mode` writer - "]
pub type EfSbootSignModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sboot_en` reader - "]
pub type EfSbootEnR = crate::FieldReader;
#[doc = "Field `ef_sboot_en` writer - "]
pub type EfSbootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_cpu1_enc_en` reader - "]
pub type EfCpu1EncEnR = crate::BitReader;
#[doc = "Field `ef_cpu1_enc_en` writer - "]
pub type EfCpu1EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu0_enc_en` reader - "]
pub type EfCpu0EncEnR = crate::BitReader;
#[doc = "Field `ef_cpu0_enc_en` writer - "]
pub type EfCpu0EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_boot_sel` reader - "]
pub type EfBootSelR = crate::FieldReader;
#[doc = "Field `ef_boot_sel` writer - "]
pub type EfBootSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ef_sw_usage_1` reader - "]
pub type EfSwUsage1R = crate::FieldReader;
#[doc = "Field `ef_sw_usage_1` writer - "]
pub type EfSwUsage1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sdu_dis` reader - "]
pub type EfSduDisR = crate::BitReader;
#[doc = "Field `ef_sdu_dis` writer - "]
pub type EfSduDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_ble_dis` reader - "]
pub type EfBleDisR = crate::BitReader;
#[doc = "Field `ef_ble_dis` writer - "]
pub type EfBleDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_wifi_dis` reader - "]
pub type EfWifiDisR = crate::BitReader;
#[doc = "Field `ef_wifi_dis` writer - "]
pub type EfWifiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_0_key_enc_en` reader - "]
pub type Ef0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_0_key_enc_en` writer - "]
pub type Ef0KeyEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cam_dis` reader - "]
pub type EfCamDisR = crate::BitReader;
#[doc = "Field `ef_cam_dis` writer - "]
pub type EfCamDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sf_dis` reader - "]
pub type EfSfDisR = crate::BitReader;
#[doc = "Field `ef_sf_dis` writer - "]
pub type EfSfDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu1_dis` reader - "]
pub type EfCpu1DisR = crate::BitReader;
#[doc = "Field `ef_cpu1_dis` writer - "]
pub type EfCpu1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu_rst_dbg_dis` reader - "]
pub type EfCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_cpu_rst_dbg_dis` writer - "]
pub type EfCpuRstDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_se_dbg_dis` reader - "]
pub type EfSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_se_dbg_dis` writer - "]
pub type EfSeDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_efuse_dbg_dis` reader - "]
pub type EfEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_efuse_dbg_dis` writer - "]
pub type EfEfuseDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_dbg_jtag_1_dis` reader - "]
pub type EfDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_dbg_jtag_1_dis` writer - "]
pub type EfDbgJtag1DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_dbg_jtag_0_dis` reader - "]
pub type EfDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_dbg_jtag_0_dis` writer - "]
pub type EfDbgJtag0DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_dbg_mode` reader - "]
pub type EfDbgModeR = crate::FieldReader;
#[doc = "Field `ef_dbg_mode` writer - "]
pub type EfDbgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&self) -> EfSfAesModeR {
        EfSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sboot_sign_mode(&self) -> EfSbootSignModeR {
        EfSbootSignModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sboot_en(&self) -> EfSbootEnR {
        EfSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_cpu1_enc_en(&self) -> EfCpu1EncEnR {
        EfCpu1EncEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_cpu0_enc_en(&self) -> EfCpu0EncEnR {
        EfCpu0EncEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_boot_sel(&self) -> EfBootSelR {
        EfBootSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&self) -> EfSwUsage1R {
        EfSwUsage1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sdu_dis(&self) -> EfSduDisR {
        EfSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_ble_dis(&self) -> EfBleDisR {
        EfBleDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_wifi_dis(&self) -> EfWifiDisR {
        EfWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&self) -> Ef0KeyEncEnR {
        Ef0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_cam_dis(&self) -> EfCamDisR {
        EfCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sf_dis(&self) -> EfSfDisR {
        EfSfDisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&self) -> EfCpu1DisR {
        EfCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&self) -> EfCpuRstDbgDisR {
        EfCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&self) -> EfSeDbgDisR {
        EfSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&self) -> EfEfuseDbgDisR {
        EfEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&self) -> EfDbgJtag1DisR {
        EfDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&self) -> EfDbgJtag0DisR {
        EfDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_dbg_mode(&self) -> EfDbgModeR {
        EfDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&mut self) -> EfSfAesModeW<'_, EfCfg0Spec> {
        EfSfAesModeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sboot_sign_mode(&mut self) -> EfSbootSignModeW<'_, EfCfg0Spec> {
        EfSbootSignModeW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sboot_en(&mut self) -> EfSbootEnW<'_, EfCfg0Spec> {
        EfSbootEnW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_cpu1_enc_en(&mut self) -> EfCpu1EncEnW<'_, EfCfg0Spec> {
        EfCpu1EncEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_cpu0_enc_en(&mut self) -> EfCpu0EncEnW<'_, EfCfg0Spec> {
        EfCpu0EncEnW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_boot_sel(&mut self) -> EfBootSelW<'_, EfCfg0Spec> {
        EfBootSelW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&mut self) -> EfSwUsage1W<'_, EfCfg0Spec> {
        EfSwUsage1W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sdu_dis(&mut self) -> EfSduDisW<'_, EfCfg0Spec> {
        EfSduDisW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_ble_dis(&mut self) -> EfBleDisW<'_, EfCfg0Spec> {
        EfBleDisW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_wifi_dis(&mut self) -> EfWifiDisW<'_, EfCfg0Spec> {
        EfWifiDisW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&mut self) -> Ef0KeyEncEnW<'_, EfCfg0Spec> {
        Ef0KeyEncEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_cam_dis(&mut self) -> EfCamDisW<'_, EfCfg0Spec> {
        EfCamDisW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sf_dis(&mut self) -> EfSfDisW<'_, EfCfg0Spec> {
        EfSfDisW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&mut self) -> EfCpu1DisW<'_, EfCfg0Spec> {
        EfCpu1DisW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&mut self) -> EfCpuRstDbgDisW<'_, EfCfg0Spec> {
        EfCpuRstDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&mut self) -> EfSeDbgDisW<'_, EfCfg0Spec> {
        EfSeDbgDisW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&mut self) -> EfEfuseDbgDisW<'_, EfCfg0Spec> {
        EfEfuseDbgDisW::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&mut self) -> EfDbgJtag1DisW<'_, EfCfg0Spec> {
        EfDbgJtag1DisW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&mut self) -> EfDbgJtag0DisW<'_, EfCfg0Spec> {
        EfDbgJtag0DisW::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_dbg_mode(&mut self) -> EfDbgModeW<'_, EfCfg0Spec> {
        EfDbgModeW::new(self, 28)
    }
}
#[doc = "ef_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCfg0Spec;
impl crate::RegisterSpec for EfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_cfg_0 to value 0"]
impl crate::Resettable for EfCfg0Spec {}
