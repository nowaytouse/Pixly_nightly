import type { Intent } from '@jet/environment/dispatching';

export interface PrefetchedIntent {
    intent: Intent<unknown>;
    data: unknown;
}

export function isPrefetchedIntents(v: unknown): v is PrefetchedIntent[] {
    return Array.isArray(v) && v.every(isPrefetchedIntent);
}

function isPrefetchedIntent(v: unknown): v is PrefetchedIntent {
    return hasIntentAndData(v) && isIntent(v.intent);
}

function hasIntentAndData(v: unknown): v is HasIntentAndData {
    return v !== null && typeof v === 'object' && 'intent' in v && 'data' in v;
}

interface HasIntentAndData {
    intent: unknown;
    data: unknown;
}

function isIntent(v: unknown): v is Intent<unknown> {
    return v !== null && typeof v === 'object' && '$kind' in v;
}
