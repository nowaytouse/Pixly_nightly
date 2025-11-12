import type { LoggerFactory, Logger } from './types';

export class CompositeLoggerFactory implements LoggerFactory {
    private readonly factories: LoggerFactory[];

    constructor(factories: LoggerFactory[]) {
        this.factories = factories;
    }

    loggerFor(name: string): Logger {
        return new CompositeLogger(
            this.factories.map((factory) => factory.loggerFor(name)),
        );
    }
}

export class CompositeLogger implements Logger {
    private readonly loggers: Logger[];

    constructor(loggers: Logger[]) {
        this.loggers = loggers;
    }

    /**
     * Log a debug level message.
     * Appropriate for verbose logging that explains steps/details of the inner state of
     * a code unit.
     *
     * Example uses include in a size-constrain datastructure, logging when the size
     * exceeds the threshold and elements are removed, or in a virtual scrolling
     * component logging when a scroll event causes a new page of elements to be loaded.
     *
     * @param args Arguments to log (same as console.debug)
     * @return empty string (for use in brackets {} in svelte components)
     */
    debug(...args: unknown[]): string {
        return this.callAll('debug', args);
    }

    /**
     * Log an info level message.
     * Appropriate for informational messages that may be relevant to consumers of a code
     * unit.
     *
     * Example uses include a router logging when transitions occur or a button logging
     * clicks.
     *
     * @param args Arguments to log (same as console.info)
     * @return empty string (for use in brackets {} in svelte components)
     */
    info(...args: unknown[]): string {
        return this.callAll('info', args);
    }

    /**
     * Log a warn level message.
     * Appropriate for situations where state has been (or likely will be) corrupted or
     * invariants have been broken.
     *
     * Example uses include a data structure warning when it is used before being fully
     * initialized.
     *
     * @param args Arguments to log (same as console.warn)
     * @return empty string (for use in brackets {} in svelte components)
     */
    warn(...args: unknown[]): string {
        return this.callAll('warn', args);
    }

    /**
     * Log an error message.
     * Appropriate for thrown errors or situations where the apps breaks or has to
     * engage in fallback behavior to avoid a more catastrophic failure.
     *
     * @param args Arguments to log (same as console.error)
     * @return empty string (for use in brackets {} in svelte components)
     */
    error(...args: unknown[]): string {
        return this.callAll('error', args);
    }

    private callAll(
        method: 'debug' | 'info' | 'warn' | 'error',
        args: unknown[],
    ): string {
        for (const logger of this.loggers) {
            logger[method](...args);
        }

        return '';
    }
}
