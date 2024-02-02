#[doc = "Register `lo_sdm_ctrl_hw5` reader"]
pub type R = crate::R<LoSdmCtrlHw5Spec>;
#[doc = "Register `lo_sdm_ctrl_hw5` writer"]
pub type W = crate::W<LoSdmCtrlHw5Spec>;
#[doc = "Field `lo_center_freq_mhz` reader - "]
pub type LoCenterFreqMhzR = crate::FieldReader<u16>;
#[doc = "Field `lo_center_freq_mhz` writer - "]
pub type LoCenterFreqMhzW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `lo_sdm_bypass_mode` reader - "]
pub type LoSdmBypassModeR = crate::FieldReader;
#[doc = "Field `lo_sdm_bypass_mode` writer - "]
pub type LoSdmBypassModeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&self) -> LoCenterFreqMhzR {
        LoCenterFreqMhzR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&self) -> LoSdmBypassModeR {
        LoSdmBypassModeR::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&mut self) -> LoCenterFreqMhzW<'_, LoSdmCtrlHw5Spec> {
        LoCenterFreqMhzW::new(self, 0)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&mut self) -> LoSdmBypassModeW<'_, LoSdmCtrlHw5Spec> {
        LoSdmBypassModeW::new(self, 12)
    }
}
#[doc = "lo_sdm_ctrl_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw5Spec;
impl crate::RegisterSpec for LoSdmCtrlHw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw5::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw5Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw5::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw5 to value 0"]
impl crate::Resettable for LoSdmCtrlHw5Spec {}
