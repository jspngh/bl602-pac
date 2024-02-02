#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub type R = crate::R<TzcGlbCtrl2Spec>;
#[doc = "Field `tzc_glb_gpio_0_lock` reader - "]
pub type TzcGlbGpio0LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_1_lock` reader - "]
pub type TzcGlbGpio1LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_2_lock` reader - "]
pub type TzcGlbGpio2LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_3_lock` reader - "]
pub type TzcGlbGpio3LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_4_lock` reader - "]
pub type TzcGlbGpio4LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_5_lock` reader - "]
pub type TzcGlbGpio5LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_6_lock` reader - "]
pub type TzcGlbGpio6LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_7_lock` reader - "]
pub type TzcGlbGpio7LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_8_lock` reader - "]
pub type TzcGlbGpio8LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_9_lock` reader - "]
pub type TzcGlbGpio9LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_10_lock` reader - "]
pub type TzcGlbGpio10LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_11_lock` reader - "]
pub type TzcGlbGpio11LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_12_lock` reader - "]
pub type TzcGlbGpio12LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_13_lock` reader - "]
pub type TzcGlbGpio13LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_14_lock` reader - "]
pub type TzcGlbGpio14LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_15_lock` reader - "]
pub type TzcGlbGpio15LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_16_lock` reader - "]
pub type TzcGlbGpio16LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_17_lock` reader - "]
pub type TzcGlbGpio17LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_18_lock` reader - "]
pub type TzcGlbGpio18LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_19_lock` reader - "]
pub type TzcGlbGpio19LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_20_lock` reader - "]
pub type TzcGlbGpio20LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_21_lock` reader - "]
pub type TzcGlbGpio21LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_22_lock` reader - "]
pub type TzcGlbGpio22LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_23_lock` reader - "]
pub type TzcGlbGpio23LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_24_lock` reader - "]
pub type TzcGlbGpio24LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_25_lock` reader - "]
pub type TzcGlbGpio25LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_26_lock` reader - "]
pub type TzcGlbGpio26LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_27_lock` reader - "]
pub type TzcGlbGpio27LockR = crate::BitReader;
#[doc = "Field `tzc_glb_gpio_28_lock` reader - "]
pub type TzcGlbGpio28LockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_0_lock(&self) -> TzcGlbGpio0LockR {
        TzcGlbGpio0LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_1_lock(&self) -> TzcGlbGpio1LockR {
        TzcGlbGpio1LockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_2_lock(&self) -> TzcGlbGpio2LockR {
        TzcGlbGpio2LockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_3_lock(&self) -> TzcGlbGpio3LockR {
        TzcGlbGpio3LockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_4_lock(&self) -> TzcGlbGpio4LockR {
        TzcGlbGpio4LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_5_lock(&self) -> TzcGlbGpio5LockR {
        TzcGlbGpio5LockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_gpio_6_lock(&self) -> TzcGlbGpio6LockR {
        TzcGlbGpio6LockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_gpio_7_lock(&self) -> TzcGlbGpio7LockR {
        TzcGlbGpio7LockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_gpio_8_lock(&self) -> TzcGlbGpio8LockR {
        TzcGlbGpio8LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_gpio_9_lock(&self) -> TzcGlbGpio9LockR {
        TzcGlbGpio9LockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_gpio_10_lock(&self) -> TzcGlbGpio10LockR {
        TzcGlbGpio10LockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_gpio_11_lock(&self) -> TzcGlbGpio11LockR {
        TzcGlbGpio11LockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_gpio_12_lock(&self) -> TzcGlbGpio12LockR {
        TzcGlbGpio12LockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_gpio_13_lock(&self) -> TzcGlbGpio13LockR {
        TzcGlbGpio13LockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_gpio_14_lock(&self) -> TzcGlbGpio14LockR {
        TzcGlbGpio14LockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_gpio_15_lock(&self) -> TzcGlbGpio15LockR {
        TzcGlbGpio15LockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_gpio_16_lock(&self) -> TzcGlbGpio16LockR {
        TzcGlbGpio16LockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_gpio_17_lock(&self) -> TzcGlbGpio17LockR {
        TzcGlbGpio17LockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_gpio_18_lock(&self) -> TzcGlbGpio18LockR {
        TzcGlbGpio18LockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_gpio_19_lock(&self) -> TzcGlbGpio19LockR {
        TzcGlbGpio19LockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_gpio_20_lock(&self) -> TzcGlbGpio20LockR {
        TzcGlbGpio20LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_gpio_21_lock(&self) -> TzcGlbGpio21LockR {
        TzcGlbGpio21LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_gpio_22_lock(&self) -> TzcGlbGpio22LockR {
        TzcGlbGpio22LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_gpio_23_lock(&self) -> TzcGlbGpio23LockR {
        TzcGlbGpio23LockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_gpio_24_lock(&self) -> TzcGlbGpio24LockR {
        TzcGlbGpio24LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_gpio_25_lock(&self) -> TzcGlbGpio25LockR {
        TzcGlbGpio25LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_gpio_26_lock(&self) -> TzcGlbGpio26LockR {
        TzcGlbGpio26LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_gpio_27_lock(&self) -> TzcGlbGpio27LockR {
        TzcGlbGpio27LockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_gpio_28_lock(&self) -> TzcGlbGpio28LockR {
        TzcGlbGpio28LockR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "tzc_glb_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl2Spec;
impl crate::RegisterSpec for TzcGlbCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl2Spec {}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TzcGlbCtrl2Spec {}
