import type { LoggerFactory, Logger } from '@amp/web-apps-logger';
import type { RequiredConsole } from '@jet-app/app-store/foundation/wrappers/console';

export class WebConsole implements RequiredConsole {
    private readonly logger: Logger;

    constructor(loggerFactory: LoggerFactory) {
        this.logger = loggerFactory.loggerFor('jet-console');
    }

    error(...data: unknown[]): void {
        this.logger.error(...data);
    }

    info(...data: unknown[]): void {
        this.logger.info(...data);
    }

    log(...data: unknown[]): void {
        this.logger.info(...data);
    }

    warn(...data: unknown[]): void {
        this.logger.warn(...data);
    }
}
