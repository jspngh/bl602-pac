#[doc = "Register `ten_dig` reader"]
pub type R = crate::R<TenDigSpec>;
#[doc = "Register `ten_dig` writer"]
pub type W = crate::W<TenDigSpec>;
#[doc = "Field `dten_clkpll_postdiv_clk` reader - "]
pub type DtenClkpllPostdivClkR = crate::BitReader;
#[doc = "Field `dten_clkpll_postdiv_clk` writer - "]
pub type DtenClkpllPostdivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_clkpll_clk96m` reader - "]
pub type DtenClkpllClk96mR = crate::BitReader;
#[doc = "Field `dten_clkpll_clk96m` writer - "]
pub type DtenClkpllClk96mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_clkpll_clk32m` reader - "]
pub type DtenClkpllClk32mR = crate::BitReader;
#[doc = "Field `dten_clkpll_clk32m` writer - "]
pub type DtenClkpllClk32mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_clkpll_fsdm` reader - "]
pub type DtenClkpllFsdmR = crate::BitReader;
#[doc = "Field `dten_clkpll_fsdm` writer - "]
pub type DtenClkpllFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_clkpll_fref` reader - "]
pub type DtenClkpllFrefR = crate::BitReader;
#[doc = "Field `dten_clkpll_fref` writer - "]
pub type DtenClkpllFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_clkpll_fin` reader - "]
pub type DtenClkpllFinR = crate::BitReader;
#[doc = "Field `dten_clkpll_fin` writer - "]
pub type DtenClkpllFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_lo_fsdm` reader - "]
pub type DtenLoFsdmR = crate::BitReader;
#[doc = "Field `dten_lo_fsdm` writer - "]
pub type DtenLoFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_lo_fref` reader - "]
pub type DtenLoFrefR = crate::BitReader;
#[doc = "Field `dten_lo_fref` writer - "]
pub type DtenLoFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtest_pull_down` reader - "]
pub type DtestPullDownR = crate::BitReader;
#[doc = "Field `dtest_pull_down` writer - "]
pub type DtestPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_dtest_en` reader - "]
pub type RfDtestEnR = crate::BitReader;
#[doc = "Field `rf_dtest_en` writer - "]
pub type RfDtestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DtenClkpllPostdivClkR {
        DtenClkpllPostdivClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&self) -> DtenClkpllClk96mR {
        DtenClkpllClk96mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&self) -> DtenClkpllClk32mR {
        DtenClkpllClk32mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DtenClkpllFsdmR {
        DtenClkpllFsdmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DtenClkpllFrefR {
        DtenClkpllFrefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DtenClkpllFinR {
        DtenClkpllFinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&self) -> DtenLoFsdmR {
        DtenLoFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&self) -> DtenLoFrefR {
        DtenLoFrefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&self) -> DtestPullDownR {
        DtestPullDownR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&self) -> RfDtestEnR {
        RfDtestEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DtenClkpllPostdivClkW<'_, TenDigSpec> {
        DtenClkpllPostdivClkW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&mut self) -> DtenClkpllClk96mW<'_, TenDigSpec> {
        DtenClkpllClk96mW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&mut self) -> DtenClkpllClk32mW<'_, TenDigSpec> {
        DtenClkpllClk32mW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&mut self) -> DtenClkpllFsdmW<'_, TenDigSpec> {
        DtenClkpllFsdmW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&mut self) -> DtenClkpllFrefW<'_, TenDigSpec> {
        DtenClkpllFrefW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&mut self) -> DtenClkpllFinW<'_, TenDigSpec> {
        DtenClkpllFinW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&mut self) -> DtenLoFsdmW<'_, TenDigSpec> {
        DtenLoFsdmW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&mut self) -> DtenLoFrefW<'_, TenDigSpec> {
        DtenLoFrefW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&mut self) -> DtestPullDownW<'_, TenDigSpec> {
        DtestPullDownW::new(self, 9)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&mut self) -> RfDtestEnW<'_, TenDigSpec> {
        RfDtestEnW::new(self, 23)
    }
}
#[doc = "digital test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_dig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_dig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TenDigSpec;
impl crate::RegisterSpec for TenDigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ten_dig::R`](R) reader structure"]
impl crate::Readable for TenDigSpec {}
#[doc = "`write(|w| ..)` method takes [`ten_dig::W`](W) writer structure"]
impl crate::Writable for TenDigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ten_dig to value 0"]
impl crate::Resettable for TenDigSpec {}
