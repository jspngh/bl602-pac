#[doc = "Register `rf_fsm_ctrl_sw` reader"]
pub type R = crate::R<RfFsmCtrlSwSpec>;
#[doc = "Register `rf_fsm_ctrl_sw` writer"]
pub type W = crate::W<RfFsmCtrlSwSpec>;
#[doc = "Field `rf_fsm_sw_st` reader - "]
pub type RfFsmSwStR = crate::FieldReader;
#[doc = "Field `rf_fsm_sw_st` writer - "]
pub type RfFsmSwStW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rf_fsm_sw_st_vld` reader - "]
pub type RfFsmSwStVldR = crate::BitReader;
#[doc = "Field `rf_fsm_sw_st_vld` writer - "]
pub type RfFsmSwStVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `full_cal_en` reader - "]
pub type FullCalEnR = crate::BitReader;
#[doc = "Field `full_cal_en` writer - "]
pub type FullCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inc_cal_timeout` reader - "]
pub type IncCalTimeoutR = crate::BitReader;
#[doc = "Field `inc_cal_timeout` writer - "]
pub type IncCalTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_unlocked` reader - "]
pub type LoUnlockedR = crate::BitReader;
#[doc = "Field `lo_unlocked` writer - "]
pub type LoUnlockedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&self) -> RfFsmSwStR {
        RfFsmSwStR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&self) -> RfFsmSwStVldR {
        RfFsmSwStVldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&self) -> FullCalEnR {
        FullCalEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&self) -> IncCalTimeoutR {
        IncCalTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&self) -> LoUnlockedR {
        LoUnlockedR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&mut self) -> RfFsmSwStW<'_, RfFsmCtrlSwSpec> {
        RfFsmSwStW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&mut self) -> RfFsmSwStVldW<'_, RfFsmCtrlSwSpec> {
        RfFsmSwStVldW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&mut self) -> FullCalEnW<'_, RfFsmCtrlSwSpec> {
        FullCalEnW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&mut self) -> IncCalTimeoutW<'_, RfFsmCtrlSwSpec> {
        IncCalTimeoutW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&mut self) -> LoUnlockedW<'_, RfFsmCtrlSwSpec> {
        LoUnlockedW::new(self, 20)
    }
}
#[doc = "rfsm status reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsmCtrlSwSpec;
impl crate::RegisterSpec for RfFsmCtrlSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm_ctrl_sw::R`](R) reader structure"]
impl crate::Readable for RfFsmCtrlSwSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm_ctrl_sw::W`](W) writer structure"]
impl crate::Writable for RfFsmCtrlSwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_sw to value 0"]
impl crate::Resettable for RfFsmCtrlSwSpec {}
