// Breaks duration down from milliseconds into hours/minutes/seconds
export function getDurationParts(durationInMilliseconds: number): {
    hours: number;
    minutes: number;
    seconds: number;
} {
    // convert ms to seconds
    const durationInSeconds = Math.floor(durationInMilliseconds / 1000);
    const duration = Math.round(durationInSeconds);

    return {
        hours: Math.floor(duration / 3600),
        minutes: Math.floor(duration / 60) % 60,
        seconds: duration % 60,
    };
}

// returns normal numeric date in YYYY-MM-DD from a date string
// AKA getNumericDateFromReleaseDate but renamed to be more generic
//
// ex: getNumericDateFromDateString('2024-04-15T08:41:03Z') => '2024-04-15'
//     getNumericDateFromDateString('15 April 2024 14:48 UTC') => '2024-04-15'
export function getNumericDateFromDateString(
    timestamp?: string,
): string | undefined {
    if (!timestamp) {
        return undefined;
    }

    return new Date(timestamp).toISOString().split('T')?.[0];
}

// Utility to format ISO8601 Duration Strings from raw milliseconds (ex: PT2M42S).
export function formatISODuration(durationInMilliseconds: number): string {
    const { hours, minutes, seconds } = getDurationParts(
        durationInMilliseconds,
    );

    if (!hours && !minutes && !seconds) {
        return 'P0D';
    }

    return [
        'PT',
        hours && `${hours}H`,
        minutes && `${minutes}M`,
        seconds && `${seconds}S`,
    ]
        .filter(Boolean)
        .join('');
}
