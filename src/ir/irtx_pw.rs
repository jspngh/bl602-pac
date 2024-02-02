#[doc = "Register `irtx_pw` reader"]
pub type R = crate::R<IrtxPwSpec>;
#[doc = "Register `irtx_pw` writer"]
pub type W = crate::W<IrtxPwSpec>;
#[doc = "Field `cr_irtx_logic0_ph0_w` reader - "]
pub type CrIrtxLogic0Ph0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic0_ph0_w` writer - "]
pub type CrIrtxLogic0Ph0WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_logic0_ph1_w` reader - "]
pub type CrIrtxLogic0Ph1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic0_ph1_w` writer - "]
pub type CrIrtxLogic0Ph1WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_logic1_ph0_w` reader - "]
pub type CrIrtxLogic1Ph0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic1_ph0_w` writer - "]
pub type CrIrtxLogic1Ph0WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_logic1_ph1_w` reader - "]
pub type CrIrtxLogic1Ph1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic1_ph1_w` writer - "]
pub type CrIrtxLogic1Ph1WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_head_ph0_w` reader - "]
pub type CrIrtxHeadPh0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_head_ph0_w` writer - "]
pub type CrIrtxHeadPh0WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_head_ph1_w` reader - "]
pub type CrIrtxHeadPh1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_head_ph1_w` writer - "]
pub type CrIrtxHeadPh1WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_tail_ph0_w` reader - "]
pub type CrIrtxTailPh0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_tail_ph0_w` writer - "]
pub type CrIrtxTailPh0WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_irtx_tail_ph1_w` reader - "]
pub type CrIrtxTailPh1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_tail_ph1_w` writer - "]
pub type CrIrtxTailPh1WW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&self) -> CrIrtxLogic0Ph0WR {
        CrIrtxLogic0Ph0WR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&self) -> CrIrtxLogic0Ph1WR {
        CrIrtxLogic0Ph1WR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&self) -> CrIrtxLogic1Ph0WR {
        CrIrtxLogic1Ph0WR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&self) -> CrIrtxLogic1Ph1WR {
        CrIrtxLogic1Ph1WR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&self) -> CrIrtxHeadPh0WR {
        CrIrtxHeadPh0WR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&self) -> CrIrtxHeadPh1WR {
        CrIrtxHeadPh1WR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&self) -> CrIrtxTailPh0WR {
        CrIrtxTailPh0WR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&self) -> CrIrtxTailPh1WR {
        CrIrtxTailPh1WR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&mut self) -> CrIrtxLogic0Ph0WW<'_, IrtxPwSpec> {
        CrIrtxLogic0Ph0WW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&mut self) -> CrIrtxLogic0Ph1WW<'_, IrtxPwSpec> {
        CrIrtxLogic0Ph1WW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&mut self) -> CrIrtxLogic1Ph0WW<'_, IrtxPwSpec> {
        CrIrtxLogic1Ph0WW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&mut self) -> CrIrtxLogic1Ph1WW<'_, IrtxPwSpec> {
        CrIrtxLogic1Ph1WW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&mut self) -> CrIrtxHeadPh0WW<'_, IrtxPwSpec> {
        CrIrtxHeadPh0WW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&mut self) -> CrIrtxHeadPh1WW<'_, IrtxPwSpec> {
        CrIrtxHeadPh1WW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&mut self) -> CrIrtxTailPh0WW<'_, IrtxPwSpec> {
        CrIrtxTailPh0WW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&mut self) -> CrIrtxTailPh1WW<'_, IrtxPwSpec> {
        CrIrtxTailPh1WW::new(self, 28)
    }
}
#[doc = "irtx_pw.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxPwSpec;
impl crate::RegisterSpec for IrtxPwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_pw::R`](R) reader structure"]
impl crate::Readable for IrtxPwSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_pw::W`](W) writer structure"]
impl crate::Writable for IrtxPwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_pw to value 0x007f_2000"]
impl crate::Resettable for IrtxPwSpec {
    const RESET_VALUE: u32 = 0x007f_2000;
}
