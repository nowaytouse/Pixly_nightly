# Module Consolidation Plan

**Date**: 2025-11-20  
**Goal**: Reduce code duplication by merging similar modules

## Identified Duplications

### 1. Quality Modules (HIGH PRIORITY)

**Problem**: Multiple modules define similar quality-related structures

**Modules**:
- `quality_checker.rs` - SSIM checker, QualityGrade enum
- `quality_metrics.rs` - QualityMetrics struct, QualityAssessor
- `quality_analyzer.rs` - QualityMetrics struct (duplicate!), ImageAnalyzer
- `quality_reporter.rs` - CompressionStats, QualityReport
- `quality_presets.rs` - QualityPreset enum, PresetConfig
- `visual_quality_scorer.rs` - Visual quality scoring

**Duplications**:
- `QualityGrade` enum defined in both quality_checker.rs and quality_metrics.rs
- `QualityMetrics` struct defined in both quality_metrics.rs and quality_analyzer.rs

**Consolidation Plan**:
```
quality/
├── mod.rs (re-exports)
├── types.rs (shared types: QualityGrade, QualityMetrics)
├── checker.rs (SSIM/PSNR checking - from quality_checker.rs)
├── analyzer.rs (quality analysis - from quality_analyzer.rs)
├── reporter.rs (reporting - from quality_reporter.rs)
├── presets.rs (presets - from quality_presets.rs)
└── visual_scorer.rs (visual scoring - from visual_quality_scorer.rs)
```

### 2. Format Modules (MEDIUM PRIORITY)

**Problem**: Multiple format-related modules with overlapping functionality

**Modules**:
- `format_selector.rs` - FormatRecommendation, FormatSelector
- `format_recommender.rs` - FormatRecommendation (duplicate!), AIFormatRecommender
- `format_corrector.rs` - Format correction
- `format_knowledge.rs` - Format knowledge base
- `format_params.rs` - Format-specific parameters

**Duplications**:
- `FormatRecommendation` struct defined in both format_selector.rs and format_recommender.rs

**Consolidation Plan**:
```
format/
├── mod.rs (re-exports)
├── types.rs (shared types: FormatRecommendation)
├── selector.rs (format selection - from format_selector.rs)
├── recommender.rs (AI recommendation - from format_recommender.rs)
├── corrector.rs (format correction - from format_corrector.rs)
├── knowledge.rs (knowledge base - from format_knowledge.rs)
└── params.rs (parameters - from format_params.rs)
```

### 3. Optimizer Modules (LOW PRIORITY)

**Modules**:
- `bayesian_optimizer.rs` - Bayesian optimization
- `same_format_optimizer.rs` - Same format optimization
- `gif_optimizer_advanced.rs` - GIF-specific optimization

**Status**: These are specialized, keep separate for now

## Implementation Steps

### Phase 1: Quality Module Consolidation

1. ✅ Create `src/quality/` directory
2. ✅ Create `src/quality/types.rs` with unified types
3. ✅ Move and refactor individual modules
4. ✅ Update `src/lib.rs` imports
5. ✅ Update all references in codebase
6. ✅ Run tests to verify
7. ✅ Commit changes

### Phase 2: Format Module Consolidation

1. ⏳ Create `src/format/` directory
2. ⏳ Create `src/format/types.rs` with unified types
3. ⏳ Move and refactor individual modules
4. ⏳ Update imports
5. ⏳ Run tests
6. ⏳ Commit changes

## Expected Benefits

- **Code Reduction**: ~15-20% reduction in quality/format related code
- **Maintainability**: Single source of truth for shared types
- **Clarity**: Clear module organization
- **Performance**: Reduced compilation time

## Risks

- **Breaking Changes**: Need to update all imports
- **Test Failures**: Need comprehensive testing
- **Merge Conflicts**: Careful git management required

## Rollback Plan

If consolidation causes issues:
1. Git revert to pre-consolidation commit
2. Analyze failures
3. Fix issues incrementally
4. Re-attempt consolidation

---

**Ref**: PROJECT_QUALITY_MANIFESTO.md - Code quality and organization
