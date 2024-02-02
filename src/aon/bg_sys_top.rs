#[doc = "Register `bg_sys_top` reader"]
pub type R = crate::R<BgSysTopSpec>;
#[doc = "Register `bg_sys_top` writer"]
pub type W = crate::W<BgSysTopSpec>;
#[doc = "Field `pmip_resv` reader - "]
pub type PmipResvR = crate::FieldReader;
#[doc = "Field `pmip_resv` writer - "]
pub type PmipResvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub type PuBgSysAonR = crate::BitReader;
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub type PuBgSysAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bg_sys_start_ctrl_aon` reader - "]
pub type BgSysStartCtrlAonR = crate::BitReader;
#[doc = "Field `bg_sys_start_ctrl_aon` writer - "]
pub type BgSysStartCtrlAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&self) -> PmipResvR {
        PmipResvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PuBgSysAonR {
        PuBgSysAonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&self) -> BgSysStartCtrlAonR {
        BgSysStartCtrlAonR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&mut self) -> PmipResvW<'_, BgSysTopSpec> {
        PmipResvW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&mut self) -> PuBgSysAonW<'_, BgSysTopSpec> {
        PuBgSysAonW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&mut self) -> BgSysStartCtrlAonW<'_, BgSysTopSpec> {
        BgSysStartCtrlAonW::new(self, 12)
    }
}
#[doc = "bg_sys_top.\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_sys_top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sys_top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgSysTopSpec;
impl crate::RegisterSpec for BgSysTopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bg_sys_top::R`](R) reader structure"]
impl crate::Readable for BgSysTopSpec {}
#[doc = "`write(|w| ..)` method takes [`bg_sys_top::W`](W) writer structure"]
impl crate::Writable for BgSysTopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bg_sys_top to value 0x1100"]
impl crate::Resettable for BgSysTopSpec {
    const RESET_VALUE: u32 = 0x1100;
}
