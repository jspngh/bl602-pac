#[doc = "Register `ef_sw_cfg_0` reader"]
pub type R = crate::R<EfSwCfg0Spec>;
#[doc = "Register `ef_sw_cfg_0` writer"]
pub type W = crate::W<EfSwCfg0Spec>;
#[doc = "Field `ef_sw_sf_aes_mode` reader - "]
pub type EfSwSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_sw_sf_aes_mode` writer - "]
pub type EfSwSfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_sboot_sign_mode` reader - "]
pub type EfSwSbootSignModeR = crate::FieldReader;
#[doc = "Field `ef_sw_sboot_sign_mode` writer - "]
pub type EfSwSbootSignModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_sboot_en` reader - "]
pub type EfSwSbootEnR = crate::FieldReader;
#[doc = "Field `ef_sw_sboot_en` writer - "]
pub type EfSwSbootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_cpu1_enc_en` reader - "]
pub type EfSwCpu1EncEnR = crate::BitReader;
#[doc = "Field `ef_sw_cpu1_enc_en` writer - "]
pub type EfSwCpu1EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu0_enc_en` reader - "]
pub type EfSwCpu0EncEnR = crate::BitReader;
#[doc = "Field `ef_sw_cpu0_enc_en` writer - "]
pub type EfSwCpu0EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_sw_usage_1` reader - "]
pub type EfSwSwUsage1R = crate::FieldReader;
#[doc = "Field `ef_sw_sw_usage_1` writer - "]
pub type EfSwSwUsage1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_sdu_dis` reader - "]
pub type EfSwSduDisR = crate::BitReader;
#[doc = "Field `ef_sw_sdu_dis` writer - "]
pub type EfSwSduDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_ble_dis` reader - "]
pub type EfSwBleDisR = crate::BitReader;
#[doc = "Field `ef_sw_ble_dis` writer - "]
pub type EfSwBleDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_wifi_dis` reader - "]
pub type EfSwWifiDisR = crate::BitReader;
#[doc = "Field `ef_sw_wifi_dis` writer - "]
pub type EfSwWifiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_0_key_enc_en` reader - "]
pub type EfSw0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_sw_0_key_enc_en` writer - "]
pub type EfSw0KeyEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cam_dis` reader - "]
pub type EfSwCamDisR = crate::BitReader;
#[doc = "Field `ef_sw_cam_dis` writer - "]
pub type EfSwCamDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_sf_dis` reader - "]
pub type EfSwSfDisR = crate::BitReader;
#[doc = "Field `ef_sw_sf_dis` writer - "]
pub type EfSwSfDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu1_dis` reader - "]
pub type EfSwCpu1DisR = crate::BitReader;
#[doc = "Field `ef_sw_cpu1_dis` writer - "]
pub type EfSwCpu1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu_rst_dbg_dis` reader - "]
pub type EfSwCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_cpu_rst_dbg_dis` writer - "]
pub type EfSwCpuRstDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_se_dbg_dis` reader - "]
pub type EfSwSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_se_dbg_dis` writer - "]
pub type EfSwSeDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_efuse_dbg_dis` reader - "]
pub type EfSwEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_efuse_dbg_dis` writer - "]
pub type EfSwEfuseDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_dbg_jtag_1_dis` reader - "]
pub type EfSwDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_jtag_1_dis` writer - "]
pub type EfSwDbgJtag1DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_dbg_jtag_0_dis` reader - "]
pub type EfSwDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_jtag_0_dis` writer - "]
pub type EfSwDbgJtag0DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_dbg_mode` reader - "]
pub type EfSwDbgModeR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_mode` writer - "]
pub type EfSwDbgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sw_sf_aes_mode(&self) -> EfSwSfAesModeR {
        EfSwSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sw_sboot_sign_mode(&self) -> EfSwSbootSignModeR {
        EfSwSbootSignModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sw_sboot_en(&self) -> EfSwSbootEnR {
        EfSwSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_sw_cpu1_enc_en(&self) -> EfSwCpu1EncEnR {
        EfSwCpu1EncEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_sw_cpu0_enc_en(&self) -> EfSwCpu0EncEnR {
        EfSwCpu0EncEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_sw_usage_1(&self) -> EfSwSwUsage1R {
        EfSwSwUsage1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sw_sdu_dis(&self) -> EfSwSduDisR {
        EfSwSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_sw_ble_dis(&self) -> EfSwBleDisR {
        EfSwBleDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_sw_wifi_dis(&self) -> EfSwWifiDisR {
        EfSwWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_sw_0_key_enc_en(&self) -> EfSw0KeyEncEnR {
        EfSw0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_sw_cam_dis(&self) -> EfSwCamDisR {
        EfSwCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sw_sf_dis(&self) -> EfSwSfDisR {
        EfSwSfDisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_sw_cpu1_dis(&self) -> EfSwCpu1DisR {
        EfSwCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_sw_cpu_rst_dbg_dis(&self) -> EfSwCpuRstDbgDisR {
        EfSwCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_sw_se_dbg_dis(&self) -> EfSwSeDbgDisR {
        EfSwSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_sw_efuse_dbg_dis(&self) -> EfSwEfuseDbgDisR {
        EfSwEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_1_dis(&self) -> EfSwDbgJtag1DisR {
        EfSwDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_0_dis(&self) -> EfSwDbgJtag0DisR {
        EfSwDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_sw_dbg_mode(&self) -> EfSwDbgModeR {
        EfSwDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sw_sf_aes_mode(&mut self) -> EfSwSfAesModeW<'_, EfSwCfg0Spec> {
        EfSwSfAesModeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sw_sboot_sign_mode(&mut self) -> EfSwSbootSignModeW<'_, EfSwCfg0Spec> {
        EfSwSbootSignModeW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sw_sboot_en(&mut self) -> EfSwSbootEnW<'_, EfSwCfg0Spec> {
        EfSwSbootEnW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_sw_cpu1_enc_en(&mut self) -> EfSwCpu1EncEnW<'_, EfSwCfg0Spec> {
        EfSwCpu1EncEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_sw_cpu0_enc_en(&mut self) -> EfSwCpu0EncEnW<'_, EfSwCfg0Spec> {
        EfSwCpu0EncEnW::new(self, 7)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_sw_usage_1(&mut self) -> EfSwSwUsage1W<'_, EfSwCfg0Spec> {
        EfSwSwUsage1W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sw_sdu_dis(&mut self) -> EfSwSduDisW<'_, EfSwCfg0Spec> {
        EfSwSduDisW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_sw_ble_dis(&mut self) -> EfSwBleDisW<'_, EfSwCfg0Spec> {
        EfSwBleDisW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_sw_wifi_dis(&mut self) -> EfSwWifiDisW<'_, EfSwCfg0Spec> {
        EfSwWifiDisW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_sw_0_key_enc_en(&mut self) -> EfSw0KeyEncEnW<'_, EfSwCfg0Spec> {
        EfSw0KeyEncEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_sw_cam_dis(&mut self) -> EfSwCamDisW<'_, EfSwCfg0Spec> {
        EfSwCamDisW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sw_sf_dis(&mut self) -> EfSwSfDisW<'_, EfSwCfg0Spec> {
        EfSwSfDisW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_sw_cpu1_dis(&mut self) -> EfSwCpu1DisW<'_, EfSwCfg0Spec> {
        EfSwCpu1DisW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_sw_cpu_rst_dbg_dis(&mut self) -> EfSwCpuRstDbgDisW<'_, EfSwCfg0Spec> {
        EfSwCpuRstDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_sw_se_dbg_dis(&mut self) -> EfSwSeDbgDisW<'_, EfSwCfg0Spec> {
        EfSwSeDbgDisW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_sw_efuse_dbg_dis(&mut self) -> EfSwEfuseDbgDisW<'_, EfSwCfg0Spec> {
        EfSwEfuseDbgDisW::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_1_dis(&mut self) -> EfSwDbgJtag1DisW<'_, EfSwCfg0Spec> {
        EfSwDbgJtag1DisW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_0_dis(&mut self) -> EfSwDbgJtag0DisW<'_, EfSwCfg0Spec> {
        EfSwDbgJtag0DisW::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_sw_dbg_mode(&mut self) -> EfSwDbgModeW<'_, EfSwCfg0Spec> {
        EfSwDbgModeW::new(self, 28)
    }
}
#[doc = "ef_sw_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfSwCfg0Spec;
impl crate::RegisterSpec for EfSwCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_sw_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfSwCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_sw_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfSwCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_sw_cfg_0 to value 0"]
impl crate::Resettable for EfSwCfg0Spec {}
