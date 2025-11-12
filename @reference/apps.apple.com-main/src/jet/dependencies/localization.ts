import type I18N from '@amp/web-apps-localization';
import type { LoggerFactory, Logger } from '@amp/web-apps-logger';
import { isNothing } from '@jet/environment';

import type { Locale } from './locale';
import { abbreviateNumber } from '~/utils/number-formatting';
import { getFileSizeParts } from '~/utils/file-size';
import {
    getPlural,
    interpolateString,
} from '@amp/web-apps-localization/src/translator';
import type { Locale as SupportedLanguageIdentifier } from '@amp/web-apps-localization';

const SECONDS_PER_MINUTE = 60;
const SECONDS_PER_HOUR = 60 * 60;
const SECONDS_PER_DAY = SECONDS_PER_HOUR * 24;
const SECONDS_PER_YEAR = SECONDS_PER_DAY * 365;

export function makeWebDoesNotImplementException(property: keyof Localization) {
    return new Error(
        `\`Localization\` method \`${property}\` is not implemented for the "web" platform`,
    );
}

/**
 * Determines if {@linkcode key} appears to be a "client" translation key
 *
 * "Client" keys are defined in `SCREAMING_SNAKE_CASE`
 */
function isClientLocalizationKey(key: string): boolean {
    return /^[A-Z_]+$/.test(key);
}

/**
 * Transforms an App Store Client-used translation key to the format that we have
 * a value for.
 *
 * This accounts for the fact that the "raw" key used by the App Store Client
 * is either a "client" key, that we filed an analogue for in our own translations,
 * or a "server" key that exists in the App Store Client translations under their
 * own namespace. In either case, we need to perform a transformation on the key as
 * they use it into a format that we have a value for.
 */
function transformKeyToSupportedFormat(key: string): string {
    return isClientLocalizationKey(key)
        ? transformClientKeyToSupportedFormat(key)
        : transformServerKeyToSupportedFormat(key);
}

/**
 * Transforms an App Store Client server-side translation key into the format
 * that we have a value for.
 *
 * This handles the fact that the App Store Client namespaces all of
 * their translation strings under `AppStore.` but does not include
 * that namespace when referencing the key. Since their tooling implicitly
 * injects that namespace for them, we have to do the same thing manually.

 * @example
 * transformServerKeyToSupportedFormat('Account.Purchases');
 * // "AppStore.Account.Purchases"
 */
function transformServerKeyToSupportedFormat(key: string): string {
    return `AppStore.${key}`;
}

/**
 * Capitalizes the first character in {@linkcode input}
 */
function capitalizeFirstCharacter(input: string): string {
    const [first, ...rest] = input;

    return first.toUpperCase() + rest.join('');
}

/**
 * Transforms an App Store Client client-side translation key into the format
 * that we have a value for.
 *
 * "Client" keys used by the App Store Client are typically provided by the OS;
 * this is not available to a web application, we need an alternative to providing
 * values for these translation keys.
 *
 * To accomplish this, we have submitted these keys to the server-side localization
 * service ourelves, under a specific namespace that designates that they are the
 * client-side keys that we needed to define. Other formatting changes are made to
 * the key at the request of the LOC team.
 *
 * @example
 * transformClientKeyToSupportedFormat('ACCOUNT_PURCHASES');
 * // "ASE.Web.AppStoreClient.Account.Purchases"
 */
function transformClientKeyToSupportedFormat(key: string): string {
    const keyInSrvLocFormat = key
        .toLowerCase()
        .split('_')
        .map((segment) => capitalizeFirstCharacter(segment))
        .join('.');

    return `ASE.Web.AppStoreClient.${keyInSrvLocFormat}`;
}

/**
 * "Web" implementation of the `AppStoreKit` {@linkcode Localization} dependency
 */
export class WebLocalization implements Localization {
    private readonly locale: Locale;
    private readonly logger: Logger;

    constructor(locale: Locale, loggerFactory: LoggerFactory) {
        this.locale = locale;
        this.logger = loggerFactory.loggerFor('jet/dependency/localization');
    }

    get i18n(): I18N {
        if (this.locale.i18n) {
            return this.locale.i18n;
        }

        throw new Error('`i18n` not yet configured ');
    }

    /**
     * The `BCP 47` identifier for the active locale
     *
     * @see {@link https://developer.apple.com/documentation/foundation/locale | Foundation Frameworks Locale Documentation}
     * @see {@link https://en.wikipedia.org/wiki/IETF_language_tag | BCP 47}
     */
    get identifier() {
        return this.locale.activeLanguage;
    }

