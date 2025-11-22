import type { WithPlatform } from 'node_modules/@jet-app/app-store/src/api/models/preview-platform';

export function isWithPlatform(x: unknown): x is WithPlatform {
    return typeof x === 'object' && x !== null && 'platform' in x;
}
