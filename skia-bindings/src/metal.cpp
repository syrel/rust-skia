#include "bindings.h"

#include "include/core/SkSurface.h"
#include "include/gpu/GrContext.h"

//
// core/SkSurface.h
//

extern "C" SkSurface *C_SkSurface_MakeFromCAMetalLayer(GrContext *context,
                                                       GrMTLHandle layer,
                                                       GrSurfaceOrigin origin,
                                                       int sampleCnt,
                                                       SkColorType colorType,
                                                       SkColorSpace *colorSpace,
                                                       const SkSurfaceProps *surfaceProps,
                                                       GrMTLHandle *drawable) {
    return SkSurface::MakeFromCAMetalLayer(context, layer, origin, sampleCnt, colorType, sp(colorSpace), surfaceProps,
                                           drawable).release();
}

extern "C" SkSurface *C_SkSurface_MakeFromMTKView(GrContext *context,
                                                  GrMTLHandle mtkView,
                                                  GrSurfaceOrigin origin,
                                                  int sampleCnt,
                                                  SkColorType colorType,
                                                  SkColorSpace *colorSpace,
                                                  const SkSurfaceProps *surfaceProps) {
    return SkSurface::MakeFromMTKView(context, mtkView, origin, sampleCnt, colorType, sp(colorSpace), surfaceProps
    ).release();
}

//
// gpu/GrContext.h
//

extern "C" GrContext* C_GrContext_MakeMetal(void* device, void* queue) {
    return GrContext::MakeMetal(device, queue).release();
}

//
// gpu/mtl/GrMtlTypes.h
//

extern "C" void C_GrMtlTextureInfo_Construct(GrMtlTextureInfo* unintialized, const void* texture) {
    new (unintialized) GrMtlTextureInfo();
    unintialized->fTexture = sk_cf_obj<const void*>(texture);
}

extern "C" void C_GrMtlTextureInfo_Destruct(GrMtlTextureInfo* self) {
    self->~GrMtlTextureInfo();
}

extern "C" bool C_GrMtlTextureInfo_Equals(const GrMtlTextureInfo* lhs, const GrMtlTextureInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/GrBackendSurface.h
//

extern "C" void C_GrBackendFormat_ConstructMtl(GrBackendFormat* uninitialized, GrMTLPixelFormat format) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeMtl(format));
}


extern "C" void C_GrBackendTexture_ConstructMtl(GrBackendTexture* uninitialized, int width, int height, GrMipMapped mipMapped, const GrMtlTextureInfo* mtlInfo) {
    new(uninitialized)GrBackendTexture(width, height, mipMapped, *mtlInfo);
}

extern "C" void C_GrBackendRenderTarget_ConstructMtl(GrBackendRenderTarget* uninitialized, int width, int height, int sampleCnt, const GrMtlTextureInfo* mtlInfo) {
    new(uninitialized)GrBackendRenderTarget(width, height, sampleCnt, *mtlInfo);
}
