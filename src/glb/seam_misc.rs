#[doc = "Register `seam_misc` reader"]
pub type R = crate::R<SeamMiscSpec>;
#[doc = "Register `seam_misc` writer"]
pub type W = crate::W<SeamMiscSpec>;
#[doc = "Field `em_sel` reader - "]
pub type EmSelR = crate::FieldReader;
#[doc = "Field `em_sel` writer - "]
pub type EmSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&self) -> EmSelR {
        EmSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&mut self) -> EmSelW<'_, SeamMiscSpec> {
        EmSelW::new(self, 0)
    }
}
#[doc = "seam_misc.\n\nYou can [`read`](crate::Reg::read) this register and get [`seam_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seam_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeamMiscSpec;
impl crate::RegisterSpec for SeamMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seam_misc::R`](R) reader structure"]
impl crate::Readable for SeamMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`seam_misc::W`](W) writer structure"]
impl crate::Writable for SeamMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets seam_misc to value 0x03"]
impl crate::Resettable for SeamMiscSpec {
    const RESET_VALUE: u32 = 0x03;
}
