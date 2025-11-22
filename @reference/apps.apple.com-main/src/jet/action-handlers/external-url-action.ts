import type { Jet } from '~/jet';
import type { LoggerFactory } from '@amp/web-apps-logger';
import type { ExternalUrlAction } from '@jet-app/app-store/api/models';

export type Dependencies = {
    jet: Jet;
    logger: LoggerFactory;
};

export function registerHandler(dependencies: Dependencies) {
    const { jet, logger } = dependencies;

    const log = logger.loggerFor('jet/action-handlers/external-url-action');

    jet.onAction('ExternalUrlAction', async (action: ExternalUrlAction) => {
        log.info('received external URL action:', action);
        return 'performed';
    });
}
