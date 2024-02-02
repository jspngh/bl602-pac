#[doc = "Register `MBIST_CTL` reader"]
pub type R = crate::R<MbistCtlSpec>;
#[doc = "Register `MBIST_CTL` writer"]
pub type W = crate::W<MbistCtlSpec>;
#[doc = "Field `irom_mbist_mode` reader - "]
pub type IromMbistModeR = crate::BitReader;
#[doc = "Field `irom_mbist_mode` writer - "]
pub type IromMbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsram_mbist_mode` reader - "]
pub type HsramMbistModeR = crate::BitReader;
#[doc = "Field `hsram_mbist_mode` writer - "]
pub type HsramMbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tag_mbist_mode` reader - "]
pub type TagMbistModeR = crate::BitReader;
#[doc = "Field `tag_mbist_mode` writer - "]
pub type TagMbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ocram_mbist_mode` reader - "]
pub type OcramMbistModeR = crate::BitReader;
#[doc = "Field `ocram_mbist_mode` writer - "]
pub type OcramMbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_mbist_mode` reader - "]
pub type WifiMbistModeR = crate::BitReader;
#[doc = "Field `wifi_mbist_mode` writer - "]
pub type WifiMbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mbist_rst_n` reader - "]
pub type RegMbistRstNR = crate::BitReader;
#[doc = "Field `reg_mbist_rst_n` writer - "]
pub type RegMbistRstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&self) -> IromMbistModeR {
        IromMbistModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&self) -> HsramMbistModeR {
        HsramMbistModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&self) -> TagMbistModeR {
        TagMbistModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&self) -> OcramMbistModeR {
        OcramMbistModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&self) -> WifiMbistModeR {
        WifiMbistModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&self) -> RegMbistRstNR {
        RegMbistRstNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&mut self) -> IromMbistModeW<'_, MbistCtlSpec> {
        IromMbistModeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&mut self) -> HsramMbistModeW<'_, MbistCtlSpec> {
        HsramMbistModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&mut self) -> TagMbistModeW<'_, MbistCtlSpec> {
        TagMbistModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&mut self) -> OcramMbistModeW<'_, MbistCtlSpec> {
        OcramMbistModeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&mut self) -> WifiMbistModeW<'_, MbistCtlSpec> {
        WifiMbistModeW::new(self, 4)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&mut self) -> RegMbistRstNW<'_, MbistCtlSpec> {
        RegMbistRstNW::new(self, 31)
    }
}
#[doc = "MBIST_CTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistCtlSpec;
impl crate::RegisterSpec for MbistCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_ctl::R`](R) reader structure"]
impl crate::Readable for MbistCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mbist_ctl::W`](W) writer structure"]
impl crate::Writable for MbistCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_CTL to value 0"]
impl crate::Resettable for MbistCtlSpec {}
