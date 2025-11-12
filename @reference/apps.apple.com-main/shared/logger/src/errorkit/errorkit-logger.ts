import type { ErrorHub, ValueOf } from './types';
import type { LoggerFactory, Logger } from '../types';

/**
 * Determines the level of logs to send to sentry.
 *
 */
export const ERROR_REPORT_LEVEL = {
    error: 'error',
    error_warn: 'error_warn',
} as const;

type ReportLevel = ValueOf<typeof ERROR_REPORT_LEVEL>;

export class ErrorKitLoggerFactory implements LoggerFactory {
    private readonly errorKit: ErrorHub;
    private readonly reportLevel: ReportLevel;
    constructor(errorKit: ErrorHub, reportLevel?: ReportLevel) {
        this.errorKit = errorKit;
        this.reportLevel = reportLevel ?? ERROR_REPORT_LEVEL.error;
    }
    loggerFor(name: string): Logger {
        return new ErrorKitLogger(name, this.errorKit, this.reportLevel);
    }
}

interface HasToString {
    toString(): string;
}

export class ErrorKitLogger implements Logger {
    private readonly name: string;
    private readonly errorKit: ErrorHub;
    private readonly reportLevel: ReportLevel;
    constructor(name: string, errorKit: ErrorHub, reportLevel: ReportLevel) {
        this.name = name;
        this.errorKit = errorKit;
        this.reportLevel = reportLevel;
    }

    private stringifyConsoleArgs(...args: unknown[]): string {
        return args.reduce((acc: string, val: unknown) => {
            let tempVal: HasToString;
            switch (true) {
                case val instanceof Error: {
                    tempVal = (val as unknown as InstanceType<typeof Error>)
                        .message;
                    break;
                }
                case typeof val === 'object': {
                    try {
                        tempVal = JSON.stringify(val);
                    } catch (e) {
                        tempVal = `failed to stringify ${val}`;
                    }
                    break;
                }
                case typeof val === 'undefined' || val === null: {
                    tempVal = `${val}`;
                    break;
                }
                default: {
                    tempVal = val as HasToString;
                }
            }

            return `${acc} ${tempVal.toString()}`;
        }, `[${this.name}]`) as string;
    }

    debug(..._args: unknown[]): string {
        return '';
    }
    info(..._args: unknown[]): string {
        return '';
    }
    warn(...args: unknown[]): string {
        if (this.reportLevel === ERROR_REPORT_LEVEL.error_warn) {
            this.errorKit.captureMessage(this.stringifyConsoleArgs(...args));
        }
        return '';
    }
    error(...args: unknown[]): string {
        const errors = args.filter((item) => item instanceof Error) as Error[];
        const message = this.stringifyConsoleArgs(...args);

        const error = errors.length === 0 ? new Error(message) : errors[0];
        error.message = message;

        this.errorKit.captureException(error);
        return '';
    }
}
