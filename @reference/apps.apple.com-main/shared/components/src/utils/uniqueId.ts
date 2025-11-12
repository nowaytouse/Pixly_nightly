import { getContext } from 'svelte';

export const UNIQUE_ID_CONTEXT_NAME = 'amp-web-unique-id';

interface UniqueContext {
    nextId: number;
}

// TODO: rdar://84029606 (Extract logger into shared util)
interface Logger {
    warn(...args: any[]): string;
}
interface LoggerFactory {
    loggerFor(name: string): Logger;
}

export function initializeUniqueIdContext(
    context: Map<string, unknown>,
    loggerFactory: LoggerFactory,
): void {
    const logger = loggerFactory.loggerFor('uniqueIdContext');

    if (context.has(UNIQUE_ID_CONTEXT_NAME)) {
        logger.warn(
            `${UNIQUE_ID_CONTEXT_NAME} context has already been created. Cannot be created more than once`,
        );
    } else {
        const INITAL_STATE: UniqueContext = { nextId: 0 };
        context.set(UNIQUE_ID_CONTEXT_NAME, INITAL_STATE);
    }
}

/**
 * Creates a unique Id string based on string provided
 *
 * @returns unique id string
 */
export type UniqueIdGenerator = () => string;

// Custom elements most likely will not be used in an environment has that initialized the Svelte
// context. Components that are later wrapped by a custom element should use this function so that
// they can generate unique ids automatically when used inside a Svelte app, but not throw an error
// when used in other contexts.
//
export function maybeGetUniqueIdGenerator(): UniqueIdGenerator | undefined {
    const UNIQUE_ID_PREFIX = 'uid-';
    const state: UniqueContext = getContext(UNIQUE_ID_CONTEXT_NAME);
    const isNextIdANumber = typeof state?.nextId === 'number';

    if (!isNextIdANumber) {
        return;
    }

    return () => {
        const id = `${UNIQUE_ID_PREFIX}${state.nextId}`;
        state.nextId += 1;
        return id;
    };
}

export function getUniqueIdGenerator(): UniqueIdGenerator {
    const uniqueIdGenerator = maybeGetUniqueIdGenerator();

    if (!uniqueIdGenerator) {
        throw new Error(
            `${UNIQUE_ID_CONTEXT_NAME} context has not been initialized. Initialize at application bootstrap.`,
        );
    }

    return uniqueIdGenerator;
}
