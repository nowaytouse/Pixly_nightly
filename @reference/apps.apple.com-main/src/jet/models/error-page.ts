import { GenericPage } from '@jet-app/app-store/api/models';
import type { Opt } from '@jet/environment';

export class ErrorPage extends GenericPage {
    constructor({ error }: { error: Opt<Error> }) {
        super([]);
        this.error = error;
    }

    // Used in our type guards to narrow a `Page` down to a `ErrorPage`
    pageType: string = 'errorPage';

    // The browser `Error`, used to determine which message to display to the user
    error: Opt<Error>;
}
