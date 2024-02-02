#[doc = "Register `clkpll_rz` reader"]
pub type R = crate::R<ClkpllRzSpec>;
#[doc = "Register `clkpll_rz` writer"]
pub type W = crate::W<ClkpllRzSpec>;
#[doc = "Field `clkpll_c4_en` reader - "]
pub type ClkpllC4EnR = crate::BitReader;
#[doc = "Field `clkpll_c4_en` writer - "]
pub type ClkpllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_r4` reader - "]
pub type ClkpllR4R = crate::FieldReader;
#[doc = "Field `clkpll_r4` writer - "]
pub type ClkpllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_r4_short` reader - "]
pub type ClkpllR4ShortR = crate::BitReader;
#[doc = "Field `clkpll_r4_short` writer - "]
pub type ClkpllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_c3` reader - "]
pub type ClkpllC3R = crate::FieldReader;
#[doc = "Field `clkpll_c3` writer - "]
pub type ClkpllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_cz` reader - "]
pub type ClkpllCzR = crate::FieldReader;
#[doc = "Field `clkpll_cz` writer - "]
pub type ClkpllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_rz` reader - "]
pub type ClkpllRzR = crate::FieldReader;
#[doc = "Field `clkpll_rz` writer - "]
pub type ClkpllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&self) -> ClkpllC4EnR {
        ClkpllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&self) -> ClkpllR4R {
        ClkpllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&self) -> ClkpllR4ShortR {
        ClkpllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&self) -> ClkpllC3R {
        ClkpllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&self) -> ClkpllCzR {
        ClkpllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&self) -> ClkpllRzR {
        ClkpllRzR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&mut self) -> ClkpllC4EnW<'_, ClkpllRzSpec> {
        ClkpllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&mut self) -> ClkpllR4W<'_, ClkpllRzSpec> {
        ClkpllR4W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&mut self) -> ClkpllR4ShortW<'_, ClkpllRzSpec> {
        ClkpllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&mut self) -> ClkpllC3W<'_, ClkpllRzSpec> {
        ClkpllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&mut self) -> ClkpllCzW<'_, ClkpllRzSpec> {
        ClkpllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&mut self) -> ClkpllRzW<'_, ClkpllRzSpec> {
        ClkpllRzW::new(self, 16)
    }
}
#[doc = "clkpll_rz.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkpll_rz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpll_rz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkpllRzSpec;
impl crate::RegisterSpec for ClkpllRzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkpll_rz::R`](R) reader structure"]
impl crate::Readable for ClkpllRzSpec {}
#[doc = "`write(|w| ..)` method takes [`clkpll_rz::W`](W) writer structure"]
impl crate::Writable for ClkpllRzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clkpll_rz to value 0x0005_a020"]
impl crate::Resettable for ClkpllRzSpec {
    const RESET_VALUE: u32 = 0x0005_a020;
}
