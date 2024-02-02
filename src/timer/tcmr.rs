#[doc = "Register `TCMR` reader"]
pub type R = crate::R<TcmrSpec>;
#[doc = "Register `TCMR` writer"]
pub type W = crate::W<TcmrSpec>;
#[doc = "Field `timer2_mode` reader - "]
pub type Timer2ModeR = crate::BitReader;
#[doc = "Field `timer2_mode` writer - "]
pub type Timer2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer3_mode` reader - "]
pub type Timer3ModeR = crate::BitReader;
#[doc = "Field `timer3_mode` writer - "]
pub type Timer3ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&self) -> Timer2ModeR {
        Timer2ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&self) -> Timer3ModeR {
        Timer3ModeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&mut self) -> Timer2ModeW<'_, TcmrSpec> {
        Timer2ModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&mut self) -> Timer3ModeW<'_, TcmrSpec> {
        Timer3ModeW::new(self, 2)
    }
}
#[doc = "TCMR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmrSpec;
impl crate::RegisterSpec for TcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmr::R`](R) reader structure"]
impl crate::Readable for TcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmr::W`](W) writer structure"]
impl crate::Writable for TcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TcmrSpec {}
