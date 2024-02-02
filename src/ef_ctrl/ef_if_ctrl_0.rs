#[doc = "Register `ef_if_ctrl_0` reader"]
pub type R = crate::R<EfIfCtrl0Spec>;
#[doc = "Register `ef_if_ctrl_0` writer"]
pub type W = crate::W<EfIfCtrl0Spec>;
#[doc = "Field `ef_if_0_autoload_p1_done` reader - "]
pub type EfIf0AutoloadP1DoneR = crate::BitReader;
#[doc = "Field `ef_if_0_autoload_done` reader - "]
pub type EfIf0AutoloadDoneR = crate::BitReader;
#[doc = "Field `ef_if_0_busy` reader - "]
pub type EfIf0BusyR = crate::BitReader;
#[doc = "Field `ef_if_0_rw` reader - "]
pub type EfIf0RwR = crate::BitReader;
#[doc = "Field `ef_if_0_rw` writer - "]
pub type EfIf0RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_trig` reader - "]
pub type EfIf0TrigR = crate::BitReader;
#[doc = "Field `ef_if_0_trig` writer - "]
pub type EfIf0TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_manual_en` reader - "]
pub type EfIf0ManualEnR = crate::BitReader;
#[doc = "Field `ef_if_0_manual_en` writer - "]
pub type EfIf0ManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_cyc_modify` reader - "]
pub type EfIf0CycModifyR = crate::BitReader;
#[doc = "Field `ef_if_0_cyc_modify` writer - "]
pub type EfIf0CycModifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_clk_sahb_data_sel` reader - "]
pub type EfClkSahbDataSelR = crate::BitReader;
#[doc = "Field `ef_clk_sahb_data_sel` writer - "]
pub type EfClkSahbDataSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_prot_code_ctrl` reader - "]
pub type EfIfProtCodeCtrlR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_ctrl` writer - "]
pub type EfIfProtCodeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ef_if_por_dig` reader - "]
pub type EfIfPorDigR = crate::BitReader;
#[doc = "Field `ef_if_por_dig` writer - "]
pub type EfIfPorDigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_clk_sahb_data_gate` reader - "]
pub type EfClkSahbDataGateR = crate::BitReader;
#[doc = "Field `ef_clk_sahb_data_gate` writer - "]
pub type EfClkSahbDataGateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_auto_rd_en` reader - "]
pub type EfIfAutoRdEnR = crate::BitReader;
#[doc = "Field `ef_if_auto_rd_en` writer - "]
pub type EfIfAutoRdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cyc_modify_lock` reader - "]
pub type EfIfCycModifyLockR = crate::BitReader;
#[doc = "Field `ef_if_cyc_modify_lock` writer - "]
pub type EfIfCycModifyLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_int` reader - "]
pub type EfIf0IntR = crate::BitReader;
#[doc = "Field `ef_if_0_int_clr` reader - "]
pub type EfIf0IntClrR = crate::BitReader;
#[doc = "Field `ef_if_0_int_clr` writer - "]
pub type EfIf0IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_int_set` reader - "]
pub type EfIf0IntSetR = crate::BitReader;
#[doc = "Field `ef_if_0_int_set` writer - "]
pub type EfIf0IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_prot_code_cyc` reader - "]
pub type EfIfProtCodeCycR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_cyc` writer - "]
pub type EfIfProtCodeCycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&self) -> EfIf0AutoloadP1DoneR {
        EfIf0AutoloadP1DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&self) -> EfIf0AutoloadDoneR {
        EfIf0AutoloadDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_0_busy(&self) -> EfIf0BusyR {
        EfIf0BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&self) -> EfIf0RwR {
        EfIf0RwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&self) -> EfIf0TrigR {
        EfIf0TrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&self) -> EfIf0ManualEnR {
        EfIf0ManualEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&self) -> EfIf0CycModifyR {
        EfIf0CycModifyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&self) -> EfClkSahbDataSelR {
        EfClkSahbDataSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&self) -> EfIfProtCodeCtrlR {
        EfIfProtCodeCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&self) -> EfIfPorDigR {
        EfIfPorDigR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&self) -> EfClkSahbDataGateR {
        EfClkSahbDataGateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&self) -> EfIfAutoRdEnR {
        EfIfAutoRdEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&self) -> EfIfCycModifyLockR {
        EfIfCycModifyLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_0_int(&self) -> EfIf0IntR {
        EfIf0IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&self) -> EfIf0IntClrR {
        EfIf0IntClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&self) -> EfIf0IntSetR {
        EfIf0IntSetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&self) -> EfIfProtCodeCycR {
        EfIfProtCodeCycR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&mut self) -> EfIf0RwW<'_, EfIfCtrl0Spec> {
        EfIf0RwW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&mut self) -> EfIf0TrigW<'_, EfIfCtrl0Spec> {
        EfIf0TrigW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&mut self) -> EfIf0ManualEnW<'_, EfIfCtrl0Spec> {
        EfIf0ManualEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&mut self) -> EfIf0CycModifyW<'_, EfIfCtrl0Spec> {
        EfIf0CycModifyW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&mut self) -> EfClkSahbDataSelW<'_, EfIfCtrl0Spec> {
        EfClkSahbDataSelW::new(self, 7)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&mut self) -> EfIfProtCodeCtrlW<'_, EfIfCtrl0Spec> {
        EfIfProtCodeCtrlW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&mut self) -> EfIfPorDigW<'_, EfIfCtrl0Spec> {
        EfIfPorDigW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&mut self) -> EfClkSahbDataGateW<'_, EfIfCtrl0Spec> {
        EfClkSahbDataGateW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&mut self) -> EfIfAutoRdEnW<'_, EfIfCtrl0Spec> {
        EfIfAutoRdEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&mut self) -> EfIfCycModifyLockW<'_, EfIfCtrl0Spec> {
        EfIfCycModifyLockW::new(self, 19)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&mut self) -> EfIf0IntClrW<'_, EfIfCtrl0Spec> {
        EfIf0IntClrW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&mut self) -> EfIf0IntSetW<'_, EfIfCtrl0Spec> {
        EfIf0IntSetW::new(self, 22)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&mut self) -> EfIfProtCodeCycW<'_, EfIfCtrl0Spec> {
        EfIfProtCodeCycW::new(self, 24)
    }
}
#[doc = "ef_if_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfCtrl0Spec;
impl crate::RegisterSpec for EfIfCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_ctrl_0::R`](R) reader structure"]
impl crate::Readable for EfIfCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_ctrl_0::W`](W) writer structure"]
impl crate::Writable for EfIfCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_ctrl_0 to value 0x0024_0003"]
impl crate::Resettable for EfIfCtrl0Spec {
    const RESET_VALUE: u32 = 0x0024_0003;
}
