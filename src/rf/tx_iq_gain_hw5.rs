#[doc = "Register `tx_iq_gain_hw5` reader"]
pub type R = crate::R<TxIqGainHw5Spec>;
#[doc = "Register `tx_iq_gain_hw5` writer"]
pub type W = crate::W<TxIqGainHw5Spec>;
#[doc = "Field `tx_iq_phase_comp_gc5` reader - "]
pub type TxIqPhaseCompGc5R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc5` writer - "]
pub type TxIqPhaseCompGc5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc5` reader - "]
pub type TxIqGainCompGc5R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc5` writer - "]
pub type TxIqGainCompGc5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc5(&self) -> TxIqPhaseCompGc5R {
        TxIqPhaseCompGc5R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc5(&self) -> TxIqGainCompGc5R {
        TxIqGainCompGc5R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc5(&mut self) -> TxIqPhaseCompGc5W<'_, TxIqGainHw5Spec> {
        TxIqPhaseCompGc5W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc5(&mut self) -> TxIqGainCompGc5W<'_, TxIqGainHw5Spec> {
        TxIqGainCompGc5W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw5Spec;
impl crate::RegisterSpec for TxIqGainHw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw5::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw5Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw5::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw5 to value 0"]
impl crate::Resettable for TxIqGainHw5Spec {}
