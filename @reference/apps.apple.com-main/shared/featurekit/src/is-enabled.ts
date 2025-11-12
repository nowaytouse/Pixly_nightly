export const isEnabled = (key: string): boolean => {
    if (typeof window !== 'undefined') {
        return window._featureKit?.isEnabled(key) ?? false;
    }

    return false;
};
