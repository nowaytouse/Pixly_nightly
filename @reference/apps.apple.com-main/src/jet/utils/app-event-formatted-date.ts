import {
    type Optional,
    isSome,
    isNothing,
} from '@jet/environment/types/optional';
import type { LocalizationWrapper } from '@jet-app/app-store/foundation/wrappers/localization';
import type {
    AppEventFormattedDate,
    AppEventBadgeKind,
} from '@jet-app/app-store/api/models';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { formattedDatesWithKind } from '@jet-app/app-store/common/app-promotions/app-event';

/**
 * Partial type of {@linkcode AppEventFormattedDate} with just the properties
 * that are actually used
 */
export type RequiredAppEventFormattedDate = Pick<
    AppEventFormattedDate,
    'displayText' | 'displayFromDate' | 'countdownToDate' | 'countdownStringKey'
>;

/**
 * Represents a client-side serialization of an {@linkcode RequiredAppEventFormattedDate}
 *
 * This is needed because our client-side code will receive the event object with `Date` properties
 * serialized as ISO 8601-formatted strings, while the server-side code will receive the original
 * `Date` values. We need to normalize this to make sure we have consistent logic in both environments
 */
type SerializedAppEventFormattedDate = Pick<
    RequiredAppEventFormattedDate,
    'displayText' | 'countdownStringKey'
> & {
    readonly displayFromDate?: string;
    readonly countdownToDate?: string;
};

function deserializeDate(value: Optional<Date | string>): Date | undefined {
    if (isNothing(value)) {
        return undefined;
    }

    return typeof value === 'string' ? new Date(value) : value;
}

/**
 * Turn {@linkcode date} in either the client- or server-side format into the
 * server-side format by parsing the ISO 8601 string values into `Date` instances
 */
function deserializeDateProperties(
    date: SerializedAppEventFormattedDate | RequiredAppEventFormattedDate,
): RequiredAppEventFormattedDate {
    const { countdownToDate, displayFromDate, ...rest } = date;

    return {
        // Normalize properties that might have been serialized as `string` to `Date`
        countdownToDate: deserializeDate(countdownToDate),
        displayFromDate: deserializeDate(displayFromDate),

        // Use all of the other properties with their existing values
        ...rest,
    };
}

/**
 * A {@linkcode RequiredAppEventFormattedDate} with a definitely-defined `.displayFromDate` property
 */
type AppEventFormattedDateWithDisplayFromDate =
    RequiredAppEventFormattedDate & {
        readonly displayFromDate: Date;
    };

function hasDisplayRequirement(
    date: RequiredAppEventFormattedDate,
): date is AppEventFormattedDateWithDisplayFromDate {
    return isSome(date.displayFromDate);
}

export function chooseAppEventDate(
    dates: (SerializedAppEventFormattedDate | RequiredAppEventFormattedDate)[],
): Optional<RequiredAppEventFormattedDate> {
    const nowTime = Date.now();

    // We might be passed `dates` in the expected format (server-side) or with their `Date`
    // properties serialized as strings (client-side); we need to normalize them all to the
    // same format
    const normalizedDates = dates.map((date) =>
        deserializeDateProperties(date),
    );

    // A `dates` member might not have a `.displayFromDate`; if that's the case, we will
    // use that as a fallback if all other options are in the future
    const fallback = normalizedDates.find(
        (date) => !hasDisplayRequirement(date),
    );

    // Find all of the `dates` members with a `.displayFromDate` in the past
    const optionsWithPastDisplayFromDates = normalizedDates
        // Ensure all `date` objects have a display requirement
        .filter((date) => hasDisplayRequirement(date))
        // Filter out any `date` objects with a display requirement in the future
        .filter((date) => {
            const dateTime = date.displayFromDate.getTime();
            const timeDifference = nowTime - dateTime;

            return timeDifference > 0;
        });

    // If there are none, use the fallback
    if (optionsWithPastDisplayFromDates.length === 0) {
        return fallback;
    }

    // Otherwise, find the `date` object with the most recent `.displayFromDate`
    return optionsWithPastDisplayFromDates.reduce((acc, next) => {
        const accTime = acc.displayFromDate.getTime();
        const nextTime = next.displayFromDate.getTime();

        // Which time is closer to "now"?
        const accTimeDiff = nowTime - accTime;
        const nextTimeDiff = nowTime - nextTime;

        return accTimeDiff > nextTimeDiff ? next : acc;
    });
}

/**
 * Partial type of {@linkcode LocalizationWrapper} with just the methods that
 * are actually called
 *
 * This partial type simplifies testing by reducing the surface area of the function's
 * dependencies
 */
type RequiredLocalization = Pick<LocalizationWrapper, 'string'>;

function msToMinutes(ms: number): number {
    return ms / (1_000 * 60);
}

export function renderDate(
    localization: RequiredLocalization,
    date: RequiredAppEventFormattedDate,
): Optional<string> {
    if (typeof date.countdownStringKey === 'string' && date.countdownToDate) {
        const nowTime = Date.now();
        const translationString = localization.string(date.countdownStringKey);

        const countdownToDateTime = date.countdownToDate.getTime();
        const diffTime = countdownToDateTime - nowTime;

        const count = Math.floor(msToMinutes(diffTime));

        return translationString.replace('@@count@@', count.toString());
    }

    if (typeof date.displayText === 'string') {
        return date.displayText;
    }

    return undefined;
}

/**
 * Helper function to compute formatted dates for app events.
 * Handles date conversion and error handling.
 *
 * @param objectGraph - objectGraph from Jet
 * @param badgeKind - The badge kind from the app event
 * @param startDate - The start date (string or Date)
 * @param endDate - The optional end date (string or Date)
 * @returns Array of formatted dates or undefined if an error occurs
 */
export function computeAppEventFormattedDates(
    objectGraph: AppStoreObjectGraph,
    badgeKind: AppEventBadgeKind,
    startDate: string | Date,
    endDate?: string | Date | null,
): RequiredAppEventFormattedDate[] | undefined {
    // Use deserializeDate function to convert dates
    const startDateObj = deserializeDate(startDate);
    const endDateObj = deserializeDate(endDate);

    // Validate that we have a valid start date
    if (!startDateObj || isNaN(startDateObj.getTime())) {
        return undefined;
    }

    return formattedDatesWithKind(
        objectGraph,
        badgeKind,
        startDateObj,
        endDateObj,
    );
}
