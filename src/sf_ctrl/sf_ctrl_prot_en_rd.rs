#[doc = "Register `sf_ctrl_prot_en_rd` reader"]
pub type R = crate::R<SfCtrlProtEnRdSpec>;
#[doc = "Field `sf_ctrl_prot_en_rd` reader - "]
pub type SfCtrlProtEnRdR = crate::BitReader;
#[doc = "Field `sf_ctrl_id0_en_rd` reader - "]
pub type SfCtrlId0EnRdR = crate::BitReader;
#[doc = "Field `sf_ctrl_id1_en_rd` reader - "]
pub type SfCtrlId1EnRdR = crate::BitReader;
#[doc = "Field `sf_if_0_trig_wr_lock` reader - "]
pub type SfIf0TrigWrLockR = crate::BitReader;
#[doc = "Field `sf_dbg_dis` reader - "]
pub type SfDbgDisR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SfCtrlProtEnRdR {
        SfCtrlProtEnRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SfCtrlId0EnRdR {
        SfCtrlId0EnRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SfCtrlId1EnRdR {
        SfCtrlId1EnRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SfIf0TrigWrLockR {
        SfIf0TrigWrLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&self) -> SfDbgDisR {
        SfDbgDisR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "sf_ctrl_prot_en_rd.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrlProtEnRdSpec;
impl crate::RegisterSpec for SfCtrlProtEnRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_prot_en_rd::R`](R) reader structure"]
impl crate::Readable for SfCtrlProtEnRdSpec {}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0x07"]
impl crate::Resettable for SfCtrlProtEnRdSpec {
    const RESET_VALUE: u32 = 0x07;
}
