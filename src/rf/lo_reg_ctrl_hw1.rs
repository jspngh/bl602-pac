#[doc = "Register `lo_reg_ctrl_hw1` reader"]
pub type R = crate::R<LoRegCtrlHw1Spec>;
#[doc = "Register `lo_reg_ctrl_hw1` writer"]
pub type W = crate::W<LoRegCtrlHw1Spec>;
#[doc = "Field `lo_fbdv_halfstep_en_rx` reader - "]
pub type LoFbdvHalfstepEnRxR = crate::BitReader;
#[doc = "Field `lo_fbdv_halfstep_en_rx` writer - "]
pub type LoFbdvHalfstepEnRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_fbdv_halfstep_en_tx` reader - "]
pub type LoFbdvHalfstepEnTxR = crate::BitReader;
#[doc = "Field `lo_fbdv_halfstep_en_tx` writer - "]
pub type LoFbdvHalfstepEnTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_sel_rx` reader - "]
pub type LoCpSelRxR = crate::BitReader;
#[doc = "Field `lo_cp_sel_rx` writer - "]
pub type LoCpSelRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_sel_tx` reader - "]
pub type LoCpSelTxR = crate::BitReader;
#[doc = "Field `lo_cp_sel_tx` writer - "]
pub type LoCpSelTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_lf_cz_rx` reader - "]
pub type LoLfCzRxR = crate::FieldReader;
#[doc = "Field `lo_lf_cz_rx` writer - "]
pub type LoLfCzRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_cz_tx` reader - "]
pub type LoLfCzTxR = crate::FieldReader;
#[doc = "Field `lo_lf_cz_tx` writer - "]
pub type LoLfCzTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_rz_rx` reader - "]
pub type LoLfRzRxR = crate::FieldReader;
#[doc = "Field `lo_lf_rz_rx` writer - "]
pub type LoLfRzRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_rz_tx` reader - "]
pub type LoLfRzTxR = crate::FieldReader;
#[doc = "Field `lo_lf_rz_tx` writer - "]
pub type LoLfRzTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_r4_rx` reader - "]
pub type LoLfR4RxR = crate::FieldReader;
#[doc = "Field `lo_lf_r4_rx` writer - "]
pub type LoLfR4RxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_r4_tx` reader - "]
pub type LoLfR4TxR = crate::FieldReader;
#[doc = "Field `lo_lf_r4_tx` writer - "]
pub type LoLfR4TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&self) -> LoFbdvHalfstepEnRxR {
        LoFbdvHalfstepEnRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&self) -> LoFbdvHalfstepEnTxR {
        LoFbdvHalfstepEnTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&self) -> LoCpSelRxR {
        LoCpSelRxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&self) -> LoCpSelTxR {
        LoCpSelTxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&self) -> LoLfCzRxR {
        LoLfCzRxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&self) -> LoLfCzTxR {
        LoLfCzTxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&self) -> LoLfRzRxR {
        LoLfRzRxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&self) -> LoLfRzTxR {
        LoLfRzTxR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&self) -> LoLfR4RxR {
        LoLfR4RxR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&self) -> LoLfR4TxR {
        LoLfR4TxR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&mut self) -> LoFbdvHalfstepEnRxW<'_, LoRegCtrlHw1Spec> {
        LoFbdvHalfstepEnRxW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&mut self) -> LoFbdvHalfstepEnTxW<'_, LoRegCtrlHw1Spec> {
        LoFbdvHalfstepEnTxW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&mut self) -> LoCpSelRxW<'_, LoRegCtrlHw1Spec> {
        LoCpSelRxW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&mut self) -> LoCpSelTxW<'_, LoRegCtrlHw1Spec> {
        LoCpSelTxW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&mut self) -> LoLfCzRxW<'_, LoRegCtrlHw1Spec> {
        LoLfCzRxW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&mut self) -> LoLfCzTxW<'_, LoRegCtrlHw1Spec> {
        LoLfCzTxW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&mut self) -> LoLfRzRxW<'_, LoRegCtrlHw1Spec> {
        LoLfRzRxW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&mut self) -> LoLfRzTxW<'_, LoRegCtrlHw1Spec> {
        LoLfRzTxW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&mut self) -> LoLfR4RxW<'_, LoRegCtrlHw1Spec> {
        LoLfR4RxW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&mut self) -> LoLfR4TxW<'_, LoRegCtrlHw1Spec> {
        LoLfR4TxW::new(self, 24)
    }
}
#[doc = "lo_reg_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_reg_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_reg_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoRegCtrlHw1Spec;
impl crate::RegisterSpec for LoRegCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_reg_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for LoRegCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_reg_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for LoRegCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for LoRegCtrlHw1Spec {}
