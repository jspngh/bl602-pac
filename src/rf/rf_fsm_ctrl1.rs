#[doc = "Register `rf_fsm_ctrl1` reader"]
pub type R = crate::R<RfFsmCtrl1Spec>;
#[doc = "Register `rf_fsm_ctrl1` writer"]
pub type W = crate::W<RfFsmCtrl1Spec>;
#[doc = "Field `rf_fsm_lo_time` reader - "]
pub type RfFsmLoTimeR = crate::FieldReader<u16>;
#[doc = "Field `rf_fsm_lo_time` writer - "]
pub type RfFsmLoTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rf_fsm_lo_rdy` reader - "]
pub type RfFsmLoRdyR = crate::BitReader;
#[doc = "Field `rf_fsm_lo_rdy` writer - "]
pub type RfFsmLoRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_lo_rdy_rst` reader - "]
pub type RfFsmLoRdyRstR = crate::BitReader;
#[doc = "Field `rf_fsm_lo_rdy_rst` writer - "]
pub type RfFsmLoRdyRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_lo_rdy_4s_1` reader - "]
pub type RfFsmLoRdy4s1R = crate::BitReader;
#[doc = "Field `rf_fsm_lo_rdy_4s_1` writer - "]
pub type RfFsmLoRdy4s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_lo_rdy_sbclr` reader - "]
pub type RfFsmLoRdySbclrR = crate::BitReader;
#[doc = "Field `rf_fsm_lo_rdy_sbclr` writer - "]
pub type RfFsmLoRdySbclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_pu_pa_dly_n` reader - "]
pub type RfFsmPuPaDlyNR = crate::FieldReader<u16>;
#[doc = "Field `rf_fsm_pu_pa_dly_n` writer - "]
pub type RfFsmPuPaDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RfFsmLoTimeR {
        RfFsmLoTimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&self) -> RfFsmLoRdyR {
        RfFsmLoRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&self) -> RfFsmLoRdyRstR {
        RfFsmLoRdyRstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&self) -> RfFsmLoRdy4s1R {
        RfFsmLoRdy4s1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&self) -> RfFsmLoRdySbclrR {
        RfFsmLoRdySbclrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&self) -> RfFsmPuPaDlyNR {
        RfFsmPuPaDlyNR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&mut self) -> RfFsmLoTimeW<'_, RfFsmCtrl1Spec> {
        RfFsmLoTimeW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&mut self) -> RfFsmLoRdyW<'_, RfFsmCtrl1Spec> {
        RfFsmLoRdyW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&mut self) -> RfFsmLoRdyRstW<'_, RfFsmCtrl1Spec> {
        RfFsmLoRdyRstW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&mut self) -> RfFsmLoRdy4s1W<'_, RfFsmCtrl1Spec> {
        RfFsmLoRdy4s1W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&mut self) -> RfFsmLoRdySbclrW<'_, RfFsmCtrl1Spec> {
        RfFsmLoRdySbclrW::new(self, 19)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&mut self) -> RfFsmPuPaDlyNW<'_, RfFsmCtrl1Spec> {
        RfFsmPuPaDlyNW::new(self, 20)
    }
}
#[doc = "rf_fsm_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsmCtrl1Spec;
impl crate::RegisterSpec for RfFsmCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm_ctrl1::R`](R) reader structure"]
impl crate::Readable for RfFsmCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm_ctrl1::W`](W) writer structure"]
impl crate::Writable for RfFsmCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_fsm_ctrl1 to value 0"]
impl crate::Resettable for RfFsmCtrl1Spec {}
