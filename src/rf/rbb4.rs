#[doc = "Register `rbb4` reader"]
pub type R = crate::R<Rbb4Spec>;
#[doc = "Register `rbb4` writer"]
pub type W = crate::W<Rbb4Spec>;
#[doc = "Field `rbb_pkdet_vth` reader - "]
pub type RbbPkdetVthR = crate::FieldReader;
#[doc = "Field `rbb_pkdet_vth` writer - "]
pub type RbbPkdetVthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rbb_pkdet_out_rstn` reader - "]
pub type RbbPkdetOutRstnR = crate::BitReader;
#[doc = "Field `rbb_pkdet_out_rstn` writer - "]
pub type RbbPkdetOutRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_pkdet_en` reader - "]
pub type RbbPkdetEnR = crate::BitReader;
#[doc = "Field `rbb_pkdet_en` writer - "]
pub type RbbPkdetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_pkdet_out_rstn_hw` reader - "]
pub type RbbPkdetOutRstnHwR = crate::BitReader;
#[doc = "Field `rbb_pkdet_out_rstn_hw` writer - "]
pub type RbbPkdetOutRstnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rbb_pkdet_en_hw` reader - "]
pub type RbbPkdetEnHwR = crate::BitReader;
#[doc = "Field `rbb_pkdet_en_hw` writer - "]
pub type RbbPkdetEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pkdet_out_raw` reader - "]
pub type PkdetOutRawR = crate::BitReader;
#[doc = "Field `pkdet_out_raw` writer - "]
pub type PkdetOutRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pkdet_out_latch` reader - "]
pub type PkdetOutLatchR = crate::BitReader;
#[doc = "Field `pkdet_out_latch` writer - "]
pub type PkdetOutLatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RbbPkdetVthR {
        RbbPkdetVthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RbbPkdetOutRstnR {
        RbbPkdetOutRstnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RbbPkdetEnR {
        RbbPkdetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RbbPkdetOutRstnHwR {
        RbbPkdetOutRstnHwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RbbPkdetEnHwR {
        RbbPkdetEnHwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PkdetOutRawR {
        PkdetOutRawR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PkdetOutLatchR {
        PkdetOutLatchR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&mut self) -> RbbPkdetVthW<'_, Rbb4Spec> {
        RbbPkdetVthW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RbbPkdetOutRstnW<'_, Rbb4Spec> {
        RbbPkdetOutRstnW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&mut self) -> RbbPkdetEnW<'_, Rbb4Spec> {
        RbbPkdetEnW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RbbPkdetOutRstnHwW<'_, Rbb4Spec> {
        RbbPkdetOutRstnHwW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&mut self) -> RbbPkdetEnHwW<'_, Rbb4Spec> {
        RbbPkdetEnHwW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&mut self) -> PkdetOutRawW<'_, Rbb4Spec> {
        PkdetOutRawW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&mut self) -> PkdetOutLatchW<'_, Rbb4Spec> {
        PkdetOutLatchW::new(self, 24)
    }
}
#[doc = "rbb4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbb4Spec;
impl crate::RegisterSpec for Rbb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb4::R`](R) reader structure"]
impl crate::Readable for Rbb4Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb4::W`](W) writer structure"]
impl crate::Writable for Rbb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb4 to value 0"]
impl crate::Resettable for Rbb4Spec {}
