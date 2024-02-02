#[doc = "Register `lo_sdm_ctrl_hw6` reader"]
pub type R = crate::R<LoSdmCtrlHw6Spec>;
#[doc = "Register `lo_sdm_ctrl_hw6` writer"]
pub type W = crate::W<LoSdmCtrlHw6Spec>;
#[doc = "Field `lo_sdmin_center` reader - "]
pub type LoSdminCenterR = crate::FieldReader<u32>;
#[doc = "Field `lo_sdmin_center` writer - "]
pub type LoSdminCenterW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn lo_sdmin_center(&self) -> LoSdminCenterR {
        LoSdminCenterR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn lo_sdmin_center(&mut self) -> LoSdminCenterW<'_, LoSdmCtrlHw6Spec> {
        LoSdminCenterW::new(self, 0)
    }
}
#[doc = "lo_sdm_ctrl_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw6Spec;
impl crate::RegisterSpec for LoSdmCtrlHw6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw6::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw6Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw6::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw6 to value 0"]
impl crate::Resettable for LoSdmCtrlHw6Spec {}
