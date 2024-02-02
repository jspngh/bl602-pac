#[doc = "Register `tbb` reader"]
pub type R = crate::R<TbbSpec>;
#[doc = "Register `tbb` writer"]
pub type W = crate::W<TbbSpec>;
#[doc = "Field `tbb_bm_sf` reader - "]
pub type TbbBmSfR = crate::FieldReader;
#[doc = "Field `tbb_bm_sf` writer - "]
pub type TbbBmSfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tbb_bm_cg` reader - "]
pub type TbbBmCgR = crate::FieldReader;
#[doc = "Field `tbb_bm_cg` writer - "]
pub type TbbBmCgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tbb_vcm` reader - "]
pub type TbbVcmR = crate::FieldReader;
#[doc = "Field `tbb_vcm` writer - "]
pub type TbbVcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tbb_cflt` reader - "]
pub type TbbCfltR = crate::FieldReader;
#[doc = "Field `tbb_cflt` writer - "]
pub type TbbCfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tbb_iq_bias_short` reader - "]
pub type TbbIqBiasShortR = crate::BitReader;
#[doc = "Field `tbb_iq_bias_short` writer - "]
pub type TbbIqBiasShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tbb_atest_out_en` reader - "]
pub type TbbAtestOutEnR = crate::BitReader;
#[doc = "Field `tbb_atest_out_en` writer - "]
pub type TbbAtestOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tbb_tosdac_q` reader - "]
pub type TbbTosdacQR = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q` writer - "]
pub type TbbTosdacQW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_i` reader - "]
pub type TbbTosdacIR = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i` writer - "]
pub type TbbTosdacIW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&self) -> TbbBmSfR {
        TbbBmSfR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&self) -> TbbBmCgR {
        TbbBmCgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&self) -> TbbVcmR {
        TbbVcmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&self) -> TbbCfltR {
        TbbCfltR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&self) -> TbbIqBiasShortR {
        TbbIqBiasShortR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&self) -> TbbAtestOutEnR {
        TbbAtestOutEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&self) -> TbbTosdacQR {
        TbbTosdacQR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&self) -> TbbTosdacIR {
        TbbTosdacIR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&mut self) -> TbbBmSfW<'_, TbbSpec> {
        TbbBmSfW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&mut self) -> TbbBmCgW<'_, TbbSpec> {
        TbbBmCgW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&mut self) -> TbbVcmW<'_, TbbSpec> {
        TbbVcmW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&mut self) -> TbbCfltW<'_, TbbSpec> {
        TbbCfltW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&mut self) -> TbbIqBiasShortW<'_, TbbSpec> {
        TbbIqBiasShortW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&mut self) -> TbbAtestOutEnW<'_, TbbSpec> {
        TbbAtestOutEnW::new(self, 15)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&mut self) -> TbbTosdacQW<'_, TbbSpec> {
        TbbTosdacQW::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&mut self) -> TbbTosdacIW<'_, TbbSpec> {
        TbbTosdacIW::new(self, 24)
    }
}
#[doc = "tbb.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbSpec;
impl crate::RegisterSpec for TbbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbb::R`](R) reader structure"]
impl crate::Readable for TbbSpec {}
#[doc = "`write(|w| ..)` method takes [`tbb::W`](W) writer structure"]
impl crate::Writable for TbbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tbb to value 0"]
impl crate::Resettable for TbbSpec {}
