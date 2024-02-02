#[doc = "Register `aon_common` reader"]
pub type R = crate::R<AonCommonSpec>;
#[doc = "Register `aon_common` writer"]
pub type W = crate::W<AonCommonSpec>;
#[doc = "Field `tmux_aon` reader - "]
pub type TmuxAonR = crate::FieldReader;
#[doc = "Field `tmux_aon` writer - "]
pub type TmuxAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ten_aon` reader - "]
pub type TenAonR = crate::BitReader;
#[doc = "Field `ten_aon` writer - "]
pub type TenAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal32k` reader - "]
pub type DtenXtal32kR = crate::BitReader;
#[doc = "Field `dten_xtal32k` writer - "]
pub type DtenXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal32k` reader - "]
pub type TenXtal32kR = crate::BitReader;
#[doc = "Field `ten_xtal32k` writer - "]
pub type TenXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_vddcore_aon` reader - "]
pub type TenVddcoreAonR = crate::BitReader;
#[doc = "Field `ten_vddcore_aon` writer - "]
pub type TenVddcoreAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo11soc_aon` reader - "]
pub type TenLdo11socAonR = crate::BitReader;
#[doc = "Field `ten_ldo11soc_aon` writer - "]
pub type TenLdo11socAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc18_0_aon` reader - "]
pub type TenDcdc18_0AonR = crate::BitReader;
#[doc = "Field `ten_dcdc18_0_aon` writer - "]
pub type TenDcdc18_0AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc18_1_aon` reader - "]
pub type TenDcdc18_1AonR = crate::BitReader;
#[doc = "Field `ten_dcdc18_1_aon` writer - "]
pub type TenDcdc18_1AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub type TenBgSysAonR = crate::BitReader;
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub type TenBgSysAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub type TenLdo15rfAonR = crate::BitReader;
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub type TenLdo15rfAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal_aon` reader - "]
pub type TenXtalAonR = crate::BitReader;
#[doc = "Field `ten_xtal_aon` writer - "]
pub type TenXtalAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal_aon` reader - "]
pub type DtenXtalAonR = crate::BitReader;
#[doc = "Field `dten_xtal_aon` writer - "]
pub type DtenXtalAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_mbg_aon` reader - "]
pub type TenMbgAonR = crate::BitReader;
#[doc = "Field `ten_mbg_aon` writer - "]
pub type TenMbgAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub type TenCipMiscAonR = crate::BitReader;
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub type TenCipMiscAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TmuxAonR {
        TmuxAonR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TenAonR {
        TenAonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DtenXtal32kR {
        DtenXtal32kR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TenXtal32kR {
        TenXtal32kR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&self) -> TenVddcoreAonR {
        TenVddcoreAonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&self) -> TenLdo11socAonR {
        TenLdo11socAonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TenDcdc18_0AonR {
        TenDcdc18_0AonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TenDcdc18_1AonR {
        TenDcdc18_1AonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TenBgSysAonR {
        TenBgSysAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TenLdo15rfAonR {
        TenLdo15rfAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TenXtalAonR {
        TenXtalAonR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DtenXtalAonR {
        DtenXtalAonR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TenMbgAonR {
        TenMbgAonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TenCipMiscAonR {
        TenCipMiscAonR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&mut self) -> TmuxAonW<'_, AonCommonSpec> {
        TmuxAonW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&mut self) -> TenAonW<'_, AonCommonSpec> {
        TenAonW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&mut self) -> DtenXtal32kW<'_, AonCommonSpec> {
        DtenXtal32kW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&mut self) -> TenXtal32kW<'_, AonCommonSpec> {
        TenXtal32kW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&mut self) -> TenVddcoreAonW<'_, AonCommonSpec> {
        TenVddcoreAonW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&mut self) -> TenLdo11socAonW<'_, AonCommonSpec> {
        TenLdo11socAonW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&mut self) -> TenDcdc18_0AonW<'_, AonCommonSpec> {
        TenDcdc18_0AonW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&mut self) -> TenDcdc18_1AonW<'_, AonCommonSpec> {
        TenDcdc18_1AonW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&mut self) -> TenBgSysAonW<'_, AonCommonSpec> {
        TenBgSysAonW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&mut self) -> TenLdo15rfAonW<'_, AonCommonSpec> {
        TenLdo15rfAonW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&mut self) -> TenXtalAonW<'_, AonCommonSpec> {
        TenXtalAonW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&mut self) -> DtenXtalAonW<'_, AonCommonSpec> {
        DtenXtalAonW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&mut self) -> TenMbgAonW<'_, AonCommonSpec> {
        TenMbgAonW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&mut self) -> TenCipMiscAonW<'_, AonCommonSpec> {
        TenCipMiscAonW::new(self, 20)
    }
}
#[doc = "aon_common.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_common::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_common::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonCommonSpec;
impl crate::RegisterSpec for AonCommonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_common::R`](R) reader structure"]
impl crate::Readable for AonCommonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_common::W`](W) writer structure"]
impl crate::Writable for AonCommonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets aon_common to value 0"]
impl crate::Resettable for AonCommonSpec {}
