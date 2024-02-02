#[doc = "Register `pa1` reader"]
pub type R = crate::R<Pa1Spec>;
#[doc = "Register `pa1` writer"]
pub type W = crate::W<Pa1Spec>;
#[doc = "Field `pa_iaq` reader - "]
pub type PaIaqR = crate::FieldReader;
#[doc = "Field `pa_iaq` writer - "]
pub type PaIaqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pa_etb_en` reader - "]
pub type PaEtbEnR = crate::BitReader;
#[doc = "Field `pa_etb_en` writer - "]
pub type PaEtbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_iet` reader - "]
pub type PaIetR = crate::FieldReader;
#[doc = "Field `pa_iet` writer - "]
pub type PaIetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcore` reader - "]
pub type PaVbcoreR = crate::FieldReader;
#[doc = "Field `pa_vbcore` writer - "]
pub type PaVbcoreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcas` reader - "]
pub type PaVbcasR = crate::FieldReader;
#[doc = "Field `pa_vbcas` writer - "]
pub type PaVbcasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pa_half_on` reader - "]
pub type PaHalfOnR = crate::BitReader;
#[doc = "Field `pa_half_on` writer - "]
pub type PaHalfOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_ib_fix` reader - "]
pub type PaIbFixR = crate::BitReader;
#[doc = "Field `pa_ib_fix` writer - "]
pub type PaIbFixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_lz_bias_en` reader - "]
pub type PaLzBiasEnR = crate::BitReader;
#[doc = "Field `pa_lz_bias_en` writer - "]
pub type PaLzBiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_pwrmx_osdac` reader - "]
pub type PaPwrmxOsdacR = crate::FieldReader;
#[doc = "Field `pa_pwrmx_osdac` writer - "]
pub type PaPwrmxOsdacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_pwrmx_dac_pn_switch` reader - "]
pub type PaPwrmxDacPnSwitchR = crate::BitReader;
#[doc = "Field `pa_pwrmx_dac_pn_switch` writer - "]
pub type PaPwrmxDacPnSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_pwrmx_bm` reader - "]
pub type PaPwrmxBmR = crate::FieldReader;
#[doc = "Field `pa_pwrmx_bm` writer - "]
pub type PaPwrmxBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pa_att_gc` reader - "]
pub type PaAttGcR = crate::FieldReader;
#[doc = "Field `pa_att_gc` writer - "]
pub type PaAttGcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&self) -> PaIaqR {
        PaIaqR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&self) -> PaEtbEnR {
        PaEtbEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&self) -> PaIetR {
        PaIetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&self) -> PaVbcoreR {
        PaVbcoreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&self) -> PaVbcasR {
        PaVbcasR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&self) -> PaHalfOnR {
        PaHalfOnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&self) -> PaIbFixR {
        PaIbFixR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&self) -> PaLzBiasEnR {
        PaLzBiasEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&self) -> PaPwrmxOsdacR {
        PaPwrmxOsdacR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&self) -> PaPwrmxDacPnSwitchR {
        PaPwrmxDacPnSwitchR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&self) -> PaPwrmxBmR {
        PaPwrmxBmR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&self) -> PaAttGcR {
        PaAttGcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&mut self) -> PaIaqW<'_, Pa1Spec> {
        PaIaqW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&mut self) -> PaEtbEnW<'_, Pa1Spec> {
        PaEtbEnW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&mut self) -> PaIetW<'_, Pa1Spec> {
        PaIetW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&mut self) -> PaVbcoreW<'_, Pa1Spec> {
        PaVbcoreW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&mut self) -> PaVbcasW<'_, Pa1Spec> {
        PaVbcasW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&mut self) -> PaHalfOnW<'_, Pa1Spec> {
        PaHalfOnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&mut self) -> PaIbFixW<'_, Pa1Spec> {
        PaIbFixW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&mut self) -> PaLzBiasEnW<'_, Pa1Spec> {
        PaLzBiasEnW::new(self, 17)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&mut self) -> PaPwrmxOsdacW<'_, Pa1Spec> {
        PaPwrmxOsdacW::new(self, 18)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&mut self) -> PaPwrmxDacPnSwitchW<'_, Pa1Spec> {
        PaPwrmxDacPnSwitchW::new(self, 22)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&mut self) -> PaPwrmxBmW<'_, Pa1Spec> {
        PaPwrmxBmW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&mut self) -> PaAttGcW<'_, Pa1Spec> {
        PaAttGcW::new(self, 28)
    }
}
#[doc = "pa1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa1Spec;
impl crate::RegisterSpec for Pa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa1::R`](R) reader structure"]
impl crate::Readable for Pa1Spec {}
#[doc = "`write(|w| ..)` method takes [`pa1::W`](W) writer structure"]
impl crate::Writable for Pa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pa1 to value 0"]
impl crate::Resettable for Pa1Spec {}
