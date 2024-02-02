#[doc = "Register `sf_ctrl_2` reader"]
pub type R = crate::R<SfCtrl2Spec>;
#[doc = "Register `sf_ctrl_2` writer"]
pub type W = crate::W<SfCtrl2Spec>;
#[doc = "Field `sf_if_pad_sel` reader - "]
pub type SfIfPadSelR = crate::FieldReader;
#[doc = "Field `sf_if_pad_sel` writer - "]
pub type SfIfPadSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_if_pad_sel_lock` reader - "]
pub type SfIfPadSelLockR = crate::BitReader;
#[doc = "Field `sf_if_pad_sel_lock` writer - "]
pub type SfIfPadSelLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_dtr_en` reader - "]
pub type SfIfDtrEnR = crate::BitReader;
#[doc = "Field `sf_if_dtr_en` writer - "]
pub type SfIfDtrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_dqs_en` reader - "]
pub type SfIfDqsEnR = crate::BitReader;
#[doc = "Field `sf_if_dqs_en` writer - "]
pub type SfIfDqsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&self) -> SfIfPadSelR {
        SfIfPadSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&self) -> SfIfPadSelLockR {
        SfIfPadSelLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&self) -> SfIfDtrEnR {
        SfIfDtrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&self) -> SfIfDqsEnR {
        SfIfDqsEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&mut self) -> SfIfPadSelW<'_, SfCtrl2Spec> {
        SfIfPadSelW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&mut self) -> SfIfPadSelLockW<'_, SfCtrl2Spec> {
        SfIfPadSelLockW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&mut self) -> SfIfDtrEnW<'_, SfCtrl2Spec> {
        SfIfDtrEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&mut self) -> SfIfDqsEnW<'_, SfCtrl2Spec> {
        SfIfDqsEnW::new(self, 5)
    }
}
#[doc = "sf_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl2Spec;
impl crate::RegisterSpec for SfCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_2::R`](R) reader structure"]
impl crate::Readable for SfCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_2::W`](W) writer structure"]
impl crate::Writable for SfCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_ctrl_2 to value 0"]
impl crate::Resettable for SfCtrl2Spec {}
