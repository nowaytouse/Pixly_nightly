// Browser ONLY logic. Must have the same exports as server.ts
// See: docs/isomorphic-imports.md

import type { Dependencies } from './types';

import { registerHandler as registerFlowActionHandler } from '~/jet/action-handlers/flow-action';
import { registerHandler as registerExternalURLActionHandler } from '~/jet/action-handlers/external-url-action';
import { registerHandler as registerCompoundActionHandler } from '~/jet/action-handlers/compound-action';

export type { Dependencies };

export function registerActionHandlers(dependencies: Dependencies) {
    registerCompoundActionHandler(dependencies);
    registerFlowActionHandler(dependencies);
    registerExternalURLActionHandler(dependencies);
}
