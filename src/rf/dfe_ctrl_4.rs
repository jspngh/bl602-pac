#[doc = "Register `dfe_ctrl_4` reader"]
pub type R = crate::R<DfeCtrl4Spec>;
#[doc = "Register `dfe_ctrl_4` writer"]
pub type W = crate::W<DfeCtrl4Spec>;
#[doc = "Field `rx_pf_th2` reader - "]
pub type RxPfTh2R = crate::FieldReader<u16>;
#[doc = "Field `rx_pf_th2` writer - "]
pub type RxPfTh2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_pf_th1` reader - "]
pub type RxPfTh1R = crate::FieldReader<u16>;
#[doc = "Field `rx_pf_th1` writer - "]
pub type RxPfTh1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_pf_q_en` reader - "]
pub type RxPfQEnR = crate::BitReader;
#[doc = "Field `rx_pf_q_en` writer - "]
pub type RxPfQEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_pf_i_en` reader - "]
pub type RxPfIEnR = crate::BitReader;
#[doc = "Field `rx_pf_i_en` writer - "]
pub type RxPfIEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_pf_th2(&self) -> RxPfTh2R {
        RxPfTh2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_pf_th1(&self) -> RxPfTh1R {
        RxPfTh1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_pf_q_en(&self) -> RxPfQEnR {
        RxPfQEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_pf_i_en(&self) -> RxPfIEnR {
        RxPfIEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_pf_th2(&mut self) -> RxPfTh2W<'_, DfeCtrl4Spec> {
        RxPfTh2W::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_pf_th1(&mut self) -> RxPfTh1W<'_, DfeCtrl4Spec> {
        RxPfTh1W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_pf_q_en(&mut self) -> RxPfQEnW<'_, DfeCtrl4Spec> {
        RxPfQEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_pf_i_en(&mut self) -> RxPfIEnW<'_, DfeCtrl4Spec> {
        RxPfIEnW::new(self, 31)
    }
}
#[doc = "dfe_ctrl_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl4Spec;
impl crate::RegisterSpec for DfeCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_4::R`](R) reader structure"]
impl crate::Readable for DfeCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_4::W`](W) writer structure"]
impl crate::Writable for DfeCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_4 to value 0"]
impl crate::Resettable for DfeCtrl4Spec {}