    decimal(
        n: number | null | undefined,
        decimalPlaces?: number | null | undefined,
    ): string | null {
        if (isNothing(n)) {
            return null;
        }

        let langCode: string = this.locale.activeLanguage;

        if (!langCode.includes('-')) {
            langCode = `${this.locale.activeLanguage}-${this.locale.activeStorefront}`;
        }

        const numberingSystem = new Intl.NumberFormat(
            langCode,
        ).resolvedOptions().numberingSystem;

        const formatter = new Intl.NumberFormat(this.locale.activeLanguage, {
            numberingSystem,
            minimumFractionDigits: decimalPlaces ?? undefined,
            maximumFractionDigits: decimalPlaces ?? undefined,
        });

        return formatter.format(n);
    }

    string(key: string): string {
        const keyInSupportedFormat = transformKeyToSupportedFormat(key);

        // `.getUninterpolatedString` is used instead of `.t` here to match
        // the behavior of the `.stringWithCount` method
        return this.i18n.getUninterpolatedString(keyInSupportedFormat);
    }

    stringForPreferredLocale(_key: string, _locale: string | null): string {
        throw makeWebDoesNotImplementException('stringForPreferredLocale');
    }

    stringWithCount(key: string, count: number): string {
        let keyInSupportedFormat = transformKeyToSupportedFormat(key);

        // The App Store Client has some behavior around pluralization that differs
        // from how the Media Apps localization normally works. In order to handle
        // this, we have to avoid the default pluralization behavior of the `.i18n.t`
        // method and do the pluralization ourselves
        const keyWithPluralizationSuffix = getPlural(
            count,
            keyInSupportedFormat,
            this.identifier as SupportedLanguageIdentifier,
        );

        // The key difference in pluralization logic is that the `other` case is
        // actually handled by the "base" key without any suffix.
        // Therefore, we should only use the pluralized key if it does not reflect
        // the `other` case
        if (!keyWithPluralizationSuffix.endsWith('.other')) {
            keyInSupportedFormat = keyWithPluralizationSuffix;
        }

        const uninterpolatedValue =
            this.i18n.getUninterpolatedString(keyInSupportedFormat);

        // Since the `count` might be interpolated into the localization string,
        // we need to run the interpolation ourselves on uninterpolated value
        return interpolateString(
            key,
            uninterpolatedValue,
            { count },
            null,
            this.identifier as SupportedLanguageIdentifier,
        );
    }

    stringWithCounts(_key: string, _counts: number[]): string {
        throw makeWebDoesNotImplementException('stringWithCounts');
    }

    uppercased(_value: string): string {
        throw makeWebDoesNotImplementException('uppercased');
    }

    /**
     * Converts a number of bytes into a localized file size string
     *
     * @param bytes The number of bytes to convert
     * @return The localized file size string
     */
    fileSize(bytes: number): string | null {
        let { count, unit } = getFileSizeParts(bytes);

        return this.i18n.t(`ASE.Web.AppStore.FileSize.${unit}`, {
            count,
        });
    }

    formattedCount(count: number | null | undefined): string | null {
        if (isNothing(count)) {
            return null;
        }

        return abbreviateNumber(count, this.locale.activeLanguage);
    }

    formattedCountForPreferredLocale(
        count: number | null,
        locale: string | null,
    ): string | null {
        if (isNothing(count)) {
            return null;
        }

        return isNothing(locale)
            ? abbreviateNumber(count, this.locale.activeLanguage)
            : abbreviateNumber(count, locale);
    }

    /**
     * Convert a date into a time ago label, showing how long ago
     * the date occurred.
     *
     * @param date The date object to convert
     * @return     The localized string representing the amount of time that has passed
     */
    timeAgo(date: Date | null | undefined): string | null {
        if (!date || !(date instanceof Date) || isNaN(date.getTime())) {
            return null;
        }

        const relativeTimeIntl = new Intl.RelativeTimeFormat(
            this.locale.activeLanguage,
            {
                style: 'narrow',
            },
        );

        const now = new Date();

        const secondsAgo = (now.getTime() - date.getTime()) / 1000;
        const minutesAgo = Math.floor(secondsAgo / SECONDS_PER_MINUTE);
        const hoursAgo = Math.floor(secondsAgo / SECONDS_PER_HOUR);
        const daysAgo = Math.floor(secondsAgo / SECONDS_PER_DAY);
        const yearsAgo = Math.floor(secondsAgo / SECONDS_PER_YEAR);
        const isSameYear = now.getFullYear() === date.getFullYear();
        const isUpcoming = date.getTime() > now.getTime();

        if (secondsAgo < 0 && isUpcoming) {
            return new Intl.DateTimeFormat(this.locale.activeLanguage, {
                month: 'short',
                day: 'numeric',
            }).format(date);
        }

        if (secondsAgo < 60) {
            return relativeTimeIntl.format(-secondsAgo, 'seconds');
        }

        if (minutesAgo < 60) {
            return relativeTimeIntl.format(-minutesAgo, 'minutes');
        }

        if (hoursAgo < 24) {
            return relativeTimeIntl.format(-hoursAgo, 'hours');
        }

        if (daysAgo < 7) {
            return relativeTimeIntl.format(-daysAgo, 'days');
        }

        if (isSameYear) {
            return new Intl.DateTimeFormat(this.locale.activeLanguage, {
                month: 'short',
                day: 'numeric',
            }).format(date);
        }

        if (yearsAgo >= 0) {
            return new Intl.DateTimeFormat(this.locale.activeLanguage, {
                day: '2-digit',
                month: '2-digit',
                year: 'numeric',
            }).format(date);
        }

        // this return statement is here to satisfy typescript, all possible cases are
        // satisfied by the above conditionals.
        return null;
    }

