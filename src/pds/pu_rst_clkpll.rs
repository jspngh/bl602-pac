#[doc = "Register `pu_rst_clkpll` reader"]
pub type R = crate::R<PuRstClkpllSpec>;
#[doc = "Register `pu_rst_clkpll` writer"]
pub type W = crate::W<PuRstClkpllSpec>;
#[doc = "Field `clkpll_sdm_reset` reader - "]
pub type ClkpllSdmResetR = crate::BitReader;
#[doc = "Field `clkpll_sdm_reset` writer - "]
pub type ClkpllSdmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_reset_postdiv` reader - "]
pub type ClkpllResetPostdivR = crate::BitReader;
#[doc = "Field `clkpll_reset_postdiv` writer - "]
pub type ClkpllResetPostdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_reset_fbdv` reader - "]
pub type ClkpllResetFbdvR = crate::BitReader;
#[doc = "Field `clkpll_reset_fbdv` writer - "]
pub type ClkpllResetFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_reset_refdiv` reader - "]
pub type ClkpllResetRefdivR = crate::BitReader;
#[doc = "Field `clkpll_reset_refdiv` writer - "]
pub type ClkpllResetRefdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_pu_postdiv` reader - "]
pub type ClkpllPuPostdivR = crate::BitReader;
#[doc = "Field `clkpll_pu_postdiv` writer - "]
pub type ClkpllPuPostdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_pu_fbdv` reader - "]
pub type ClkpllPuFbdvR = crate::BitReader;
#[doc = "Field `clkpll_pu_fbdv` writer - "]
pub type ClkpllPuFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_pu_clamp_op` reader - "]
pub type ClkpllPuClampOpR = crate::BitReader;
#[doc = "Field `clkpll_pu_clamp_op` writer - "]
pub type ClkpllPuClampOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_pu_pfd` reader - "]
pub type ClkpllPuPfdR = crate::BitReader;
#[doc = "Field `clkpll_pu_pfd` writer - "]
pub type ClkpllPuPfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_pu_cp` reader - "]
pub type ClkpllPuCpR = crate::BitReader;
#[doc = "Field `clkpll_pu_cp` writer - "]
pub type ClkpllPuCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_clkpll_sfreg` reader - "]
pub type PuClkpllSfregR = crate::BitReader;
#[doc = "Field `pu_clkpll_sfreg` writer - "]
pub type PuClkpllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_clkpll` reader - "]
pub type PuClkpllR = crate::BitReader;
#[doc = "Field `pu_clkpll` writer - "]
pub type PuClkpllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&self) -> ClkpllSdmResetR {
        ClkpllSdmResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&self) -> ClkpllResetPostdivR {
        ClkpllResetPostdivR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&self) -> ClkpllResetFbdvR {
        ClkpllResetFbdvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&self) -> ClkpllResetRefdivR {
        ClkpllResetRefdivR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&self) -> ClkpllPuPostdivR {
        ClkpllPuPostdivR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&self) -> ClkpllPuFbdvR {
        ClkpllPuFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&self) -> ClkpllPuClampOpR {
        ClkpllPuClampOpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&self) -> ClkpllPuPfdR {
        ClkpllPuPfdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&self) -> ClkpllPuCpR {
        ClkpllPuCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&self) -> PuClkpllSfregR {
        PuClkpllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&self) -> PuClkpllR {
        PuClkpllR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&mut self) -> ClkpllSdmResetW<'_, PuRstClkpllSpec> {
        ClkpllSdmResetW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&mut self) -> ClkpllResetPostdivW<'_, PuRstClkpllSpec> {
        ClkpllResetPostdivW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&mut self) -> ClkpllResetFbdvW<'_, PuRstClkpllSpec> {
        ClkpllResetFbdvW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&mut self) -> ClkpllResetRefdivW<'_, PuRstClkpllSpec> {
        ClkpllResetRefdivW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&mut self) -> ClkpllPuPostdivW<'_, PuRstClkpllSpec> {
        ClkpllPuPostdivW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&mut self) -> ClkpllPuFbdvW<'_, PuRstClkpllSpec> {
        ClkpllPuFbdvW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&mut self) -> ClkpllPuClampOpW<'_, PuRstClkpllSpec> {
        ClkpllPuClampOpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&mut self) -> ClkpllPuPfdW<'_, PuRstClkpllSpec> {
        ClkpllPuPfdW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&mut self) -> ClkpllPuCpW<'_, PuRstClkpllSpec> {
        ClkpllPuCpW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&mut self) -> PuClkpllSfregW<'_, PuRstClkpllSpec> {
        PuClkpllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&mut self) -> PuClkpllW<'_, PuRstClkpllSpec> {
        PuClkpllW::new(self, 10)
    }
}
#[doc = "pu_rst_clkpll.\n\nYou can [`read`](crate::Reg::read) this register and get [`pu_rst_clkpll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PuRstClkpllSpec;
impl crate::RegisterSpec for PuRstClkpllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pu_rst_clkpll::R`](R) reader structure"]
impl crate::Readable for PuRstClkpllSpec {}
#[doc = "`write(|w| ..)` method takes [`pu_rst_clkpll::W`](W) writer structure"]
impl crate::Writable for PuRstClkpllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pu_rst_clkpll to value 0x01f0"]
impl crate::Resettable for PuRstClkpllSpec {
    const RESET_VALUE: u32 = 0x01f0;
}
