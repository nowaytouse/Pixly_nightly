# Complete Implementation Guide - All Missing Parameters

## 🎯 Goal
Implement ALL 15 missing parameters to ensure every UI element has real kernel support.

## 📦 JXL Parameters Implementation

### 1. Distance Parameter
```rust
// In JXLParams struct
pub distance: f32,  // 0.0-15.0

// In cjxl command builder (after line 218)
if params.distance > 0.0 {
    cmd.args(&["--distance", &params.distance.to_string()]);
}

// In Default impl
distance: 1.0,
```

### 2. Bit Depth Parameter
```rust
// In JXLParams struct
pub bit_depth: u8,  // 8, 10, 12, or 16

// In cjxl command builder
cmd.args(&["--bits_per_sample", &params.bit_depth.to_string()]);

// In Default impl
bit_depth: 8,
```

### 3. Color Space Parameter
```rust
// In JXLParams struct
pub color_space: String,  // "sRGB", "Display P3", etc.

// In cjxl command builder
if !params.color_space.is_empty() && params.color_space != "sRGB" {
    cmd.args(&["--color_space", &params.color_space]);
}

// In Default impl
color_space: "sRGB".to_string(),
```

### 4. Patches Parameter
```rust
// In JXLParams struct
pub patches: u8,  // 0-4

// In cjxl command builder
if params.patches > 0 {
    cmd.args(&["--patches", &params.patches.to_string()]);
}

// In Default impl
patches: 1,
```

---

## 🎬 AVIF Parameters Implementation

### Create AVIFParams Struct
```rust
#[derive(Debug, Clone)]
pub struct AVIFParams {
    pub quality: u8,           // 0-100
    pub speed: u8,             // 0-10
    pub min_quantizer: u8,     // 0-63
    pub max_quantizer: u8,     // 0-63
    pub chroma_subsampling: String, // "420", "422", "444"
    pub bit_depth: u8,         // 8, 10, 12
    pub tiles_rows: u8,        // 1-8
    pub tiles_cols: u8,        // 1-8
    pub premultiply_alpha: bool,
}

impl Default for AVIFParams {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 6,
            min_quantizer: 0,
            max_quantizer: 63,
            chroma_subsampling: "420".to_string(),
            bit_depth: 8,
            tiles_rows: 1,
            tiles_cols: 1,
            premultiply_alpha: false,
        }
    }
}
```

### AVIF Command Builder
```rust
pub fn convert_to_avif<P: AsRef<Path>>(
    &self,
    input: P,
    output: P,
    params: &AVIFParams,
) -> Result<ConversionResult> {
    let mut cmd = Command::new("avifenc");
    
    cmd.arg(input.as_ref());
    cmd.arg(output.as_ref());
    
    // Quality
    cmd.args(&["--min", &params.min_quantizer.to_string()]);
    cmd.args(&["--max", &params.max_quantizer.to_string()]);
    
    // Speed
    cmd.args(&["--speed", &params.speed.to_string()]);
    
    // Chroma subsampling
    cmd.args(&["--yuv", &params.chroma_subsampling]);
    
    // Bit depth
    cmd.args(&["--depth", &params.bit_depth.to_string()]);
    
    // Tiles
    if params.tiles_rows > 1 || params.tiles_cols > 1 {
        let rows_log2 = (params.tiles_rows as f32).log2() as u8;
        let cols_log2 = (params.tiles_cols as f32).log2() as u8;
        cmd.args(&["--tilerowslog2", &rows_log2.to_string()]);
        cmd.args(&["--tilecolslog2", &cols_log2.to_string()]);
    }
    
    // Premultiplied alpha
    if params.premultiply_alpha {
        cmd.arg("--premultiply");
    }
    
    // Execute...
}
```

---

## 🍎 HEIC Parameters Implementation

### Create HEICParams Struct
```rust
#[derive(Debug, Clone)]
pub struct HEICParams {
    pub quality: u8,           // 1-100
    pub encoder: String,       // "x265" or "libheif"
    pub chroma_subsampling: String, // "420", "422", "444"
    pub lossless: bool,
    pub embed_thumbnail: bool,
}

impl Default for HEICParams {
    fn default() -> Self {
        Self {
            quality: 85,
            encoder: "x265".to_string(),
            chroma_subsampling: "444".to_string(),
            lossless: false,
            embed_thumbnail: true,
        }
    }
}
```

