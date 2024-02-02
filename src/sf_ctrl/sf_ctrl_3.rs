#[doc = "Register `sf_ctrl_3` reader"]
pub type R = crate::R<SfCtrl3Spec>;
#[doc = "Register `sf_ctrl_3` writer"]
pub type W = crate::W<SfCtrl3Spec>;
#[doc = "Field `sf_cmds_wrap_len` reader - "]
pub type SfCmdsWrapLenR = crate::FieldReader;
#[doc = "Field `sf_cmds_wrap_len` writer - "]
pub type SfCmdsWrapLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sf_cmds_en` reader - "]
pub type SfCmdsEnR = crate::BitReader;
#[doc = "Field `sf_cmds_en` writer - "]
pub type SfCmdsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_bt_dly` reader - "]
pub type SfCmdsBtDlyR = crate::FieldReader;
#[doc = "Field `sf_cmds_bt_dly` writer - "]
pub type SfCmdsBtDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_cmds_bt_en` reader - "]
pub type SfCmdsBtEnR = crate::BitReader;
#[doc = "Field `sf_cmds_bt_en` writer - "]
pub type SfCmdsBtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_wrap_q_ini` reader - "]
pub type SfCmdsWrapQIniR = crate::BitReader;
#[doc = "Field `sf_cmds_wrap_q_ini` writer - "]
pub type SfCmdsWrapQIniW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_wrap_mode` reader - "]
pub type SfCmdsWrapModeR = crate::BitReader;
#[doc = "Field `sf_cmds_wrap_mode` writer - "]
pub type SfCmdsWrapModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_ack_lat` reader - "]
pub type SfIf1AckLatR = crate::FieldReader;
#[doc = "Field `sf_if_1_ack_lat` writer - "]
pub type SfIf1AckLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&self) -> SfCmdsWrapLenR {
        SfCmdsWrapLenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&self) -> SfCmdsEnR {
        SfCmdsEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&self) -> SfCmdsBtDlyR {
        SfCmdsBtDlyR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&self) -> SfCmdsBtEnR {
        SfCmdsBtEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&self) -> SfCmdsWrapQIniR {
        SfCmdsWrapQIniR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&self) -> SfCmdsWrapModeR {
        SfCmdsWrapModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&self) -> SfIf1AckLatR {
        SfIf1AckLatR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&mut self) -> SfCmdsWrapLenW<'_, SfCtrl3Spec> {
        SfCmdsWrapLenW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&mut self) -> SfCmdsEnW<'_, SfCtrl3Spec> {
        SfCmdsEnW::new(self, 4)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&mut self) -> SfCmdsBtDlyW<'_, SfCtrl3Spec> {
        SfCmdsBtDlyW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&mut self) -> SfCmdsBtEnW<'_, SfCtrl3Spec> {
        SfCmdsBtEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&mut self) -> SfCmdsWrapQIniW<'_, SfCtrl3Spec> {
        SfCmdsWrapQIniW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&mut self) -> SfCmdsWrapModeW<'_, SfCtrl3Spec> {
        SfCmdsWrapModeW::new(self, 10)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&mut self) -> SfIf1AckLatW<'_, SfCtrl3Spec> {
        SfIf1AckLatW::new(self, 29)
    }
}
#[doc = "sf_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl3Spec;
impl crate::RegisterSpec for SfCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_3::R`](R) reader structure"]
impl crate::Readable for SfCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_3::W`](W) writer structure"]
impl crate::Writable for SfCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_ctrl_3 to value 0x2000_0046"]
impl crate::Resettable for SfCtrl3Spec {
    const RESET_VALUE: u32 = 0x2000_0046;
}
