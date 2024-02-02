#[doc = "Register `tzc_glb_ctrl_3` reader"]
pub type R = crate::R<TzcGlbCtrl3Spec>;
#[doc = "Register `tzc_glb_ctrl_3` writer"]
pub type W = crate::W<TzcGlbCtrl3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl3Spec;
impl crate::RegisterSpec for TzcGlbCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_3::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_3::W`](W) writer structure"]
impl crate::Writable for TzcGlbCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_3 to value 0"]
impl crate::Resettable for TzcGlbCtrl3Spec {}
