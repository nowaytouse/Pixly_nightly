/* eslint-disable import/prefer-default-export */
// eslint-disable-next-line no-restricted-imports
import { tick as svelteTick, onDestroy } from 'svelte';

// Unfortantely for TS to recognize that this can be awaited
// we need to leave `Promise<void | never>` otherwise TS hints
// will suggest removing the await.
// See @remarks for reason to disable `then`
type TickType = () => Omit<Promise<string>, 'then'> | Promise<void | never>;

type SafeTickCallback = (tick: TickType) => Promise<void | never>;

class DestroyedError extends Error {
    constructor() {
        super('component was destroyed before tick resolved.');
        this.name = 'DestroyedError';
    }
}

/**
 * Provides a safer way to use svelte's tick helper.
 *
 * This prevents code that relies on tick() from running
 * if the component is destroyed while the tick resolution
 * is inflight.
 *
 * @remarks
 * To avoid floating promises (promises with no return statements)
 * it is safer to use the `async/await` syntax.
 *
 * If this is used with the `.then()` syntax without the promise
 * being returned the DestroyedError will bubble up to sentry.
 *
 * @example
 * ```ts
 * const safeTick = makeSafeTick();
 * onMount(async() => {
 *     await safeTick(async (tick) => {
 *         // Use tick normally
 *         await tick();
 *         // ...
 *     });
 * });
 * ```
 */
export const makeSafeTick = (): ((
    callback: SafeTickCallback,
) => Promise<void | never>) => {
    let destroyed = false;
    onDestroy(() => {
        destroyed = true;
    });

    return async (callback) => {
        try {
            await callback(async () => {
                await svelteTick();
                if (destroyed) throw new DestroyedError();
            });
        } catch (e) {
            if (!(e instanceof DestroyedError)) throw e;
        }
    };
};
