#[doc = "Register `dfe_ctrl_7` reader"]
pub type R = crate::R<DfeCtrl7Spec>;
#[doc = "Register `dfe_ctrl_7` writer"]
pub type W = crate::W<DfeCtrl7Spec>;
#[doc = "Field `rx_pm_start_ofs` reader - "]
pub type RxPmStartOfsR = crate::FieldReader<u16>;
#[doc = "Field `rx_pm_start_ofs` writer - "]
pub type RxPmStartOfsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rx_pm_acc_len` reader - "]
pub type RxPmAccLenR = crate::FieldReader<u16>;
#[doc = "Field `rx_pm_acc_len` writer - "]
pub type RxPmAccLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&self) -> RxPmStartOfsR {
        RxPmStartOfsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&self) -> RxPmAccLenR {
        RxPmAccLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&mut self) -> RxPmStartOfsW<'_, DfeCtrl7Spec> {
        RxPmStartOfsW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&mut self) -> RxPmAccLenW<'_, DfeCtrl7Spec> {
        RxPmAccLenW::new(self, 16)
    }
}
#[doc = "dfe_ctrl_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl7Spec;
impl crate::RegisterSpec for DfeCtrl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_7::R`](R) reader structure"]
impl crate::Readable for DfeCtrl7Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_7::W`](W) writer structure"]
impl crate::Writable for DfeCtrl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_7 to value 0"]
impl crate::Resettable for DfeCtrl7Spec {}
