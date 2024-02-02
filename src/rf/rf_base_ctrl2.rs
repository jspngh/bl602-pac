#[doc = "Register `rf_base_ctrl2` reader"]
pub type R = crate::R<RfBaseCtrl2Spec>;
#[doc = "Register `rf_base_ctrl2` writer"]
pub type W = crate::W<RfBaseCtrl2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ZRF Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_base_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_base_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfBaseCtrl2Spec;
impl crate::RegisterSpec for RfBaseCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_base_ctrl2::R`](R) reader structure"]
impl crate::Readable for RfBaseCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_base_ctrl2::W`](W) writer structure"]
impl crate::Writable for RfBaseCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_base_ctrl2 to value 0"]
impl crate::Resettable for RfBaseCtrl2Spec {}