### HEIC Command Builder
```rust
pub fn convert_to_heic<P: AsRef<Path>>(
    &self,
    input: P,
    output: P,
    params: &HEICParams,
) -> Result<ConversionResult> {
    let mut cmd = if params.encoder == "x265" {
        let mut c = Command::new("ffmpeg");
        c.args(&["-i", input.as_ref().to_str().unwrap()]);
        c.args(&["-c:v", "libx265"]);
        c.args(&["-crf", &((100 - params.quality) / 2).to_string()]);
        
        if params.lossless {
            c.arg("-x265-params").arg("lossless=1");
        }
        
        // Chroma subsampling
        let pix_fmt = match params.chroma_subsampling.as_str() {
            "420" => "yuv420p",
            "422" => "yuv422p",
            "444" => "yuv444p",
            _ => "yuv420p",
        };
        c.args(&["-pix_fmt", pix_fmt]);
        
        c.arg(output.as_ref());
        c
    } else {
        // libheif encoder
        let mut c = Command::new("heif-enc");
        c.arg(input.as_ref());
        c.args(&["-q", &params.quality.to_string()]);
        
        if params.lossless {
            c.arg("--lossless");
        }
        
        if params.embed_thumbnail {
            c.arg("--thumb");
        }
        
        c.arg(output.as_ref());
        c
    };
    
    // Execute...
}
```

---

## 🔧 CLI Integration

### Update CLI Parameter Parsing

Add to `src/cli_convert.rs` or relevant CLI module:

```rust
// JXL parameters
if format == "jxl" {
    if let Some(distance) = matches.value_of("jxl-distance") {
        jxl_params.distance = distance.parse()?;
    }
    if let Some(bit_depth) = matches.value_of("jxl-bit-depth") {
        jxl_params.bit_depth = bit_depth.parse()?;
    }
    if let Some(color_space) = matches.value_of("jxl-color-space") {
        jxl_params.color_space = color_space.to_string();
    }
    if let Some(patches) = matches.value_of("jxl-patches") {
        jxl_params.patches = patches.parse()?;
    }
}

// AVIF parameters
if format == "avif" {
    if let Some(speed) = matches.value_of("avif-speed") {
        avif_params.speed = speed.parse()?;
    }
    // ... etc
}

// HEIC parameters
if format == "heic" {
    if let Some(encoder) = matches.value_of("heic-encoder") {
        heic_params.encoder = encoder.to_string();
    }
    // ... etc
}
```

---

## ✅ Testing Checklist

### For Each Parameter:
- [ ] Add to struct
- [ ] Add to Default impl
- [ ] Add to command builder
- [ ] Add CLI argument parsing
- [ ] Write unit test
- [ ] Test with real file
- [ ] Verify output quality
- [ ] Update documentation

### Test Commands:
```bash
# JXL with all parameters
cargo run -- convert input.png output.jxl \
  --jxl-distance 1.5 \
  --jxl-bit-depth 10 \
  --jxl-color-space "Display P3" \
  --jxl-patches 2

# AVIF with all parameters
cargo run -- convert input.png output.avif \
  --avif-speed 6 \
  --avif-min-quantizer 10 \
  --avif-max-quantizer 50 \
  --avif-chroma 444 \
  --avif-bit-depth 10

# HEIC with all parameters
cargo run -- convert input.png output.heic \
  --heic-encoder x265 \
  --heic-quality 90 \
  --heic-chroma 444 \
  --heic-lossless
```

---

## 📊 Progress Tracking

| Parameter | Struct | Default | Command | CLI | Test | Status |
|-----------|--------|---------|---------|-----|------|--------|
| JXL Distance | ✅ | ⏳ | ⏳ | ⏳ | ⏳ | In Progress |
| JXL Bit Depth | ✅ | ⏳ | ⏳ | ⏳ | ⏳ | In Progress |
| JXL Color Space | ✅ | ⏳ | ⏳ | ⏳ | ⏳ | In Progress |
| JXL Patches | ✅ | ⏳ | ⏳ | ⏳ | ⏳ | In Progress |
| AVIF Speed | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ | Not Started |
| ... | ... | ... | ... | ... | ... | ... |

---

## 🎯 Next Steps

1. **Immediate**: Complete JXL parameters (in progress)
2. **Phase 2**: Implement AVIF parameters
3. **Phase 3**: Implement HEIC parameters
4. **Phase 4**: Full integration testing
5. **Phase 5**: Update web interface to pass parameters

---

**Status**: 🟡 IN PROGRESS - JXL parameters being implemented
**Target**: 100% parameter implementation
**Current**: 42% → Target: 100%
