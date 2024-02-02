#[doc = "Register `GPADC_32M_SRC_CTRL` reader"]
pub type R = crate::R<Gpadc32mSrcCtrlSpec>;
#[doc = "Register `GPADC_32M_SRC_CTRL` writer"]
pub type W = crate::W<Gpadc32mSrcCtrlSpec>;
#[doc = "Field `gpadc_32m_clk_div` reader - "]
pub type Gpadc32mClkDivR = crate::FieldReader;
#[doc = "Field `gpadc_32m_clk_div` writer - "]
pub type Gpadc32mClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpadc_32m_clk_sel` reader - "]
pub type Gpadc32mClkSelR = crate::BitReader;
#[doc = "Field `gpadc_32m_clk_sel` writer - "]
pub type Gpadc32mClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_32m_div_en` reader - "]
pub type Gpadc32mDivEnR = crate::BitReader;
#[doc = "Field `gpadc_32m_div_en` writer - "]
pub type Gpadc32mDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> Gpadc32mClkDivR {
        Gpadc32mClkDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> Gpadc32mClkSelR {
        Gpadc32mClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> Gpadc32mDivEnR {
        Gpadc32mDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&mut self) -> Gpadc32mClkDivW<'_, Gpadc32mSrcCtrlSpec> {
        Gpadc32mClkDivW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&mut self) -> Gpadc32mClkSelW<'_, Gpadc32mSrcCtrlSpec> {
        Gpadc32mClkSelW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&mut self) -> Gpadc32mDivEnW<'_, Gpadc32mSrcCtrlSpec> {
        Gpadc32mDivEnW::new(self, 8)
    }
}
#[doc = "Clock configuration for GPADC\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_32m_src_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_32m_src_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpadc32mSrcCtrlSpec;
impl crate::RegisterSpec for Gpadc32mSrcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_32m_src_ctrl::R`](R) reader structure"]
impl crate::Readable for Gpadc32mSrcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_32m_src_ctrl::W`](W) writer structure"]
impl crate::Writable for Gpadc32mSrcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_32M_SRC_CTRL to value 0x0102"]
impl crate::Resettable for Gpadc32mSrcCtrlSpec {
    const RESET_VALUE: u32 = 0x0102;
}
