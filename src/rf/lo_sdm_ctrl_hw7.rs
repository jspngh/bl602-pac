#[doc = "Register `lo_sdm_ctrl_hw7` reader"]
pub type R = crate::R<LoSdmCtrlHw7Spec>;
#[doc = "Register `lo_sdm_ctrl_hw7` writer"]
pub type W = crate::W<LoSdmCtrlHw7Spec>;
#[doc = "Field `lo_sdmin_1m` reader - "]
pub type LoSdmin1mR = crate::FieldReader<u32>;
#[doc = "Field `lo_sdmin_1m` writer - "]
pub type LoSdmin1mW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_1m(&self) -> LoSdmin1mR {
        LoSdmin1mR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_1m(&mut self) -> LoSdmin1mW<'_, LoSdmCtrlHw7Spec> {
        LoSdmin1mW::new(self, 0)
    }
}
#[doc = "lo_sdm_ctrl_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw7Spec;
impl crate::RegisterSpec for LoSdmCtrlHw7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw7::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw7Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw7::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw7 to value 0"]
impl crate::Resettable for LoSdmCtrlHw7Spec {}
