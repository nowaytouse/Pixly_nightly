# Missing Parameters Audit - Complete Implementation Required

## 🔍 Audit Date: 2025-11-17

This document lists **ALL** parameters displayed in the web interface that need kernel implementation.

---

## 📦 JXL (JPEG XL) Parameters

### ✅ Already Implemented
- [x] Quality (0-100)
- [x] Effort (1-9)
- [x] Lossless
- [x] Modular
- [x] Progressive
- [x] Responsive
- [x] Gaborish
- [x] Photon Noise
- [x] Decoding Speed

### ❌ MISSING - Need Implementation
- [ ] **Distance** (0-15, float) - `jxlDistance`
  - UI: Slider, 0-15, step 0.1, default 1.0
  - cjxl parameter: `--distance`
  - Description: Psychovisual distance (0=lossless, higher=more compression)
  
- [ ] **Bit Depth** (8/10/12/16) - `jxlBitDepth`
  - UI: Select dropdown
  - cjxl parameter: `--bits_per_sample`
  - Description: Color bit depth
  
- [ ] **Color Space** (sRGB/P3/Adobe/ProPhoto) - `jxlColorSpace`
  - UI: Select dropdown
  - cjxl parameter: `--color_space`
  - Description: Color space selection
  
- [ ] **Patches** (0-4) - `jxlPatches`
  - UI: Slider, 0-4, default 1
  - cjxl parameter: `--patches`
  - Description: Edge enhancement level

---

## 🎬 AVIF Parameters

### ✅ Already Implemented
- [x] Quality (basic)

### ❌ MISSING - Need Implementation
- [ ] **Speed** (0-10) - `avifSpeed`
  - UI: Slider, 0-10, default 6
  - avifenc parameter: `--speed`
  - Description: Encoding speed vs quality trade-off
  
- [ ] **Min Quantizer** (0-63) - `avifMinQuantizer`
  - UI: Slider, 0-63, default 0
  - avifenc parameter: `--min`
  - Description: Minimum quantizer (max quality limit)
  
- [ ] **Max Quantizer** (0-63) - `avifMaxQuantizer`
  - UI: Slider, 0-63, default 63
  - avifenc parameter: `--max`
  - Description: Maximum quantizer (min quality limit)
  
- [ ] **Chroma Subsampling** (420/422/444) - `avifChromaSubsampling`
  - UI: Select dropdown, default 420
  - avifenc parameter: `--yuv`
  - Description: Color subsampling mode
  
- [ ] **Bit Depth** (8/10/12) - `avifBitDepth`
  - UI: Select dropdown, default 8
  - avifenc parameter: `--depth`
  - Description: Color bit depth
  
- [ ] **Tiles** (rows × cols) - `avifTilesRows`, `avifTilesCols`
  - UI: Two sliders, 1-8 each, default 1×1
  - avifenc parameter: `--tilerowslog2`, `--tilecolslog2`
  - Description: Parallel encoding tiles
  
- [ ] **Premultiplied Alpha** (boolean) - `avifPremultiply`
  - UI: Checkbox, default false
  - avifenc parameter: `--premultiply`
  - Description: Premultiply alpha channel

---

## 🍎 HEIC/HEIF Parameters

### ✅ Already Implemented
- [x] Basic quality

### ❌ MISSING - Need Implementation
- [ ] **Quality** (1-100) - `heicQuality`
  - UI: Slider, 1-100, default 85
  - x265/libheif parameter: `-q` or `--quality`
  - Description: Compression quality
  
- [ ] **Encoder** (x265/libheif) - `heicEncoder`
  - UI: Select dropdown, default x265
  - Description: Encoder selection
  
- [ ] **Chroma Subsampling** (420/422/444) - `heicChromaSubsampling`
  - UI: Select dropdown, default 444
  - x265 parameter: `--output-depth` + chroma mode
  - Description: Color subsampling mode
  
- [ ] **Lossless** (boolean) - `heicLossless`
  - UI: Checkbox, default false
  - x265 parameter: `--lossless`
  - Description: Lossless encoding
  
- [ ] **Embed Thumbnail** (boolean) - `heicThumbEmbed`
  - UI: Checkbox, default true
  - libheif parameter: `--thumb`
  - Description: Embed thumbnail in file

---

## 📊 Summary

| Format | Total Parameters | Implemented | Missing | Completion |
|--------|-----------------|-------------|---------|------------|
| JXL    | 13              | 9           | 4       | 69%        |
| AVIF   | 8               | 1           | 7       | 13%        |
| HEIC   | 5               | 1           | 4       | 20%        |
| **TOTAL** | **26**      | **11**      | **15**  | **42%**    |

---

## 🎯 Implementation Priority

### Phase 1: Critical Parameters (High Impact)
1. JXL Distance - Most important quality control
2. AVIF Speed - Encoding performance
3. AVIF Chroma Subsampling - Quality vs size
4. HEIC Encoder selection - Core functionality

### Phase 2: Quality Parameters
5. JXL Bit Depth
6. AVIF Bit Depth
7. AVIF Min/Max Quantizer
8. HEIC Chroma Subsampling

### Phase 3: Advanced Features
9. JXL Color Space
10. JXL Patches
11. AVIF Tiles
12. AVIF Premultiplied Alpha
13. HEIC Lossless
14. HEIC Embed Thumbnail

---

## 🔧 Implementation Plan

### Step 1: Update Parameter Structures
- Extend `JXLParams` struct
- Create `AVIFParams` struct
- Create `HEICParams` struct

### Step 2: Update Command Builders
- Add parameters to cjxl command
- Add parameters to avifenc command
- Add parameters to x265/libheif command

### Step 3: Update CLI Interface
- Add parameter parsing
- Add validation
- Add defaults

### Step 4: Testing
- Unit tests for each parameter
- Integration tests
- Real file conversion tests

---

## ⚠️ Current Status

**CRITICAL**: 58% of displayed parameters are non-functional!

Users see these options in the UI but clicking them has **NO EFFECT** on the output.

This violates the PROJECT_QUALITY_MANIFESTO.md principle: **"Oppose ornamental code"**

---

## ✅ Action Required

**IMPLEMENT ALL 15 MISSING PARAMETERS IMMEDIATELY**

No more fake UI! Every slider, checkbox, and dropdown must have real kernel support!

---

**Last Updated**: 2025-11-17
**Status**: 🔴 CRITICAL - 15 parameters missing implementation
