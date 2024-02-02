#[doc = "Register `clkpll_cp` reader"]
pub type R = crate::R<ClkpllCpSpec>;
#[doc = "Register `clkpll_cp` writer"]
pub type W = crate::W<ClkpllCpSpec>;
#[doc = "Field `clkpll_sel_cp_bias` reader - "]
pub type ClkpllSelCpBiasR = crate::BitReader;
#[doc = "Field `clkpll_sel_cp_bias` writer - "]
pub type ClkpllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_icp_5u` reader - "]
pub type ClkpllIcp5uR = crate::FieldReader;
#[doc = "Field `clkpll_icp_5u` writer - "]
pub type ClkpllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_icp_1u` reader - "]
pub type ClkpllIcp1uR = crate::FieldReader;
#[doc = "Field `clkpll_icp_1u` writer - "]
pub type ClkpllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_int_frac_sw` reader - "]
pub type ClkpllIntFracSwR = crate::BitReader;
#[doc = "Field `clkpll_int_frac_sw` writer - "]
pub type ClkpllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_cp_startup_en` reader - "]
pub type ClkpllCpStartupEnR = crate::BitReader;
#[doc = "Field `clkpll_cp_startup_en` writer - "]
pub type ClkpllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_cp_opamp_en` reader - "]
pub type ClkpllCpOpampEnR = crate::BitReader;
#[doc = "Field `clkpll_cp_opamp_en` writer - "]
pub type ClkpllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&self) -> ClkpllSelCpBiasR {
        ClkpllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&self) -> ClkpllIcp5uR {
        ClkpllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&self) -> ClkpllIcp1uR {
        ClkpllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&self) -> ClkpllIntFracSwR {
        ClkpllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&self) -> ClkpllCpStartupEnR {
        ClkpllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&self) -> ClkpllCpOpampEnR {
        ClkpllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&mut self) -> ClkpllSelCpBiasW<'_, ClkpllCpSpec> {
        ClkpllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&mut self) -> ClkpllIcp5uW<'_, ClkpllCpSpec> {
        ClkpllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&mut self) -> ClkpllIcp1uW<'_, ClkpllCpSpec> {
        ClkpllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&mut self) -> ClkpllIntFracSwW<'_, ClkpllCpSpec> {
        ClkpllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&mut self) -> ClkpllCpStartupEnW<'_, ClkpllCpSpec> {
        ClkpllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&mut self) -> ClkpllCpOpampEnW<'_, ClkpllCpSpec> {
        ClkpllCpOpampEnW::new(self, 10)
    }
}
#[doc = "clkpll_cp.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_cp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_cp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllCpSpec;
impl crate::RegisterSpec for ClkpllCpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_cp::R`](R) reader structure"]
impl crate::Readable for ClkpllCpSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_cp::W`](W) writer structure"]
impl crate::Writable for ClkpllCpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_cp to value 0x0741"]
impl crate::Resettable for ClkpllCpSpec {
    const RESET_VALUE: u32 = 0x0741;
}
