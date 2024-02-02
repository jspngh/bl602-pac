#[doc = "Register `MBIST_STAT` reader"]
pub type R = crate::R<MbistStatSpec>;
#[doc = "Field `irom_mbist_done` reader - "]
pub type IromMbistDoneR = crate::BitReader;
#[doc = "Field `hsram_mbist_done` reader - "]
pub type HsramMbistDoneR = crate::BitReader;
#[doc = "Field `tag_mbist_done` reader - "]
pub type TagMbistDoneR = crate::BitReader;
#[doc = "Field `ocram_mbist_done` reader - "]
pub type OcramMbistDoneR = crate::BitReader;
#[doc = "Field `wifi_mbist_done` reader - "]
pub type WifiMbistDoneR = crate::BitReader;
#[doc = "Field `irom_mbist_fail` reader - "]
pub type IromMbistFailR = crate::BitReader;
#[doc = "Field `hsram_mbist_fail` reader - "]
pub type HsramMbistFailR = crate::BitReader;
#[doc = "Field `tag_mbist_fail` reader - "]
pub type TagMbistFailR = crate::BitReader;
#[doc = "Field `ocram_mbist_fail` reader - "]
pub type OcramMbistFailR = crate::BitReader;
#[doc = "Field `wifi_mbist_fail` reader - "]
pub type WifiMbistFailR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IromMbistDoneR {
        IromMbistDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HsramMbistDoneR {
        HsramMbistDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TagMbistDoneR {
        TagMbistDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OcramMbistDoneR {
        OcramMbistDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WifiMbistDoneR {
        WifiMbistDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IromMbistFailR {
        IromMbistFailR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HsramMbistFailR {
        HsramMbistFailR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TagMbistFailR {
        TagMbistFailR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OcramMbistFailR {
        OcramMbistFailR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WifiMbistFailR {
        WifiMbistFailR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "MBIST_STAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistStatSpec;
impl crate::RegisterSpec for MbistStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_stat::R`](R) reader structure"]
impl crate::Readable for MbistStatSpec {}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MbistStatSpec {}
