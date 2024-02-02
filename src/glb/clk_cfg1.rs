#[doc = "Register `clk_cfg1` reader"]
pub type R = crate::R<ClkCfg1Spec>;
#[doc = "Register `clk_cfg1` writer"]
pub type W = crate::W<ClkCfg1Spec>;
#[doc = "Field `wifi_mac_core_div` reader - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub type WifiMacCoreDivR = crate::FieldReader;
#[doc = "Field `wifi_mac_core_div` writer - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub type WifiMacCoreDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wifi_mac_wt_div` reader - WiFi encryption clock divider"]
pub type WifiMacWtDivR = crate::FieldReader;
#[doc = "Field `wifi_mac_wt_div` writer - WiFi encryption clock divider"]
pub type WifiMacWtDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ble_clk_sel` reader - "]
pub type BleClkSelR = crate::FieldReader;
#[doc = "Field `ble_clk_sel` writer - "]
pub type BleClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ble_en` reader - Bluetooth clock enable"]
pub type BleEnR = crate::BitReader;
#[doc = "Field `ble_en` writer - Bluetooth clock enable"]
pub type BleEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&self) -> WifiMacCoreDivR {
        WifiMacCoreDivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&self) -> WifiMacWtDivR {
        WifiMacWtDivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BleClkSelR {
        BleClkSelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    pub fn ble_en(&self) -> BleEnR {
        BleEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&mut self) -> WifiMacCoreDivW<'_, ClkCfg1Spec> {
        WifiMacCoreDivW::new(self, 0)
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&mut self) -> WifiMacWtDivW<'_, ClkCfg1Spec> {
        WifiMacWtDivW::new(self, 4)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&mut self) -> BleClkSelW<'_, ClkCfg1Spec> {
        BleClkSelW::new(self, 16)
    }
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    pub fn ble_en(&mut self) -> BleEnW<'_, ClkCfg1Spec> {
        BleEnW::new(self, 24)
    }
}
#[doc = "clk_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCfg1Spec;
impl crate::RegisterSpec for ClkCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg1::R`](R) reader structure"]
impl crate::Readable for ClkCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg1::W`](W) writer structure"]
impl crate::Writable for ClkCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clk_cfg1 to value 0x0110_0001"]
impl crate::Resettable for ClkCfg1Spec {
    const RESET_VALUE: u32 = 0x0110_0001;
}
