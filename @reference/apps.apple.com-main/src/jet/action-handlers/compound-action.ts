import type { LoggerFactory } from '@amp/web-apps-logger';
import type { Jet } from '~/jet';
import type { CompoundAction } from '~/jet/models';

export type Dependencies = {
    jet: Jet;
    logger: LoggerFactory;
};

export async function registerHandler(dependencies: Dependencies) {
    const { jet, logger } = dependencies;

    const log = logger.loggerFor('jet/action-handlers/compound-action');

    jet.onAction('compoundAction', async (action: CompoundAction) => {
        log.info('received CompoundAction:', action);

        const { subactions = [] } = action;

        // Perform actions in sequence
        for (const action of subactions) {
            await jet.perform(action).catch((e) => {
                // Throwing error stops for...of execution
                // TODO: rdar://73165545 (Error Handling Across App)
                throw new Error(
                    `an error occurred while handling CompoundAction: ${e}`,
                );
            });
        }

        return 'performed';
    });
}
