import { getModalPageStore } from '~/stores/modalPage';
import { isGenericPage, type Page } from '../models';
import type { Logger } from '@amp/web-apps-logger/src';

/**
 * This function handles rendering flow action pages into a modal container.
 * NOTE: Rendering a page in a modal will not update URL or history
 *
 * @param page page promise
 * @param log app logger
 */
export const handleModalPresentation = (
    page: { promise: Promise<Page> },
    log: Logger<unknown[]>,
    pageDetail?: string,
) => {
    page.promise
        .then((page) => {
            if (isGenericPage(page)) {
                const modalStore = getModalPageStore();
                modalStore.setPage({ page, pageDetail });
            } else {
                throw new Error('only generic page is rendered in modal');
            }
        })
        .catch((e) => {
            log.error('modal presentation failed', e);
        });
};
