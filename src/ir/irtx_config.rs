#[doc = "Register `irtx_config` reader"]
pub type R = crate::R<IrtxConfigSpec>;
#[doc = "Register `irtx_config` writer"]
pub type W = crate::W<IrtxConfigSpec>;
#[doc = "Field `cr_irtx_en` reader - "]
pub type CrIrtxEnR = crate::BitReader;
#[doc = "Field `cr_irtx_en` writer - "]
pub type CrIrtxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_out_inv` reader - "]
pub type CrIrtxOutInvR = crate::BitReader;
#[doc = "Field `cr_irtx_out_inv` writer - "]
pub type CrIrtxOutInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_mod_en` reader - "]
pub type CrIrtxModEnR = crate::BitReader;
#[doc = "Field `cr_irtx_mod_en` writer - "]
pub type CrIrtxModEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_swm_en` reader - "]
pub type CrIrtxSwmEnR = crate::BitReader;
#[doc = "Field `cr_irtx_swm_en` writer - "]
pub type CrIrtxSwmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_data_en` reader - "]
pub type CrIrtxDataEnR = crate::BitReader;
#[doc = "Field `cr_irtx_data_en` writer - "]
pub type CrIrtxDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_logic0_hl_inv` reader - "]
pub type CrIrtxLogic0HlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_logic0_hl_inv` writer - "]
pub type CrIrtxLogic0HlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_logic1_hl_inv` reader - "]
pub type CrIrtxLogic1HlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_logic1_hl_inv` writer - "]
pub type CrIrtxLogic1HlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_head_en` reader - "]
pub type CrIrtxHeadEnR = crate::BitReader;
#[doc = "Field `cr_irtx_head_en` writer - "]
pub type CrIrtxHeadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_head_hl_inv` reader - "]
pub type CrIrtxHeadHlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_head_hl_inv` writer - "]
pub type CrIrtxHeadHlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_tail_en` reader - "]
pub type CrIrtxTailEnR = crate::BitReader;
#[doc = "Field `cr_irtx_tail_en` writer - "]
pub type CrIrtxTailEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_tail_hl_inv` reader - "]
pub type CrIrtxTailHlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_tail_hl_inv` writer - "]
pub type CrIrtxTailHlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_data_num` reader - "]
pub type CrIrtxDataNumR = crate::FieldReader;
#[doc = "Field `cr_irtx_data_num` writer - "]
pub type CrIrtxDataNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&self) -> CrIrtxEnR {
        CrIrtxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&self) -> CrIrtxOutInvR {
        CrIrtxOutInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&self) -> CrIrtxModEnR {
        CrIrtxModEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&self) -> CrIrtxSwmEnR {
        CrIrtxSwmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&self) -> CrIrtxDataEnR {
        CrIrtxDataEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&self) -> CrIrtxLogic0HlInvR {
        CrIrtxLogic0HlInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&self) -> CrIrtxLogic1HlInvR {
        CrIrtxLogic1HlInvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&self) -> CrIrtxHeadEnR {
        CrIrtxHeadEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&self) -> CrIrtxHeadHlInvR {
        CrIrtxHeadHlInvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&self) -> CrIrtxTailEnR {
        CrIrtxTailEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&self) -> CrIrtxTailHlInvR {
        CrIrtxTailHlInvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&self) -> CrIrtxDataNumR {
        CrIrtxDataNumR::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&mut self) -> CrIrtxEnW<'_, IrtxConfigSpec> {
        CrIrtxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&mut self) -> CrIrtxOutInvW<'_, IrtxConfigSpec> {
        CrIrtxOutInvW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&mut self) -> CrIrtxModEnW<'_, IrtxConfigSpec> {
        CrIrtxModEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&mut self) -> CrIrtxSwmEnW<'_, IrtxConfigSpec> {
        CrIrtxSwmEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&mut self) -> CrIrtxDataEnW<'_, IrtxConfigSpec> {
        CrIrtxDataEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&mut self) -> CrIrtxLogic0HlInvW<'_, IrtxConfigSpec> {
        CrIrtxLogic0HlInvW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&mut self) -> CrIrtxLogic1HlInvW<'_, IrtxConfigSpec> {
        CrIrtxLogic1HlInvW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&mut self) -> CrIrtxHeadEnW<'_, IrtxConfigSpec> {
        CrIrtxHeadEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&mut self) -> CrIrtxHeadHlInvW<'_, IrtxConfigSpec> {
        CrIrtxHeadHlInvW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&mut self) -> CrIrtxTailEnW<'_, IrtxConfigSpec> {
        CrIrtxTailEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&mut self) -> CrIrtxTailHlInvW<'_, IrtxConfigSpec> {
        CrIrtxTailHlInvW::new(self, 11)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&mut self) -> CrIrtxDataNumW<'_, IrtxConfigSpec> {
        CrIrtxDataNumW::new(self, 12)
    }
}
#[doc = "irtx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxConfigSpec;
impl crate::RegisterSpec for IrtxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_config::R`](R) reader structure"]
impl crate::Readable for IrtxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_config::W`](W) writer structure"]
impl crate::Writable for IrtxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_config to value 0x0001_f510"]
impl crate::Resettable for IrtxConfigSpec {
    const RESET_VALUE: u32 = 0x0001_f510;
}
