#[doc = "Register `lo` reader"]
pub type R = crate::R<LoSpec>;
#[doc = "Register `lo` writer"]
pub type W = crate::W<LoSpec>;
#[doc = "Field `lo_lf_rz_hw` reader - "]
pub type LoLfRzHwR = crate::FieldReader;
#[doc = "Field `lo_lf_rz_hw` writer - "]
pub type LoLfRzHwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_r4_hw` reader - "]
pub type LoLfR4HwR = crate::FieldReader;
#[doc = "Field `lo_lf_r4_hw` writer - "]
pub type LoLfR4HwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_cz_hw` reader - "]
pub type LoLfCzHwR = crate::FieldReader;
#[doc = "Field `lo_lf_cz_hw` writer - "]
pub type LoLfCzHwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_rz` reader - "]
pub type LoLfRzR = crate::FieldReader;
#[doc = "Field `lo_lf_rz` writer - "]
pub type LoLfRzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_cz` reader - "]
pub type LoLfCzR = crate::FieldReader;
#[doc = "Field `lo_lf_cz` writer - "]
pub type LoLfCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_r4` reader - "]
pub type LoLfR4R = crate::FieldReader;
#[doc = "Field `lo_lf_r4` writer - "]
pub type LoLfR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_lf_r4_short` reader - "]
pub type LoLfR4ShortR = crate::BitReader;
#[doc = "Field `lo_lf_r4_short` writer - "]
pub type LoLfR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_slipped_dn` reader - "]
pub type LoSlippedDnR = crate::BitReader;
#[doc = "Field `lo_slipped_dn` writer - "]
pub type LoSlippedDnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_slipped_up` reader - "]
pub type LoSlippedUpR = crate::BitReader;
#[doc = "Field `lo_slipped_up` writer - "]
pub type LoSlippedUpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&self) -> LoLfRzHwR {
        LoLfRzHwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&self) -> LoLfR4HwR {
        LoLfR4HwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&self) -> LoLfCzHwR {
        LoLfCzHwR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&self) -> LoLfRzR {
        LoLfRzR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&self) -> LoLfCzR {
        LoLfCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&self) -> LoLfR4R {
        LoLfR4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&self) -> LoLfR4ShortR {
        LoLfR4ShortR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&self) -> LoSlippedDnR {
        LoSlippedDnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&self) -> LoSlippedUpR {
        LoSlippedUpR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&mut self) -> LoLfRzHwW<'_, LoSpec> {
        LoLfRzHwW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&mut self) -> LoLfR4HwW<'_, LoSpec> {
        LoLfR4HwW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&mut self) -> LoLfCzHwW<'_, LoSpec> {
        LoLfCzHwW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&mut self) -> LoLfRzW<'_, LoSpec> {
        LoLfRzW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&mut self) -> LoLfCzW<'_, LoSpec> {
        LoLfCzW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&mut self) -> LoLfR4W<'_, LoSpec> {
        LoLfR4W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&mut self) -> LoLfR4ShortW<'_, LoSpec> {
        LoLfR4ShortW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&mut self) -> LoSlippedDnW<'_, LoSpec> {
        LoSlippedDnW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&mut self) -> LoSlippedUpW<'_, LoSpec> {
        LoSlippedUpW::new(self, 24)
    }
}
#[doc = "lo.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSpec;
impl crate::RegisterSpec for LoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LoSpec {}
#[doc = "`write(|w| ..)` method takes [`lo::W`](W) writer structure"]
impl crate::Writable for LoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo to value 0"]
impl crate::Resettable for LoSpec {}
