#[doc = "Register `clkpll_output_en` reader"]
pub type R = crate::R<ClkpllOutputEnSpec>;
#[doc = "Register `clkpll_output_en` writer"]
pub type W = crate::W<ClkpllOutputEnSpec>;
#[doc = "Field `clkpll_en_480m` reader - "]
pub type ClkpllEn480mR = crate::BitReader;
#[doc = "Field `clkpll_en_480m` writer - "]
pub type ClkpllEn480mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_240m` reader - "]
pub type ClkpllEn240mR = crate::BitReader;
#[doc = "Field `clkpll_en_240m` writer - "]
pub type ClkpllEn240mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_192m` reader - "]
pub type ClkpllEn192mR = crate::BitReader;
#[doc = "Field `clkpll_en_192m` writer - "]
pub type ClkpllEn192mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_160m` reader - "]
pub type ClkpllEn160mR = crate::BitReader;
#[doc = "Field `clkpll_en_160m` writer - "]
pub type ClkpllEn160mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_120m` reader - "]
pub type ClkpllEn120mR = crate::BitReader;
#[doc = "Field `clkpll_en_120m` writer - "]
pub type ClkpllEn120mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_96m` reader - "]
pub type ClkpllEn96mR = crate::BitReader;
#[doc = "Field `clkpll_en_96m` writer - "]
pub type ClkpllEn96mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_80m` reader - "]
pub type ClkpllEn80mR = crate::BitReader;
#[doc = "Field `clkpll_en_80m` writer - "]
pub type ClkpllEn80mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_48m` reader - "]
pub type ClkpllEn48mR = crate::BitReader;
#[doc = "Field `clkpll_en_48m` writer - "]
pub type ClkpllEn48mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_32m` reader - "]
pub type ClkpllEn32mR = crate::BitReader;
#[doc = "Field `clkpll_en_32m` writer - "]
pub type ClkpllEn32mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_en_div2_480m` reader - "]
pub type ClkpllEnDiv2_480mR = crate::BitReader;
#[doc = "Field `clkpll_en_div2_480m` writer - "]
pub type ClkpllEnDiv2_480mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&self) -> ClkpllEn480mR {
        ClkpllEn480mR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&self) -> ClkpllEn240mR {
        ClkpllEn240mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&self) -> ClkpllEn192mR {
        ClkpllEn192mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&self) -> ClkpllEn160mR {
        ClkpllEn160mR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&self) -> ClkpllEn120mR {
        ClkpllEn120mR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&self) -> ClkpllEn96mR {
        ClkpllEn96mR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&self) -> ClkpllEn80mR {
        ClkpllEn80mR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&self) -> ClkpllEn48mR {
        ClkpllEn48mR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&self) -> ClkpllEn32mR {
        ClkpllEn32mR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&self) -> ClkpllEnDiv2_480mR {
        ClkpllEnDiv2_480mR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&mut self) -> ClkpllEn480mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn480mW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&mut self) -> ClkpllEn240mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn240mW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&mut self) -> ClkpllEn192mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn192mW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&mut self) -> ClkpllEn160mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn160mW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&mut self) -> ClkpllEn120mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn120mW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&mut self) -> ClkpllEn96mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn96mW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&mut self) -> ClkpllEn80mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn80mW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&mut self) -> ClkpllEn48mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn48mW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&mut self) -> ClkpllEn32mW<'_, ClkpllOutputEnSpec> {
        ClkpllEn32mW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&mut self) -> ClkpllEnDiv2_480mW<'_, ClkpllOutputEnSpec> {
        ClkpllEnDiv2_480mW::new(self, 9)
    }
}
#[doc = "clkpll_output_en.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_output_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_output_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllOutputEnSpec;
impl crate::RegisterSpec for ClkpllOutputEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_output_en::R`](R) reader structure"]
impl crate::Readable for ClkpllOutputEnSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_output_en::W`](W) writer structure"]
impl crate::Writable for ClkpllOutputEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_output_en to value 0x0100"]
impl crate::Resettable for ClkpllOutputEnSpec {
    const RESET_VALUE: u32 = 0x0100;
}
