#[doc = "Register `pa2` reader"]
pub type R = crate::R<Pa2Spec>;
#[doc = "Register `pa2` writer"]
pub type W = crate::W<Pa2Spec>;
#[doc = "Field `pa_etb_en_hw` reader - "]
pub type PaEtbEnHwR = crate::BitReader;
#[doc = "Field `pa_etb_en_hw` writer - "]
pub type PaEtbEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_iet_hw` reader - "]
pub type PaIetHwR = crate::FieldReader;
#[doc = "Field `pa_iet_hw` writer - "]
pub type PaIetHwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcore_hw` reader - "]
pub type PaVbcoreHwR = crate::FieldReader;
#[doc = "Field `pa_vbcore_hw` writer - "]
pub type PaVbcoreHwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcas_hw` reader - "]
pub type PaVbcasHwR = crate::FieldReader;
#[doc = "Field `pa_vbcas_hw` writer - "]
pub type PaVbcasHwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pa_half_on_hw` reader - "]
pub type PaHalfOnHwR = crate::BitReader;
#[doc = "Field `pa_half_on_hw` writer - "]
pub type PaHalfOnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_ib_fix_hw` reader - "]
pub type PaIbFixHwR = crate::BitReader;
#[doc = "Field `pa_ib_fix_hw` writer - "]
pub type PaIbFixHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&self) -> PaEtbEnHwR {
        PaEtbEnHwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&self) -> PaIetHwR {
        PaIetHwR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&self) -> PaVbcoreHwR {
        PaVbcoreHwR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&self) -> PaVbcasHwR {
        PaVbcasHwR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&self) -> PaHalfOnHwR {
        PaHalfOnHwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&self) -> PaIbFixHwR {
        PaIbFixHwR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&mut self) -> PaEtbEnHwW<'_, Pa2Spec> {
        PaEtbEnHwW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&mut self) -> PaIetHwW<'_, Pa2Spec> {
        PaIetHwW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&mut self) -> PaVbcoreHwW<'_, Pa2Spec> {
        PaVbcoreHwW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&mut self) -> PaVbcasHwW<'_, Pa2Spec> {
        PaVbcasHwW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&mut self) -> PaHalfOnHwW<'_, Pa2Spec> {
        PaHalfOnHwW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&mut self) -> PaIbFixHwW<'_, Pa2Spec> {
        PaIbFixHwW::new(self, 17)
    }
}
#[doc = "RX normal bias mode registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pa2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa2Spec;
impl crate::RegisterSpec for Pa2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa2::R`](R) reader structure"]
impl crate::Readable for Pa2Spec {}
#[doc = "`write(|w| ..)` method takes [`pa2::W`](W) writer structure"]
impl crate::Writable for Pa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pa2 to value 0"]
impl crate::Resettable for Pa2Spec {}
