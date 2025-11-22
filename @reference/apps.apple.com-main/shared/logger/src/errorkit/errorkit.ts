import { Severity } from '@sentry/types';
import type { Logger, LoggerFactory } from '../types';
import type {
    captureException,
    captureMessage,
    addBreadcrumb,
    ErrorHub,
    ErrorKitConfig,
} from './types';

type PartialSentryModule = {
    captureException: typeof captureException;
    captureMessage: typeof captureMessage;
    addBreadcrumb: typeof addBreadcrumb;
};

export type ErrorKitInstance = InstanceType<typeof ErrorKit>;

export const setupErrorKit = (
    config: ErrorKitConfig,
    loggerFactory: LoggerFactory,
): ErrorKitInstance | undefined => {
    if (typeof window === 'undefined') return;
    const log = loggerFactory.loggerFor('errorkit');
    const isMultiDev = window.location.href.includes('multidev');
    const BUILD_ENV = process.env.NODE_ENV;
    const isErrorKitEnabled = BUILD_ENV === 'production' && !isMultiDev;

    const initializeErrorKit =
        async (): Promise<PartialSentryModule | null> => {
            let sentry: PartialSentryModule | null = null;

            if (isErrorKitEnabled) {
                try {
                    const { createSentryConfig } = await import(
                        '@amp-metrics/sentrykit'
                    );
                    const Sentry = await import('@sentry/browser');
                    Sentry.init(createSentryConfig(config));

                    sentry = {
                        addBreadcrumb: Sentry.addBreadcrumb,
                        captureException: Sentry.captureException,
                        captureMessage: Sentry.captureMessage,
                    };
                } catch (e) {
                    log.error('something went wrong setting up errorKit', e);
                }
            }

            return sentry;
        };

    return new ErrorKit(initializeErrorKit(), log, isErrorKitEnabled);
};

class ErrorKit implements ErrorHub {
    private readonly sentry: Promise<PartialSentryModule | null>;
    private readonly logger: Logger;
    private readonly isErrorKitEnabled: boolean;
    constructor(
        sentry: Promise<PartialSentryModule | null>,
        log: Logger,
        isErrorKitEnabled: boolean,
    ) {
        this.sentry = sentry;
        this.logger = log;
        this.isErrorKitEnabled = isErrorKitEnabled;

        if (!isErrorKitEnabled) {
            log.debug('errorkit is disabled');
        }
    }

    async captureMessage(message: string) {
        if (!this.isErrorKitEnabled) return;
        const sentry = await this.sentry;

        if (sentry) {
            sentry.addBreadcrumb({
                category: 'log.warn',
                level: Severity.Warning,
            });
            sentry.captureMessage(message, Severity.Warning);
        } else {
            this.logger.warn(`${message} was not sent to errorKit`);
        }
    }

    async captureException(exception: Error) {
        if (!this.isErrorKitEnabled) return;
        const sentry = await this.sentry;

        if (sentry) {
            sentry.addBreadcrumb({
                type: 'error',
                category: 'error',
                level: Severity.Error,
            });
            sentry.captureException(exception);
        } else {
            this.logger.warn(
                `The following exception was not sent to errorKit:`,
                exception,
            );
        }
    }
}
