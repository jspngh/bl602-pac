#[doc = "Register `dfe_ctrl_9` reader"]
pub type R = crate::R<DfeCtrl9Spec>;
#[doc = "Register `dfe_ctrl_9` writer"]
pub type W = crate::W<DfeCtrl9Spec>;
#[doc = "Field `rx_pm_iqacc_q` reader - "]
pub type RxPmIqaccQR = crate::FieldReader<u32>;
#[doc = "Field `rx_pm_iqacc_q` writer - "]
pub type RxPmIqaccQW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_q(&self) -> RxPmIqaccQR {
        RxPmIqaccQR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_q(&mut self) -> RxPmIqaccQW<'_, DfeCtrl9Spec> {
        RxPmIqaccQW::new(self, 0)
    }
}
#[doc = "dfe_ctrl_9.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl9Spec;
impl crate::RegisterSpec for DfeCtrl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_9::R`](R) reader structure"]
impl crate::Readable for DfeCtrl9Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_9::W`](W) writer structure"]
impl crate::Writable for DfeCtrl9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_9 to value 0"]
impl crate::Resettable for DfeCtrl9Spec {}
