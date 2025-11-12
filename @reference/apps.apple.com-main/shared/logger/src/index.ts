import { getContext } from 'svelte';
import type { Logger, LoggerFactory } from './types';

export * from './composite';
export * from './console';
export * from './deferred';
export * from './recording';
export * from './sampled';
export * from './types';
export * from './void';

const CONTEXT_NAME = 'loggerFactory';

export function setContext(
    context: Map<string, unknown>,
    factory: LoggerFactory,
): void {
    context.set(CONTEXT_NAME, factory);
}

export function loggerFor(subject: string): Logger {
    const factory = getContext(CONTEXT_NAME) as LoggerFactory | undefined;

    if (!factory) {
        throw new Error(
            'loggerFor called before setContext or outside of svelte component init',
        );
    }

    return factory.loggerFor(subject);
}
