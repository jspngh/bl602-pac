#[doc = "Register `WIFI_BT_COEX_CTRL` reader"]
pub type R = crate::R<WifiBtCoexCtrlSpec>;
#[doc = "Register `WIFI_BT_COEX_CTRL` writer"]
pub type W = crate::W<WifiBtCoexCtrlSpec>;
#[doc = "Field `coex_bt_channel` reader - "]
pub type CoexBtChannelR = crate::FieldReader;
#[doc = "Field `coex_bt_channel` writer - "]
pub type CoexBtChannelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `coex_bt_pti` reader - "]
pub type CoexBtPtiR = crate::FieldReader;
#[doc = "Field `coex_bt_pti` writer - "]
pub type CoexBtPtiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `coex_bt_bw` reader - "]
pub type CoexBtBwR = crate::BitReader;
#[doc = "Field `coex_bt_bw` writer - "]
pub type CoexBtBwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_gpio_bt_coex` reader - "]
pub type EnGpioBtCoexR = crate::BitReader;
#[doc = "Field `en_gpio_bt_coex` writer - "]
pub type EnGpioBtCoexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&self) -> CoexBtChannelR {
        CoexBtChannelR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&self) -> CoexBtPtiR {
        CoexBtPtiR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&self) -> CoexBtBwR {
        CoexBtBwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&self) -> EnGpioBtCoexR {
        EnGpioBtCoexR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&mut self) -> CoexBtChannelW<'_, WifiBtCoexCtrlSpec> {
        CoexBtChannelW::new(self, 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&mut self) -> CoexBtPtiW<'_, WifiBtCoexCtrlSpec> {
        CoexBtPtiW::new(self, 7)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&mut self) -> CoexBtBwW<'_, WifiBtCoexCtrlSpec> {
        CoexBtBwW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&mut self) -> EnGpioBtCoexW<'_, WifiBtCoexCtrlSpec> {
        EnGpioBtCoexW::new(self, 12)
    }
}
#[doc = "WIFI_BT_COEX_CTRL.\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_bt_coex_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_bt_coex_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiBtCoexCtrlSpec;
impl crate::RegisterSpec for WifiBtCoexCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_bt_coex_ctrl::R`](R) reader structure"]
impl crate::Readable for WifiBtCoexCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wifi_bt_coex_ctrl::W`](W) writer structure"]
impl crate::Writable for WifiBtCoexCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_BT_COEX_CTRL to value 0"]
impl crate::Resettable for WifiBtCoexCtrlSpec {}
