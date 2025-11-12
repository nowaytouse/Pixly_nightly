export function shouldShowNavigationItem(
    visibilityPreferencesKey: string | null,
    isEditing: boolean,
    data: Record<string, boolean> | null,
    itemVisibilityPreferenceKey: string,
): boolean {
    // If there are no visibility preferences,
    // the item should always be shown.
    if (!visibilityPreferencesKey) {
        return true;
    }

    // If the visibility preference of an item
    // is in an editing state, it should be shown.
    if (isEditing) {
        return true;
    }

    // Show the item if the visibility preference is to show it.
    if (data && data[itemVisibilityPreferenceKey]) {
        return true;
    }

    return false;
}
