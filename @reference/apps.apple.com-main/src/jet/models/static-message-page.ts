import { GenericPage } from '@jet-app/app-store/api/models';

const contentTypes = [
    'win-back',
    'carrier',
    'invoice',
    'contingent-price',
] as const;

export type ContentType = (typeof contentTypes)[number];

export class StaticMessagePage extends GenericPage {
    constructor({
        titleLocKey,
        contentType,
    }: {
        titleLocKey: string;
        contentType: ContentType;
    }) {
        super([]);
        this.titleLocKey = titleLocKey;
        this.contentType = contentType;
    }

    titleLocKey?: string;

    // Used to indicate which type of content the page needs to show, used to pull in the proper
    // LOC keys when rendering
    contentType: ContentType;

    // Used in our type guards to narrow a `Page` down to a `StaticMessagePage`
    pageType: string = 'staticMessagePage';
}
