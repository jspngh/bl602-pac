#[doc = "Register `clkpll_sdm` reader"]
pub type R = crate::R<ClkpllSdmSpec>;
#[doc = "Register `clkpll_sdm` writer"]
pub type W = crate::W<ClkpllSdmSpec>;
#[doc = "Field `clkpll_sdmin` reader - "]
pub type ClkpllSdminR = crate::FieldReader<u32>;
#[doc = "Field `clkpll_sdmin` writer - "]
pub type ClkpllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `clkpll_dither_sel` reader - "]
pub type ClkpllDitherSelR = crate::FieldReader;
#[doc = "Field `clkpll_dither_sel` writer - "]
pub type ClkpllDitherSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_sdm_flag` reader - "]
pub type ClkpllSdmFlagR = crate::BitReader;
#[doc = "Field `clkpll_sdm_flag` writer - "]
pub type ClkpllSdmFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_sdm_bypass` reader - "]
pub type ClkpllSdmBypassR = crate::BitReader;
#[doc = "Field `clkpll_sdm_bypass` writer - "]
pub type ClkpllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&self) -> ClkpllSdminR {
        ClkpllSdminR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&self) -> ClkpllDitherSelR {
        ClkpllDitherSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&self) -> ClkpllSdmFlagR {
        ClkpllSdmFlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&self) -> ClkpllSdmBypassR {
        ClkpllSdmBypassR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&mut self) -> ClkpllSdminW<'_, ClkpllSdmSpec> {
        ClkpllSdminW::new(self, 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&mut self) -> ClkpllDitherSelW<'_, ClkpllSdmSpec> {
        ClkpllDitherSelW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&mut self) -> ClkpllSdmFlagW<'_, ClkpllSdmSpec> {
        ClkpllSdmFlagW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&mut self) -> ClkpllSdmBypassW<'_, ClkpllSdmSpec> {
        ClkpllSdmBypassW::new(self, 29)
    }
}
#[doc = "clkpll_sdm.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_sdm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_sdm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllSdmSpec;
impl crate::RegisterSpec for ClkpllSdmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_sdm::R`](R) reader structure"]
impl crate::Readable for ClkpllSdmSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_sdm::W`](W) writer structure"]
impl crate::Writable for ClkpllSdmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_sdm to value 0x1060_0000"]
impl crate::Resettable for ClkpllSdmSpec {
    const RESET_VALUE: u32 = 0x1060_0000;
}
