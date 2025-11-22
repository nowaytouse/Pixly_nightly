const ENABLED_FEATURES = new Set([
    // Make the `ProductPageIntentController` return a `ShelfBasedProductPage` instance
    'shelves_2_0_product',
    // Enable shelf-based "Top Charts" features
    // 'shelves_2_0_top_charts',
    // Make the `RibbonBarShelf` contain an array of `RibbonBarItem`s
    'shelves_2_0_generic',
    // Enable AX Metadata
    'product_accessibility_support_2025A',
]);

export class WebFeatureFlags implements FeatureFlags {
    isEnabled(feature: string): boolean {
        return ENABLED_FEATURES.has(feature);
    }

    isGSEUIEnabled(_feature: string): boolean {
        return false;
    }
}
