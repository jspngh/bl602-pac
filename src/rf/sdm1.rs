#[doc = "Register `sdm1` reader"]
pub type R = crate::R<Sdm1Spec>;
#[doc = "Register `sdm1` writer"]
pub type W = crate::W<Sdm1Spec>;
#[doc = "Field `lo_sdm_dither_sel_hw` reader - "]
pub type LoSdmDitherSelHwR = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_hw` writer - "]
pub type LoSdmDitherSelHwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_bypass_hw` reader - "]
pub type LoSdmBypassHwR = crate::BitReader;
#[doc = "Field `lo_sdm_bypass_hw` writer - "]
pub type LoSdmBypassHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_sdm_dither_sel` reader - "]
pub type LoSdmDitherSelR = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel` writer - "]
pub type LoSdmDitherSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_bypass` reader - "]
pub type LoSdmBypassR = crate::BitReader;
#[doc = "Field `lo_sdm_bypass` writer - "]
pub type LoSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_sdm_rstb` reader - "]
pub type LoSdmRstbR = crate::BitReader;
#[doc = "Field `lo_sdm_rstb` writer - "]
pub type LoSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_sdm_rstb_hw` reader - "]
pub type LoSdmRstbHwR = crate::BitReader;
#[doc = "Field `lo_sdm_rstb_hw` writer - "]
pub type LoSdmRstbHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_sdm_flag` reader - "]
pub type LoSdmFlagR = crate::BitReader;
#[doc = "Field `lo_sdm_flag` writer - "]
pub type LoSdmFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&self) -> LoSdmDitherSelHwR {
        LoSdmDitherSelHwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&self) -> LoSdmBypassHwR {
        LoSdmBypassHwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&self) -> LoSdmDitherSelR {
        LoSdmDitherSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&self) -> LoSdmBypassR {
        LoSdmBypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&self) -> LoSdmRstbR {
        LoSdmRstbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&self) -> LoSdmRstbHwR {
        LoSdmRstbHwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&self) -> LoSdmFlagR {
        LoSdmFlagR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&mut self) -> LoSdmDitherSelHwW<'_, Sdm1Spec> {
        LoSdmDitherSelHwW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&mut self) -> LoSdmBypassHwW<'_, Sdm1Spec> {
        LoSdmBypassHwW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&mut self) -> LoSdmDitherSelW<'_, Sdm1Spec> {
        LoSdmDitherSelW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&mut self) -> LoSdmBypassW<'_, Sdm1Spec> {
        LoSdmBypassW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&mut self) -> LoSdmRstbW<'_, Sdm1Spec> {
        LoSdmRstbW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&mut self) -> LoSdmRstbHwW<'_, Sdm1Spec> {
        LoSdmRstbHwW::new(self, 17)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&mut self) -> LoSdmFlagW<'_, Sdm1Spec> {
        LoSdmFlagW::new(self, 20)
    }
}
#[doc = "sdm1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdm1Spec;
impl crate::RegisterSpec for Sdm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm1::R`](R) reader structure"]
impl crate::Readable for Sdm1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm1::W`](W) writer structure"]
impl crate::Writable for Sdm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sdm1 to value 0"]
impl crate::Resettable for Sdm1Spec {}
