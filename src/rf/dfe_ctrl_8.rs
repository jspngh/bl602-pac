#[doc = "Register `dfe_ctrl_8` reader"]
pub type R = crate::R<DfeCtrl8Spec>;
#[doc = "Register `dfe_ctrl_8` writer"]
pub type W = crate::W<DfeCtrl8Spec>;
#[doc = "Field `rx_pm_iqacc_i` reader - "]
pub type RxPmIqaccIR = crate::FieldReader<u32>;
#[doc = "Field `rx_pm_iqacc_i` writer - "]
pub type RxPmIqaccIW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&self) -> RxPmIqaccIR {
        RxPmIqaccIR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&mut self) -> RxPmIqaccIW<'_, DfeCtrl8Spec> {
        RxPmIqaccIW::new(self, 0)
    }
}
#[doc = "dfe_ctrl_8.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl8Spec;
impl crate::RegisterSpec for DfeCtrl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_8::R`](R) reader structure"]
impl crate::Readable for DfeCtrl8Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_8::W`](W) writer structure"]
impl crate::Writable for DfeCtrl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_8 to value 0"]
impl crate::Resettable for DfeCtrl8Spec {}
