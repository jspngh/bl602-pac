#[doc = "Register `rfif_dig_ctrl` reader"]
pub type R = crate::R<RfifDigCtrlSpec>;
#[doc = "Register `rfif_dig_ctrl` writer"]
pub type W = crate::W<RfifDigCtrlSpec>;
#[doc = "Field `test_from_pad_en` reader - "]
pub type TestFromPadEnR = crate::BitReader;
#[doc = "Field `test_from_pad_en` writer - "]
pub type TestFromPadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_gc_from_pad_en` reader - "]
pub type TestGcFromPadEnR = crate::BitReader;
#[doc = "Field `test_gc_from_pad_en` writer - "]
pub type TestGcFromPadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfckg_rxclk_div2_mode` reader - "]
pub type RfckgRxclkDiv2ModeR = crate::BitReader;
#[doc = "Field `rfckg_rxclk_div2_mode` writer - "]
pub type RfckgRxclkDiv2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfif_int_lo_unlocked_mask` reader - "]
pub type RfifIntLoUnlockedMaskR = crate::BitReader;
#[doc = "Field `rfif_int_lo_unlocked_mask` writer - "]
pub type RfifIntLoUnlockedMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfif_ppud_cnt2` reader - "]
pub type RfifPpudCnt2R = crate::FieldReader<u16>;
#[doc = "Field `rfif_ppud_cnt2` writer - "]
pub type RfifPpudCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `rfif_ppud_cnt1` reader - "]
pub type RfifPpudCnt1R = crate::FieldReader;
#[doc = "Field `rfif_ppud_cnt1` writer - "]
pub type RfifPpudCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rfif_ppud_manaual_en` reader - "]
pub type RfifPpudManaualEnR = crate::BitReader;
#[doc = "Field `rfif_ppud_manaual_en` writer - "]
pub type RfifPpudManaualEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&self) -> TestFromPadEnR {
        TestFromPadEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&self) -> TestGcFromPadEnR {
        TestGcFromPadEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&self) -> RfckgRxclkDiv2ModeR {
        RfckgRxclkDiv2ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&self) -> RfifIntLoUnlockedMaskR {
        RfifIntLoUnlockedMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&self) -> RfifPpudCnt2R {
        RfifPpudCnt2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&self) -> RfifPpudCnt1R {
        RfifPpudCnt1R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&self) -> RfifPpudManaualEnR {
        RfifPpudManaualEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&mut self) -> TestFromPadEnW<'_, RfifDigCtrlSpec> {
        TestFromPadEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&mut self) -> TestGcFromPadEnW<'_, RfifDigCtrlSpec> {
        TestGcFromPadEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&mut self) -> RfckgRxclkDiv2ModeW<'_, RfifDigCtrlSpec> {
        RfckgRxclkDiv2ModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&mut self) -> RfifIntLoUnlockedMaskW<'_, RfifDigCtrlSpec> {
        RfifIntLoUnlockedMaskW::new(self, 3)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&mut self) -> RfifPpudCnt2W<'_, RfifDigCtrlSpec> {
        RfifPpudCnt2W::new(self, 16)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&mut self) -> RfifPpudCnt1W<'_, RfifDigCtrlSpec> {
        RfifPpudCnt1W::new(self, 25)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&mut self) -> RfifPpudManaualEnW<'_, RfifDigCtrlSpec> {
        RfifPpudManaualEnW::new(self, 30)
    }
}
#[doc = "rfif_dig_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_dig_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_dig_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifDigCtrlSpec;
impl crate::RegisterSpec for RfifDigCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfif_dig_ctrl::R`](R) reader structure"]
impl crate::Readable for RfifDigCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rfif_dig_ctrl::W`](W) writer structure"]
impl crate::Writable for RfifDigCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfif_dig_ctrl to value 0"]
impl crate::Resettable for RfifDigCtrlSpec {}
