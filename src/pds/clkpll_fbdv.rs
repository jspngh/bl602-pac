#[doc = "Register `clkpll_fbdv` reader"]
pub type R = crate::R<ClkpllFbdvSpec>;
#[doc = "Register `clkpll_fbdv` writer"]
pub type W = crate::W<ClkpllFbdvSpec>;
#[doc = "Field `clkpll_sel_sample_clk` reader - "]
pub type ClkpllSelSampleClkR = crate::FieldReader;
#[doc = "Field `clkpll_sel_sample_clk` writer - "]
pub type ClkpllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_sel_fb_clk` reader - "]
pub type ClkpllSelFbClkR = crate::FieldReader;
#[doc = "Field `clkpll_sel_fb_clk` writer - "]
pub type ClkpllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clkpll_sel_sample_clk(&self) -> ClkpllSelSampleClkR {
        ClkpllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn clkpll_sel_fb_clk(&self) -> ClkpllSelFbClkR {
        ClkpllSelFbClkR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clkpll_sel_sample_clk(&mut self) -> ClkpllSelSampleClkW<'_, ClkpllFbdvSpec> {
        ClkpllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn clkpll_sel_fb_clk(&mut self) -> ClkpllSelFbClkW<'_, ClkpllFbdvSpec> {
        ClkpllSelFbClkW::new(self, 2)
    }
}
#[doc = "clkpll_fbdv.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_fbdv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_fbdv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllFbdvSpec;
impl crate::RegisterSpec for ClkpllFbdvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_fbdv::R`](R) reader structure"]
impl crate::Readable for ClkpllFbdvSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_fbdv::W`](W) writer structure"]
impl crate::Writable for ClkpllFbdvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_fbdv to value 0x05"]
impl crate::Resettable for ClkpllFbdvSpec {
    const RESET_VALUE: u32 = 0x05;
}
