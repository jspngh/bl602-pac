#[doc = "Register `tx_iq_gain_hw0` reader"]
pub type R = crate::R<TxIqGainHw0Spec>;
#[doc = "Register `tx_iq_gain_hw0` writer"]
pub type W = crate::W<TxIqGainHw0Spec>;
#[doc = "Field `tx_iq_phase_comp_gc0` reader - "]
pub type TxIqPhaseCompGc0R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc0` writer - "]
pub type TxIqPhaseCompGc0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc0` reader - "]
pub type TxIqGainCompGc0R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc0` writer - "]
pub type TxIqGainCompGc0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc0(&self) -> TxIqPhaseCompGc0R {
        TxIqPhaseCompGc0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc0(&self) -> TxIqGainCompGc0R {
        TxIqGainCompGc0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc0(&mut self) -> TxIqPhaseCompGc0W<'_, TxIqGainHw0Spec> {
        TxIqPhaseCompGc0W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc0(&mut self) -> TxIqGainCompGc0W<'_, TxIqGainHw0Spec> {
        TxIqGainCompGc0W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw0Spec;
impl crate::RegisterSpec for TxIqGainHw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw0::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw0::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw0 to value 0"]
impl crate::Resettable for TxIqGainHw0Spec {}
