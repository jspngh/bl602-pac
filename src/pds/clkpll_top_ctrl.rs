#[doc = "Register `clkpll_top_ctrl` reader"]
pub type R = crate::R<ClkpllTopCtrlSpec>;
#[doc = "Register `clkpll_top_ctrl` writer"]
pub type W = crate::W<ClkpllTopCtrlSpec>;
#[doc = "Field `clkpll_postdiv` reader - "]
pub type ClkpllPostdivR = crate::FieldReader;
#[doc = "Field `clkpll_postdiv` writer - "]
pub type ClkpllPostdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `clkpll_refdiv_ratio` reader - "]
pub type ClkpllRefdivRatioR = crate::FieldReader;
#[doc = "Field `clkpll_refdiv_ratio` writer - "]
pub type ClkpllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `clkpll_xtal_rc32m_sel` reader - "]
pub type ClkpllXtalRc32mSelR = crate::BitReader;
#[doc = "Field `clkpll_xtal_rc32m_sel` writer - "]
pub type ClkpllXtalRc32mSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_refclk_sel` reader - "]
pub type ClkpllRefclkSelR = crate::BitReader;
#[doc = "Field `clkpll_refclk_sel` writer - "]
pub type ClkpllRefclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_vg11_sel` reader - "]
pub type ClkpllVg11SelR = crate::FieldReader;
#[doc = "Field `clkpll_vg11_sel` writer - "]
pub type ClkpllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_vg13_sel` reader - "]
pub type ClkpllVg13SelR = crate::FieldReader;
#[doc = "Field `clkpll_vg13_sel` writer - "]
pub type ClkpllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&self) -> ClkpllPostdivR {
        ClkpllPostdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&self) -> ClkpllRefdivRatioR {
        ClkpllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&self) -> ClkpllXtalRc32mSelR {
        ClkpllXtalRc32mSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&self) -> ClkpllRefclkSelR {
        ClkpllRefclkSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&self) -> ClkpllVg11SelR {
        ClkpllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&self) -> ClkpllVg13SelR {
        ClkpllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&mut self) -> ClkpllPostdivW<'_, ClkpllTopCtrlSpec> {
        ClkpllPostdivW::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&mut self) -> ClkpllRefdivRatioW<'_, ClkpllTopCtrlSpec> {
        ClkpllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&mut self) -> ClkpllXtalRc32mSelW<'_, ClkpllTopCtrlSpec> {
        ClkpllXtalRc32mSelW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&mut self) -> ClkpllRefclkSelW<'_, ClkpllTopCtrlSpec> {
        ClkpllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&mut self) -> ClkpllVg11SelW<'_, ClkpllTopCtrlSpec> {
        ClkpllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&mut self) -> ClkpllVg13SelW<'_, ClkpllTopCtrlSpec> {
        ClkpllVg13SelW::new(self, 24)
    }
}
#[doc = "clkpll_top_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_top_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_top_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllTopCtrlSpec;
impl crate::RegisterSpec for ClkpllTopCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_top_ctrl::R`](R) reader structure"]
impl crate::Readable for ClkpllTopCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_top_ctrl::W`](W) writer structure"]
impl crate::Writable for ClkpllTopCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_top_ctrl to value 0x0110_0414"]
impl crate::Resettable for ClkpllTopCtrlSpec {
    const RESET_VALUE: u32 = 0x0110_0414;
}
