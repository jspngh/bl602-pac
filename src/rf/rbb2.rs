#[doc = "Register `rbb2` reader"]
pub type R = crate::R<Rbb2Spec>;
#[doc = "Register `rbb2` writer"]
pub type W = crate::W<Rbb2Spec>;
#[doc = "Field `rbb_cap2_fc_q` reader - "]
pub type RbbCap2FcQR = crate::FieldReader;
#[doc = "Field `rbb_cap2_fc_q` writer - "]
pub type RbbCap2FcQW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rbb_cap2_fc_i` reader - "]
pub type RbbCap2FcIR = crate::FieldReader;
#[doc = "Field `rbb_cap2_fc_i` writer - "]
pub type RbbCap2FcIW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rbb_cap1_fc_q` reader - "]
pub type RbbCap1FcQR = crate::FieldReader;
#[doc = "Field `rbb_cap1_fc_q` writer - "]
pub type RbbCap1FcQW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rbb_cap1_fc_i` reader - "]
pub type RbbCap1FcIR = crate::FieldReader;
#[doc = "Field `rbb_cap1_fc_i` writer - "]
pub type RbbCap1FcIW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&self) -> RbbCap2FcQR {
        RbbCap2FcQR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&self) -> RbbCap2FcIR {
        RbbCap2FcIR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&self) -> RbbCap1FcQR {
        RbbCap1FcQR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i(&self) -> RbbCap1FcIR {
        RbbCap1FcIR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&mut self) -> RbbCap2FcQW<'_, Rbb2Spec> {
        RbbCap2FcQW::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&mut self) -> RbbCap2FcIW<'_, Rbb2Spec> {
        RbbCap2FcIW::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&mut self) -> RbbCap1FcQW<'_, Rbb2Spec> {
        RbbCap1FcQW::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i(&mut self) -> RbbCap1FcIW<'_, Rbb2Spec> {
        RbbCap1FcIW::new(self, 24)
    }
}
#[doc = "rbb2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbb2Spec;
impl crate::RegisterSpec for Rbb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb2::R`](R) reader structure"]
impl crate::Readable for Rbb2Spec {}
#[doc = "`write(|w| ..)` method takes [`rbb2::W`](W) writer structure"]
impl crate::Writable for Rbb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb2 to value 0"]
impl crate::Resettable for Rbb2Spec {}
