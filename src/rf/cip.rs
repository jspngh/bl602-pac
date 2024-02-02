#[doc = "Register `cip` reader"]
pub type R = crate::R<CipSpec>;
#[doc = "Register `cip` writer"]
pub type W = crate::W<CipSpec>;
#[doc = "Field `vg11_sel` reader - "]
pub type Vg11SelR = crate::FieldReader;
#[doc = "Field `vg11_sel` writer - "]
pub type Vg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vg13_sel` reader - "]
pub type Vg13SelR = crate::FieldReader;
#[doc = "Field `vg13_sel` writer - "]
pub type Vg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> Vg11SelR {
        Vg11SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&self) -> Vg13SelR {
        Vg13SelR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&mut self) -> Vg11SelW<'_, CipSpec> {
        Vg11SelW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&mut self) -> Vg13SelW<'_, CipSpec> {
        Vg13SelW::new(self, 2)
    }
}
#[doc = "RX normal bias mode registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CipSpec;
impl crate::RegisterSpec for CipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cip::R`](R) reader structure"]
impl crate::Readable for CipSpec {}
#[doc = "`write(|w| ..)` method takes [`cip::W`](W) writer structure"]
impl crate::Writable for CipSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cip to value 0"]
impl crate::Resettable for CipSpec {}
