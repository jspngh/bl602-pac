#[doc = "Register `sf_aes_key_r2_0` reader"]
pub struct R(crate::R<SF_AES_KEY_R2_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_KEY_R2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_AES_KEY_R2_0_SPEC>> for R {
    fn from(reader: crate::R<SF_AES_KEY_R2_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes_key_r2_0` writer"]
pub struct W(crate::W<SF_AES_KEY_R2_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_KEY_R2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<SF_AES_KEY_R2_0_SPEC>> for W {
    fn from(writer: crate::W<SF_AES_KEY_R2_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_key_r2_0` reader - "]
pub struct SF_AES_KEY_R2_0_R(crate::FieldReader<u32, u32>);
impl SF_AES_KEY_R2_0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_AES_KEY_R2_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_KEY_R2_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_key_r2_0` writer - "]
pub struct SF_AES_KEY_R2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_KEY_R2_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_0(&self) -> SF_AES_KEY_R2_0_R {
        SF_AES_KEY_R2_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_0(&mut self) -> SF_AES_KEY_R2_0_W {
        SF_AES_KEY_R2_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes_key_r2_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_0](index.html) module"]
pub struct SF_AES_KEY_R2_0_SPEC;
impl crate::RegisterSpec for SF_AES_KEY_R2_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes_key_r2_0::R](R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_0::W](W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_aes_key_r2_0 to value 0"]
impl crate::Resettable for SF_AES_KEY_R2_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
