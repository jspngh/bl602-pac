#[doc = "Register `gpdac_data` reader"]
pub type R = crate::R<GpdacDataSpec>;
#[doc = "Register `gpdac_data` writer"]
pub type W = crate::W<GpdacDataSpec>;
#[doc = "Field `gpdac_b_data` reader - "]
pub type GpdacBDataR = crate::FieldReader<u16>;
#[doc = "Field `gpdac_b_data` writer - "]
pub type GpdacBDataW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `gpdac_a_data` reader - "]
pub type GpdacADataR = crate::FieldReader<u16>;
#[doc = "Field `gpdac_a_data` writer - "]
pub type GpdacADataW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&self) -> GpdacBDataR {
        GpdacBDataR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&self) -> GpdacADataR {
        GpdacADataR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&mut self) -> GpdacBDataW<'_, GpdacDataSpec> {
        GpdacBDataW::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&mut self) -> GpdacADataW<'_, GpdacDataSpec> {
        GpdacADataW::new(self, 16)
    }
}
#[doc = "gpdac_data.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacDataSpec;
impl crate::RegisterSpec for GpdacDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_data::R`](R) reader structure"]
impl crate::Readable for GpdacDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_data::W`](W) writer structure"]
impl crate::Writable for GpdacDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_data to value 0"]
impl crate::Resettable for GpdacDataSpec {}
