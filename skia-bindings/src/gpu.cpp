#include "bindings.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkDrawable.h"
#include "include/core/SkSurface.h"
#include "include/core/SkSurfaceCharacterization.h"
#include "include/core/SkImageGenerator.h"
#include "include/core/SkImage.h"

//
// core/SkSurface.h
//

extern "C" SkSurface* C_SkSurface_MakeFromBackendTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTexture(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            sp(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendRenderTarget(
        GrContext* context,
        const GrBackendRenderTarget* backendRenderTarget,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps
        ) {
    return SkSurface::MakeFromBackendRenderTarget(
            context,
            *backendRenderTarget,
            origin,
            colorType,
            sp(colorSpace),
            surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendTextureAsRenderTarget(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTextureAsRenderTarget(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            sp(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget(
    GrContext* context,
    SkBudgeted budgeted,
    const SkImageInfo* imageInfo,
    int sampleCount, GrSurfaceOrigin surfaceOrigin,
    const SkSurfaceProps* surfaceProps,
    bool shouldCreateWithMips) {
    return SkSurface::MakeRenderTarget(
            context,
            budgeted,
            *imageInfo,
            sampleCount,
            surfaceOrigin,
            surfaceProps,
            shouldCreateWithMips).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget2(
        GrContext* context,
        const SkSurfaceCharacterization& characterization,
        SkBudgeted budgeted) {
    return SkSurface::MakeRenderTarget(
            context,
            characterization,
            budgeted).release();
}

extern "C" SkSurface *C_SkSurface_MakeFromBackendTexture2(
        GrContext *context,
        const SkSurfaceCharacterization &characterization,
        const GrBackendTexture *backendTexture) {
    return SkSurface::MakeFromBackendTexture(
            context,
            characterization,
            *backendTexture).release();
}

extern "C" void C_SkSurface_getBackendTexture(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendTexture* backendTexture) {
    *backendTexture = self->getBackendTexture(handleAccess);
}

extern "C" void C_SkSurface_getBackendRenderTarget(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendRenderTarget *backendRenderTarget) {
    *backendRenderTarget = self->getBackendRenderTarget(handleAccess);
}

//
// core/SkSurfaceCharacterization.h
//

extern "C" const SkImageInfo* C_SkSurfaceCharacterization_imageInfo(const SkSurfaceCharacterization* self) {
    return &self->imageInfo();
}

//
// core/SkImageGenerator.h
//

extern "C" bool C_SkImageGenerator_isValid(const SkImageGenerator* self, GrContext* context) {
    return self->isValid(context);
}

//
// gpu/GrBackendSurface.h
//

// GrBackendRenderTarget

extern "C" void C_GrBackendRenderTarget_Construct(GrBackendRenderTarget* uninitialized) {
    new(uninitialized) GrBackendRenderTarget();
}

extern "C" void C_GrBackendRenderTarget_CopyConstruct(GrBackendRenderTarget* uninitialized, const GrBackendRenderTarget* renderTarget) {
    new(uninitialized) GrBackendRenderTarget(*renderTarget);
}

extern "C" void C_GrBackendRenderTarget_destruct(GrBackendRenderTarget* self) {
    self->~GrBackendRenderTarget();
}

// GrBackendTexture

extern "C" void C_GrBackendTexture_Construct(GrBackendTexture* uninitialized) {
    new(uninitialized) GrBackendTexture();
}

extern "C" void C_GrBackendTexture_CopyConstruct(GrBackendTexture* uninitialized, const GrBackendTexture* texture) {
    new(uninitialized) GrBackendTexture(*texture);
}

extern "C" void C_GrBackendTexture_destruct(const GrBackendTexture* self) {
    self->~GrBackendTexture();
}

// GrBackendFormat

extern "C" void C_GrBackendFormat_Construct(GrBackendFormat* uninitialized) {
    new(uninitialized)GrBackendFormat();
}

extern "C" void C_GrBackendFormat_destruct(GrBackendFormat* self) {
    self->~GrBackendFormat();
}

extern "C" bool C_GrBackendFormat_Equals(const GrBackendFormat* lhs, const GrBackendFormat* rhs) {
    return *lhs == *rhs;
}

//
// gpu/GrContext.h
//

extern "C" bool C_GrContext_colorTypeSupportedAsSurface(const GrContext* self, SkColorType colorType) {
    return self->colorTypeSupportedAsSurface(colorType);
}

extern "C" bool C_GrContext_abandoned(GrContext* self) {
    return self->abandoned();
}

extern "C" void C_GrContext_flush(GrContext* self) {
    self->flush();
}

extern "C" size_t C_GrContext_ComputeImageSize(SkImage* image, GrMipMapped mm, bool useNextPow2) {
    return GrContext::ComputeImageSize(sp(image), mm, useNextPow2);
}

extern "C" void C_GrContext_defaultBackendFormat(const GrContext* self, SkColorType ct, GrRenderable renderable, GrBackendFormat* result) {
    *result = self->defaultBackendFormat(ct, renderable);
}

extern "C" void C_GrContext_compressedBackendFormat(const GrContext* self, SkImage::CompressionType compression, GrBackendFormat* result) {
    *result = self->compressedBackendFormat(compression);
}

//
// gpu/GrBackendDrawableInfo.h
//

extern "C" void C_GrBackendDrawableInfo_Construct(GrBackendDrawableInfo* uninitialized) {
    new(uninitialized) GrBackendDrawableInfo();
}

extern "C" void C_GrBackendDrawableInfo_Construct2(GrBackendDrawableInfo* uninitialized, const GrVkDrawableInfo* info) {
    new(uninitialized) GrBackendDrawableInfo(*info);
}

extern "C" void C_GrBackendDrawableInfo_destruct(GrBackendDrawableInfo* self) {
    self->~GrBackendDrawableInfo();
}

extern "C" bool C_GrBackendDrawableInfo_isValid(const GrBackendDrawableInfo* self) {
    return self->isValid();
}

extern "C" GrBackendApi C_GrBackendDrawableInfo_backend(const GrBackendDrawableInfo* self) {
    return self->backend();
}

//
// core/SkCanvas.h
//

extern "C" GrContext* C_SkCanvas_getGrContext(SkCanvas* self) {
    return self->getGrContext();
}

//
// core/SkDrawable.h
//

extern "C" SkDrawable::GpuDrawHandler *C_SkDrawable_snapGpuDrawHandler(SkDrawable *self, GrBackendApi backendApi,
                                                                       const SkMatrix *matrix,
                                                                       const SkIRect *clipBounds,
                                                                       const SkImageInfo *bufferInfo) {
    return self->snapGpuDrawHandler(backendApi, *matrix, *clipBounds, *bufferInfo).release();
}

extern "C" void C_SkDrawable_GpuDrawHandler_delete(SkDrawable::GpuDrawHandler *self) {
    delete self;
}

extern "C" void C_SkDrawable_GpuDrawHandler_draw(SkDrawable::GpuDrawHandler *self, const GrBackendDrawableInfo *info) {
    self->draw(*info);
}

//
// core/SkImage.h
//


extern "C" SkImage *C_SkImage_MakeTextureFromCompressed(GrContext *context, SkData *data, int width, int height,
                                                SkImage::CompressionType type, GrMipMapped mipMapped,
                                                GrProtected prot) {
    return SkImage::MakeTextureFromCompressed(context, sp(data), width, height, type, mipMapped, prot).release();
}

extern "C" void C_SkImage_getBackendTexture(
        const SkImage* self,
        bool flushPendingGrContextIO,
        GrSurfaceOrigin* origin,
        GrBackendTexture* result)
{
    *result = self->getBackendTexture(flushPendingGrContextIO, origin);
}

extern "C" SkImage *C_SkImage_MakeFromCompressed(GrContext *context, SkData *encoded, int width, int height,
                                                 SkImage::CompressionType type, GrMipMapped mipMapped, GrProtected 
                                                 prot) {
    return SkImage::MakeFromCompressed(context, sp(encoded), width, height, type, mipMapped, prot).release();
}

extern "C" SkImage* C_SkImage_DecodeToTexture(GrContext* ctx, const void* encoded, size_t length, const SkIRect* subset) {
    return SkImage::DecodeToTexture(ctx, encoded, length, subset).release();
}

extern "C" SkImage* C_SkImage_MakeFromTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromTexture(context, *backendTexture, origin, colorType, alphaType, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromTextureWithReleaseProc(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace,
        SkImage::TextureReleaseProc releaseProc,
        SkImage::ReleaseContext releaseContext) {
    return SkImage::MakeFromTexture(context, *backendTexture, origin, colorType, alphaType, sp(colorSpace), releaseProc, releaseContext).release();
}

extern "C" SkImage* C_SkImage_MakeCrossContextFromPixmap(
        GrContext* context,
        const SkPixmap* pixmap,
        bool buildMips,
        bool limitToMaxTextureSize) {
    return SkImage::MakeCrossContextFromPixmap(context, *pixmap, buildMips, limitToMaxTextureSize).release();
}

extern "C" SkImage* C_SkImage_MakeFromAdoptedTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromAdoptedTexture(context, *backendTexture, origin, colorType, alphaType, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopy(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture& backendTexture,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopyWithExternalBackend(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, backendTexture,
            sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATextures(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATextures(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopy(
            context, yuvColorSpace, nv12Textures, imageOrigin,
            sp(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture* backendTexture,
        SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopyWithExternalBackend(
            context,
            yuvColorSpace, nv12Textures, imageOrigin, *backendTexture,
            sp(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_makeTextureImage(
        const SkImage* self,
        GrContext* context,
        GrMipMapped mipMapped,
        SkBudgeted budgeted) {
    return self->makeTextureImage(context, mipMapped, budgeted).release();
}
