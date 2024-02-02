#[doc = "Register `se_sha_0_ctrl` reader"]
pub type R = crate::R<SeSha0CtrlSpec>;
#[doc = "Register `se_sha_0_ctrl` writer"]
pub type W = crate::W<SeSha0CtrlSpec>;
#[doc = "Field `se_sha_0_busy` reader - "]
pub type SeSha0BusyR = crate::BitReader;
#[doc = "Field `se_sha_0_trig_1t` reader - "]
pub type SeSha0Trig1tR = crate::BitReader;
#[doc = "Field `se_sha_0_trig_1t` writer - "]
pub type SeSha0Trig1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_mode` reader - "]
pub type SeSha0ModeR = crate::FieldReader;
#[doc = "Field `se_sha_0_mode` writer - "]
pub type SeSha0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `se_sha_0_en` reader - "]
pub type SeSha0EnR = crate::BitReader;
#[doc = "Field `se_sha_0_en` writer - "]
pub type SeSha0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_hash_sel` reader - "]
pub type SeSha0HashSelR = crate::BitReader;
#[doc = "Field `se_sha_0_hash_sel` writer - "]
pub type SeSha0HashSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int` reader - "]
pub type SeSha0IntR = crate::BitReader;
#[doc = "Field `se_sha_0_int_clr_1t` reader - "]
pub type SeSha0IntClr1tR = crate::BitReader;
#[doc = "Field `se_sha_0_int_clr_1t` writer - "]
pub type SeSha0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int_set_1t` reader - "]
pub type SeSha0IntSet1tR = crate::BitReader;
#[doc = "Field `se_sha_0_int_set_1t` writer - "]
pub type SeSha0IntSet1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int_mask` reader - "]
pub type SeSha0IntMaskR = crate::BitReader;
#[doc = "Field `se_sha_0_int_mask` writer - "]
pub type SeSha0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_link_mode` reader - "]
pub type SeSha0LinkModeR = crate::BitReader;
#[doc = "Field `se_sha_0_link_mode` writer - "]
pub type SeSha0LinkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_msg_len` reader - "]
pub type SeSha0MsgLenR = crate::FieldReader<u16>;
#[doc = "Field `se_sha_0_msg_len` writer - "]
pub type SeSha0MsgLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_busy(&self) -> SeSha0BusyR {
        SeSha0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_0_trig_1t(&self) -> SeSha0Trig1tR {
        SeSha0Trig1tR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn se_sha_0_mode(&self) -> SeSha0ModeR {
        SeSha0ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_sha_0_en(&self) -> SeSha0EnR {
        SeSha0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&self) -> SeSha0HashSelR {
        SeSha0HashSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_sha_0_int(&self) -> SeSha0IntR {
        SeSha0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_sha_0_int_clr_1t(&self) -> SeSha0IntClr1tR {
        SeSha0IntClr1tR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_sha_0_int_set_1t(&self) -> SeSha0IntSet1tR {
        SeSha0IntSet1tR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&self) -> SeSha0IntMaskR {
        SeSha0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&self) -> SeSha0LinkModeR {
        SeSha0LinkModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&self) -> SeSha0MsgLenR {
        SeSha0MsgLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_0_trig_1t(&mut self) -> SeSha0Trig1tW<'_, SeSha0CtrlSpec> {
        SeSha0Trig1tW::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn se_sha_0_mode(&mut self) -> SeSha0ModeW<'_, SeSha0CtrlSpec> {
        SeSha0ModeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_sha_0_en(&mut self) -> SeSha0EnW<'_, SeSha0CtrlSpec> {
        SeSha0EnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&mut self) -> SeSha0HashSelW<'_, SeSha0CtrlSpec> {
        SeSha0HashSelW::new(self, 6)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_sha_0_int_clr_1t(&mut self) -> SeSha0IntClr1tW<'_, SeSha0CtrlSpec> {
        SeSha0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_sha_0_int_set_1t(&mut self) -> SeSha0IntSet1tW<'_, SeSha0CtrlSpec> {
        SeSha0IntSet1tW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&mut self) -> SeSha0IntMaskW<'_, SeSha0CtrlSpec> {
        SeSha0IntMaskW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&mut self) -> SeSha0LinkModeW<'_, SeSha0CtrlSpec> {
        SeSha0LinkModeW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&mut self) -> SeSha0MsgLenW<'_, SeSha0CtrlSpec> {
        SeSha0MsgLenW::new(self, 16)
    }
}
#[doc = "se_sha_0_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0CtrlSpec;
impl crate::RegisterSpec for SeSha0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_ctrl::R`](R) reader structure"]
impl crate::Readable for SeSha0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_ctrl::W`](W) writer structure"]
impl crate::Writable for SeSha0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_sha_0_ctrl to value 0"]
impl crate::Resettable for SeSha0CtrlSpec {}
