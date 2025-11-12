export type Level = 'debug' | 'info' | 'warn' | 'error';
// Numbers correspond to the levels above, with 0 meaning "no level"
type LevelNum = 4 | 3 | 2 | 1 | 0;

interface Rules {
    named?: Record<string, LevelNum>;
    defaultLevel?: LevelNum;
}

const LEVEL_TO_NUM: Record<Level | 'off' | '*' | '', LevelNum> = {
    '*': 4,
    debug: 4,
    info: 3,
    warn: 2,
    error: 1,
    off: 0,
    '': 0,
};

/**
 * Parses log filtering instructions from localStorage.onyxLog.
 * The instructions are a series of comma separated directives that restrict
 * logging. Restrictions indicate the highest log level that a named logger
 * will emit. The name of the logger is the string passed to
 * LoggerFactory.loggerFor.
 *
 * By default (ex. empty rule string), no logs will be emitted.
 *
 * The format of the directives is NAME=LEVEL. LEVEL can be one of:
 *
 *   - * - all levels are logged (debug, info, warn, error)
 *   - debug - same as above
 *   - info - everything but debug is logged
 *   - warn - everything but info and debug is logged
 *   - error - only errors are logged
 *   - off (or empty string, ex. "MyClass=") - nothing will be logged
 *
 * Some examples:
 *
 *   - '*=*' will emit all log levels from all loggers
 *   - '*=info,Foo=off' will emit everything but debug except or logs from
 *     the named logger Foo (which will be entirely suppressed)
 *   - 'Bar=error,Baz=warn' will emit errors from Bar and Baz and warnings from
 *     Baz
 *
 * NOTE: Keep this in sync with README.md!
 */
function parseRules(): Rules {
    const onyxLog: string = (() => {
        try {
            // The typeof check is for SSR
            return (
                (typeof window !== 'undefined'
                    ? window.localStorage.onyxLog
                    : '') || ''
            );
        } catch {
            // window.localStorage will throw when referenced (at all) when
            // Chrome has it disabled
            // See: rdar://93367396 (Guard localStorage and sessionStorage use)
            return '';
        }
    })();

    const PRODUCTION_DEFAULT = {}; // no logs unless specified
    const DEV_DEFAULT = {
        defaultLevel: LEVEL_TO_NUM['*'], // All logs unless specified
    };
    const isDevelopment = (() => {
        // This is a little tricky. The ENV var is not real. It's replaced by
        // rollup-plugin-replace. Thus, we can't do the usual of testing for
        // the existence of `process` and then doing `process?.env` etc.
        // Instead, we just try the whole thing and try/catch. This way,
        // rollup-plugin-replace sees that entire string verbatim and can
        // replace it with the proper environment.
        try {
            // @ts-ignore
            return process.env.NODE_ENV !== 'production';
        } catch {
            return false;
        }
    })();
    const defaultRules = isDevelopment ? DEV_DEFAULT : PRODUCTION_DEFAULT;

    // If the localStorage is specified, start from a clean slate. Otherwise,
    // use the environment default
    const rules: Rules = onyxLog.length > 0 ? {} : defaultRules;

    for (const directive of onyxLog.split(',').filter((v) => v)) {
        // Invalid directive, must be of the form 'name=level'
        const parts = directive.split('=');
        if (parts.length !== 2) {
            continue;
        }

        const [name, maxLevelName] = parts;
        const maxLevel =
            LEVEL_TO_NUM[maxLevelName as keyof typeof LEVEL_TO_NUM];

        // Invalid level
        if (typeof maxLevel === 'undefined') {
            continue;
        }

        if (name === '*') {
            rules.defaultLevel = maxLevel;
        } else {
            rules.named = rules.named ?? {};
            rules.named[name] = maxLevel;
        }
    }

    return rules;
}

export function shouldLog(name: string, level: Level): boolean {
    const rules = parseRules();

    // Rules for the named logger take precedence over the default
    const maxLevel = (rules.named || {})[name] ?? rules.defaultLevel ?? 0;
    return LEVEL_TO_NUM[level] <= maxLevel;
}
