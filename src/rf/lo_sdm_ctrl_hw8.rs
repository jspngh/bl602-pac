#[doc = "Register `lo_sdm_ctrl_hw8` reader"]
pub type R = crate::R<LoSdmCtrlHw8Spec>;
#[doc = "Register `lo_sdm_ctrl_hw8` writer"]
pub type W = crate::W<LoSdmCtrlHw8Spec>;
#[doc = "Field `lo_sdmin_if` reader - "]
pub type LoSdminIfR = crate::FieldReader<u32>;
#[doc = "Field `lo_sdmin_if` writer - "]
pub type LoSdminIfW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_if(&self) -> LoSdminIfR {
        LoSdminIfR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_if(&mut self) -> LoSdminIfW<'_, LoSdmCtrlHw8Spec> {
        LoSdminIfW::new(self, 0)
    }
}
#[doc = "lo_sdm_ctrl_hw8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw8Spec;
impl crate::RegisterSpec for LoSdmCtrlHw8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw8::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw8Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw8::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw8 to value 0"]
impl crate::Resettable for LoSdmCtrlHw8Spec {}