    timeAgoWithContext(
        _date: Date | null | undefined,
        _context: DateContext,
    ): string | null {
        return null;
    }

    formatDate(format: string, date: Date | null | undefined): string | null {
        if (isNothing(date)) {
            return null;
        }

        let formatterConfiguration: Intl.DateTimeFormatOptions | undefined;

        switch (format) {
            case 'MMM d': // e.g. Jan 29
                formatterConfiguration = {
                    month: 'short',
                    day: 'numeric',
                };
                break;
            case 'MMMM d': // e.g. January 29
                formatterConfiguration = {
                    month: 'long',
                    day: 'numeric',
                };
                break;
            case 'j:mm': // e.g. 9:00
                formatterConfiguration = {
                    hour: 'numeric',
                    minute: '2-digit',
                };
                break;
            case 'MMM d, y': // e.g. Jan 29, 2025
                formatterConfiguration = {
                    month: 'short',
                    day: 'numeric',
                    year: 'numeric',
                };
                break;
            case 'MMMM d, y': // e.g. "January 29, 2025"
                formatterConfiguration = {
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric',
                };
                break;
            case 'EEE j:mm': // e.g. "SAT 9:00PM"
                formatterConfiguration = {
                    weekday: 'short',
                    hour: 'numeric',
                    minute: '2-digit',
                    hour12: true,
                };
                break;
            case 'd، MMM، yyyy': // e.g. "29 Jan 2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;
            case 'MMM d, yyyy': // e.g. "Jan 29, 2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;
            case 'd MMM yyyy': // e.g. "29 January 2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'long',
                    year: 'numeric',
                };
                break;
            case 'yyyy MMMM d': // e.g. "2025 January 29"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'long',
                    year: 'numeric',
                };
            case 'd M yyyy':
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;
            case 'd MMM., yyyy':
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'long',
                    year: 'numeric',
                };
                break;
            case 'dd/MM/yyyy': // e.g. "29/01/2025"
                formatterConfiguration = {
                    day: '2-digit',
                    month: '2-digit',
                    year: 'numeric',
                };
                break;
            case 'd MMM , yyyy': // e.g. "29 Jan , 2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;
            case 'd. MMM. yyyy.': // e.g. "29. Jan. 2025."
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;

            case 'd. MMM yyyy': // e.g. "29. Jan 2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;

            case 'yyyy. MMM d.': // e.g. "2025. Jan 29."
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                };
                break;

            case 'd.M.yyyy': // e.g. "29.1.2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'numeric',
                    year: 'numeric',
                };
                break;

            case 'd/M/yyyy': // e.g. "29/1/2025"
                formatterConfiguration = {
                    day: 'numeric',
                    month: 'numeric',
                    year: 'numeric',
                };
                break;
            default:
                this.logger.warn(
                    `\`formatDate\` called with unexpected format \`${format}\``,
                );
                return null;
        }

        return new Intl.DateTimeFormat(
            this.locale.activeLanguage,
            formatterConfiguration,
        ).format(date);
    }

    formatDateWithContext(
        format: string,
        date: Date | null | undefined,
        _context: DateContext,
    ): string | null {
        return this.formatDate(format, date);
    }

    formatDateInSentence(
        sentence: string,
        format: string,
        date: Date | null | undefined,
    ): string | null {
        const formattedDate = this.formatDate(format, date);

        if (isNothing(formattedDate)) {
            return null;
        }

        return (
            sentence
                // "Server-Side" LOC keys us `@@date@@` to mark the date to replace
                .replace('@@date@@', formattedDate)
                // "Client-Side" LOC keys use `%@` to mark the date to replace
                .replace('%@', formattedDate)
        );
    }

    relativeDate(date: Date | null | undefined): string | null {
        if (isNothing(date)) {
            return null;
        }

        return date.toString();
    }

    formatDuration(_value: number, _unit: TimeUnit): string | null {
        throw makeWebDoesNotImplementException('formatDuration');
    }
}
