#[doc = "Register `dfe_ctrl_17` reader"]
pub type R = crate::R<DfeCtrl17Spec>;
#[doc = "Register `dfe_ctrl_17` writer"]
pub type W = crate::W<DfeCtrl17Spec>;
#[doc = "Field `rf_tbb_ind_gc8` reader - "]
pub type RfTbbIndGc8R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc8` writer - "]
pub type RfTbbIndGc8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc9` reader - "]
pub type RfTbbIndGc9R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc9` writer - "]
pub type RfTbbIndGc9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc10` reader - "]
pub type RfTbbIndGc10R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc10` writer - "]
pub type RfTbbIndGc10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc11` reader - "]
pub type RfTbbIndGc11R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc11` writer - "]
pub type RfTbbIndGc11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc12` reader - "]
pub type RfTbbIndGc12R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc12` writer - "]
pub type RfTbbIndGc12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc13` reader - "]
pub type RfTbbIndGc13R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc13` writer - "]
pub type RfTbbIndGc13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc14` reader - "]
pub type RfTbbIndGc14R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc14` writer - "]
pub type RfTbbIndGc14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rf_tbb_ind_gc15` reader - "]
pub type RfTbbIndGc15R = crate::FieldReader;
#[doc = "Field `rf_tbb_ind_gc15` writer - "]
pub type RfTbbIndGc15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&self) -> RfTbbIndGc8R {
        RfTbbIndGc8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&self) -> RfTbbIndGc9R {
        RfTbbIndGc9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&self) -> RfTbbIndGc10R {
        RfTbbIndGc10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&self) -> RfTbbIndGc11R {
        RfTbbIndGc11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&self) -> RfTbbIndGc12R {
        RfTbbIndGc12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&self) -> RfTbbIndGc13R {
        RfTbbIndGc13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&self) -> RfTbbIndGc14R {
        RfTbbIndGc14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&self) -> RfTbbIndGc15R {
        RfTbbIndGc15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&mut self) -> RfTbbIndGc8W<'_, DfeCtrl17Spec> {
        RfTbbIndGc8W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&mut self) -> RfTbbIndGc9W<'_, DfeCtrl17Spec> {
        RfTbbIndGc9W::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&mut self) -> RfTbbIndGc10W<'_, DfeCtrl17Spec> {
        RfTbbIndGc10W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&mut self) -> RfTbbIndGc11W<'_, DfeCtrl17Spec> {
        RfTbbIndGc11W::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&mut self) -> RfTbbIndGc12W<'_, DfeCtrl17Spec> {
        RfTbbIndGc12W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&mut self) -> RfTbbIndGc13W<'_, DfeCtrl17Spec> {
        RfTbbIndGc13W::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&mut self) -> RfTbbIndGc14W<'_, DfeCtrl17Spec> {
        RfTbbIndGc14W::new(self, 24)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&mut self) -> RfTbbIndGc15W<'_, DfeCtrl17Spec> {
        RfTbbIndGc15W::new(self, 28)
    }
}
#[doc = "dfe_ctrl_17.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl17Spec;
impl crate::RegisterSpec for DfeCtrl17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_17::R`](R) reader structure"]
impl crate::Readable for DfeCtrl17Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_17::W`](W) writer structure"]
impl crate::Writable for DfeCtrl17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_17 to value 0"]
impl crate::Resettable for DfeCtrl17Spec {}
