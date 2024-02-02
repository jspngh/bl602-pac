#[doc = "Register `tzc_glb_ctrl_1` reader"]
pub type R = crate::R<TzcGlbCtrl1Spec>;
#[doc = "Field `tzc_glb_swrst_s20_lock` reader - "]
pub type TzcGlbSwrstS20LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s21_lock` reader - "]
pub type TzcGlbSwrstS21LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s22_lock` reader - "]
pub type TzcGlbSwrstS22LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s23_lock` reader - "]
pub type TzcGlbSwrstS23LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s24_lock` reader - "]
pub type TzcGlbSwrstS24LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s25_lock` reader - "]
pub type TzcGlbSwrstS25LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s26_lock` reader - "]
pub type TzcGlbSwrstS26LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s27_lock` reader - "]
pub type TzcGlbSwrstS27LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s28_lock` reader - "]
pub type TzcGlbSwrstS28LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s29_lock` reader - "]
pub type TzcGlbSwrstS29LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2a_lock` reader - "]
pub type TzcGlbSwrstS2aLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2b_lock` reader - "]
pub type TzcGlbSwrstS2bLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2c_lock` reader - "]
pub type TzcGlbSwrstS2cLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2d_lock` reader - "]
pub type TzcGlbSwrstS2dLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2e_lock` reader - "]
pub type TzcGlbSwrstS2eLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s2f_lock` reader - "]
pub type TzcGlbSwrstS2fLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s10_lock` reader - "]
pub type TzcGlbSwrstS10LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s11_lock` reader - "]
pub type TzcGlbSwrstS11LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s12_lock` reader - "]
pub type TzcGlbSwrstS12LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s13_lock` reader - "]
pub type TzcGlbSwrstS13LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s14_lock` reader - "]
pub type TzcGlbSwrstS14LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s15_lock` reader - "]
pub type TzcGlbSwrstS15LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s16_lock` reader - "]
pub type TzcGlbSwrstS16LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s17_lock` reader - "]
pub type TzcGlbSwrstS17LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s18_lock` reader - "]
pub type TzcGlbSwrstS18LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s19_lock` reader - "]
pub type TzcGlbSwrstS19LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1a_lock` reader - "]
pub type TzcGlbSwrstS1aLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1b_lock` reader - "]
pub type TzcGlbSwrstS1bLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1c_lock` reader - "]
pub type TzcGlbSwrstS1cLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1d_lock` reader - "]
pub type TzcGlbSwrstS1dLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1e_lock` reader - "]
pub type TzcGlbSwrstS1eLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s1f_lock` reader - "]
pub type TzcGlbSwrstS1fLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s20_lock(&self) -> TzcGlbSwrstS20LockR {
        TzcGlbSwrstS20LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s21_lock(&self) -> TzcGlbSwrstS21LockR {
        TzcGlbSwrstS21LockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s22_lock(&self) -> TzcGlbSwrstS22LockR {
        TzcGlbSwrstS22LockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s23_lock(&self) -> TzcGlbSwrstS23LockR {
        TzcGlbSwrstS23LockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s24_lock(&self) -> TzcGlbSwrstS24LockR {
        TzcGlbSwrstS24LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s25_lock(&self) -> TzcGlbSwrstS25LockR {
        TzcGlbSwrstS25LockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s26_lock(&self) -> TzcGlbSwrstS26LockR {
        TzcGlbSwrstS26LockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s27_lock(&self) -> TzcGlbSwrstS27LockR {
        TzcGlbSwrstS27LockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s28_lock(&self) -> TzcGlbSwrstS28LockR {
        TzcGlbSwrstS28LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s29_lock(&self) -> TzcGlbSwrstS29LockR {
        TzcGlbSwrstS29LockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2a_lock(&self) -> TzcGlbSwrstS2aLockR {
        TzcGlbSwrstS2aLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2b_lock(&self) -> TzcGlbSwrstS2bLockR {
        TzcGlbSwrstS2bLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2c_lock(&self) -> TzcGlbSwrstS2cLockR {
        TzcGlbSwrstS2cLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2d_lock(&self) -> TzcGlbSwrstS2dLockR {
        TzcGlbSwrstS2dLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2e_lock(&self) -> TzcGlbSwrstS2eLockR {
        TzcGlbSwrstS2eLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2f_lock(&self) -> TzcGlbSwrstS2fLockR {
        TzcGlbSwrstS2fLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s10_lock(&self) -> TzcGlbSwrstS10LockR {
        TzcGlbSwrstS10LockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s11_lock(&self) -> TzcGlbSwrstS11LockR {
        TzcGlbSwrstS11LockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s12_lock(&self) -> TzcGlbSwrstS12LockR {
        TzcGlbSwrstS12LockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s13_lock(&self) -> TzcGlbSwrstS13LockR {
        TzcGlbSwrstS13LockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s14_lock(&self) -> TzcGlbSwrstS14LockR {
        TzcGlbSwrstS14LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s15_lock(&self) -> TzcGlbSwrstS15LockR {
        TzcGlbSwrstS15LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s16_lock(&self) -> TzcGlbSwrstS16LockR {
        TzcGlbSwrstS16LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s17_lock(&self) -> TzcGlbSwrstS17LockR {
        TzcGlbSwrstS17LockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s18_lock(&self) -> TzcGlbSwrstS18LockR {
        TzcGlbSwrstS18LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s19_lock(&self) -> TzcGlbSwrstS19LockR {
        TzcGlbSwrstS19LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1a_lock(&self) -> TzcGlbSwrstS1aLockR {
        TzcGlbSwrstS1aLockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1b_lock(&self) -> TzcGlbSwrstS1bLockR {
        TzcGlbSwrstS1bLockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1c_lock(&self) -> TzcGlbSwrstS1cLockR {
        TzcGlbSwrstS1cLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1d_lock(&self) -> TzcGlbSwrstS1dLockR {
        TzcGlbSwrstS1dLockR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1e_lock(&self) -> TzcGlbSwrstS1eLockR {
        TzcGlbSwrstS1eLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1f_lock(&self) -> TzcGlbSwrstS1fLockR {
        TzcGlbSwrstS1fLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "tzc_glb_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl1Spec;
impl crate::RegisterSpec for TzcGlbCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl1Spec {}
#[doc = "`reset()` method sets tzc_glb_ctrl_1 to value 0"]
impl crate::Resettable for TzcGlbCtrl1Spec {}
