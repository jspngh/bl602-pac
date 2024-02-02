#[doc = "Register `rf_fsm_ctrl_hw` reader"]
pub type R = crate::R<RfFsmCtrlHwSpec>;
#[doc = "Register `rf_fsm_ctrl_hw` writer"]
pub type W = crate::W<RfFsmCtrlHwSpec>;
#[doc = "Field `rf_fsm_ctrl_en` reader - "]
pub type RfFsmCtrlEnR = crate::BitReader;
#[doc = "Field `rf_fsm_ctrl_en` writer - "]
pub type RfFsmCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_t2r_cal_mode` reader - "]
pub type RfFsmT2rCalModeR = crate::FieldReader;
#[doc = "Field `rf_fsm_t2r_cal_mode` writer - "]
pub type RfFsmT2rCalModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rf_fsm_state` reader - "]
pub type RfFsmStateR = crate::FieldReader;
#[doc = "Field `rf_fsm_state` writer - "]
pub type RfFsmStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_rc_state_dbg` reader - "]
pub type RfRcStateDbgR = crate::FieldReader;
#[doc = "Field `rf_rc_state_dbg` writer - "]
pub type RfRcStateDbgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_rc_state_dbg_en` reader - "]
pub type RfRcStateDbgEnR = crate::BitReader;
#[doc = "Field `rf_rc_state_dbg_en` writer - "]
pub type RfRcStateDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_st_int_sel` reader - "]
pub type RfFsmStIntSelR = crate::FieldReader;
#[doc = "Field `rf_fsm_st_int_sel` writer - "]
pub type RfFsmStIntSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_fsm_st_int` reader - "]
pub type RfFsmStIntR = crate::BitReader;
#[doc = "Field `rf_fsm_st_int` writer - "]
pub type RfFsmStIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_st_int_clr` reader - "]
pub type RfFsmStIntClrR = crate::BitReader;
#[doc = "Field `rf_fsm_st_int_clr` writer - "]
pub type RfFsmStIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_fsm_st_int_set` reader - "]
pub type RfFsmStIntSetR = crate::BitReader;
#[doc = "Field `rf_fsm_st_int_set` writer - "]
pub type RfFsmStIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_rc_state_value` reader - "]
pub type RfRcStateValueR = crate::FieldReader;
#[doc = "Field `rf_rc_state_value` writer - "]
pub type RfRcStateValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&self) -> RfFsmCtrlEnR {
        RfFsmCtrlEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&self) -> RfFsmT2rCalModeR {
        RfFsmT2rCalModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RfFsmStateR {
        RfFsmStateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&self) -> RfRcStateDbgR {
        RfRcStateDbgR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&self) -> RfRcStateDbgEnR {
        RfRcStateDbgEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&self) -> RfFsmStIntSelR {
        RfFsmStIntSelR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&self) -> RfFsmStIntR {
        RfFsmStIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&self) -> RfFsmStIntClrR {
        RfFsmStIntClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&self) -> RfFsmStIntSetR {
        RfFsmStIntSetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&self) -> RfRcStateValueR {
        RfRcStateValueR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&mut self) -> RfFsmCtrlEnW<'_, RfFsmCtrlHwSpec> {
        RfFsmCtrlEnW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&mut self) -> RfFsmT2rCalModeW<'_, RfFsmCtrlHwSpec> {
        RfFsmT2rCalModeW::new(self, 2)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&mut self) -> RfFsmStateW<'_, RfFsmCtrlHwSpec> {
        RfFsmStateW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&mut self) -> RfRcStateDbgW<'_, RfFsmCtrlHwSpec> {
        RfRcStateDbgW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&mut self) -> RfRcStateDbgEnW<'_, RfFsmCtrlHwSpec> {
        RfRcStateDbgEnW::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&mut self) -> RfFsmStIntSelW<'_, RfFsmCtrlHwSpec> {
        RfFsmStIntSelW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&mut self) -> RfFsmStIntW<'_, RfFsmCtrlHwSpec> {
        RfFsmStIntW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&mut self) -> RfFsmStIntClrW<'_, RfFsmCtrlHwSpec> {
        RfFsmStIntClrW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&mut self) -> RfFsmStIntSetW<'_, RfFsmCtrlHwSpec> {
        RfFsmStIntSetW::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&mut self) -> RfRcStateValueW<'_, RfFsmCtrlHwSpec> {
        RfRcStateValueW::new(self, 28)
    }
}
#[doc = "Digital Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsmCtrlHwSpec;
impl crate::RegisterSpec for RfFsmCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for RfFsmCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for RfFsmCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_hw to value 0"]
impl crate::Resettable for RfFsmCtrlHwSpec {}
