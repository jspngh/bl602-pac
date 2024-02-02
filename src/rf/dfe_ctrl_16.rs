#[doc = "Register `dfe_ctrl_16` reader"]
pub type R = crate::R<DfeCtrl16Spec>;
#[doc = "Register `dfe_ctrl_16` writer"]
pub type W = crate::W<DfeCtrl16Spec>;
#[doc = "Field `rf_tbb_ind_gc0` reader - "]
pub type RfTbbIndGc0R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc0` writer - "]
pub type RfTbbIndGc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc1` reader - "]
pub type RfTbbIndGc1R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc1` writer - "]
pub type RfTbbIndGc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc2` reader - "]
pub type RfTbbIndGc2R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc2` writer - "]
pub type RfTbbIndGc2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc3` reader - "]
pub type RfTbbIndGc3R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc3` writer - "]
pub type RfTbbIndGc3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc4` reader - "]
pub type RfTbbIndGc4R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc4` writer - "]
pub type RfTbbIndGc4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc5` reader - "]
pub type RfTbbIndGc5R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc5` writer - "]
pub type RfTbbIndGc5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc6` reader - "]
pub type RfTbbIndGc6R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc6` writer - "]
pub type RfTbbIndGc6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc7` reader - "]
pub type RfTbbIndGc7R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc7` writer - "]
pub type RfTbbIndGc7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&self) -> RfTbbIndGc0R {
        RfTbbIndGc0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&self) -> RfTbbIndGc1R {
        RfTbbIndGc1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&self) -> RfTbbIndGc2R {
        RfTbbIndGc2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&self) -> RfTbbIndGc3R {
        RfTbbIndGc3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&self) -> RfTbbIndGc4R {
        RfTbbIndGc4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&self) -> RfTbbIndGc5R {
        RfTbbIndGc5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&self) -> RfTbbIndGc6R {
        RfTbbIndGc6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&self) -> RfTbbIndGc7R {
        RfTbbIndGc7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&mut self) -> RfTbbIndGc0W<'_, DfeCtrl16Spec> {
        RfTbbIndGc0W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&mut self) -> RfTbbIndGc1W<'_, DfeCtrl16Spec> {
        RfTbbIndGc1W::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&mut self) -> RfTbbIndGc2W<'_, DfeCtrl16Spec> {
        RfTbbIndGc2W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&mut self) -> RfTbbIndGc3W<'_, DfeCtrl16Spec> {
        RfTbbIndGc3W::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&mut self) -> RfTbbIndGc4W<'_, DfeCtrl16Spec> {
        RfTbbIndGc4W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&mut self) -> RfTbbIndGc5W<'_, DfeCtrl16Spec> {
        RfTbbIndGc5W::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&mut self) -> RfTbbIndGc6W<'_, DfeCtrl16Spec> {
        RfTbbIndGc6W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&mut self) -> RfTbbIndGc7W<'_, DfeCtrl16Spec> {
        RfTbbIndGc7W::new(self, 28)
    }
}
#[doc = "dfe_ctrl_16.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl16Spec;
impl crate::RegisterSpec for DfeCtrl16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_16::R`](R) reader structure"]
impl crate::Readable for DfeCtrl16Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_16::W`](W) writer structure"]
impl crate::Writable for DfeCtrl16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_16 to value 0"]
impl crate::Resettable for DfeCtrl16Spec {}
