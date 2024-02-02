#[doc = "Register `TMSR2` reader"]
pub type R = crate::R<Tmsr2Spec>;
#[doc = "Field `tmsr_0` reader - "]
pub type Tmsr0R = crate::BitReader;
#[doc = "Field `tmsr_1` reader - "]
pub type Tmsr1R = crate::BitReader;
#[doc = "Field `tmsr_2` reader - "]
pub type Tmsr2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> Tmsr0R {
        Tmsr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> Tmsr1R {
        Tmsr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&self) -> Tmsr2R {
        Tmsr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "TMSR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmsr2Spec;
impl crate::RegisterSpec for Tmsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmsr2::R`](R) reader structure"]
impl crate::Readable for Tmsr2Spec {}
#[doc = "`reset()` method sets TMSR2 to value 0"]
impl crate::Resettable for Tmsr2Spec {}
