#[doc = "Register `clkpll_vco` reader"]
pub type R = crate::R<ClkpllVcoSpec>;
#[doc = "Register `clkpll_vco` writer"]
pub type W = crate::W<ClkpllVcoSpec>;
#[doc = "Field `clkpll_vco_speed` reader - "]
pub type ClkpllVcoSpeedR = crate::FieldReader;
#[doc = "Field `clkpll_vco_speed` writer - "]
pub type ClkpllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clkpll_shrtr` reader - "]
pub type ClkpllShrtrR = crate::BitReader;
#[doc = "Field `clkpll_shrtr` writer - "]
pub type ClkpllShrtrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&self) -> ClkpllVcoSpeedR {
        ClkpllVcoSpeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&self) -> ClkpllShrtrR {
        ClkpllShrtrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&mut self) -> ClkpllVcoSpeedW<'_, ClkpllVcoSpec> {
        ClkpllVcoSpeedW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&mut self) -> ClkpllShrtrW<'_, ClkpllVcoSpec> {
        ClkpllShrtrW::new(self, 3)
    }
}
#[doc = "clkpll_vco.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_vco::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_vco::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllVcoSpec;
impl crate::RegisterSpec for ClkpllVcoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_vco::R`](R) reader structure"]
impl crate::Readable for ClkpllVcoSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_vco::W`](W) writer structure"]
impl crate::Writable for ClkpllVcoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_vco to value 0x07"]
impl crate::Resettable for ClkpllVcoSpec {
    const RESET_VALUE: u32 = 0x07;
}
