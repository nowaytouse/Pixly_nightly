import type {
    ClientIdentifier,
    Host as NativeHost,
    ProcessPlatform,
} from '@jet/environment';
import type {} from '@jet/engine'; // For ClientIdentifier.Unknown

export class Host implements NativeHost {
    platform: ProcessPlatform = 'web';

    get osBuild(): never {
        throw makeWebDoesNotImplementException('osBuild');
    }

    get deviceModel(): string {
        return 'web';
    }

    get devicePhysicalModel(): never {
        throw makeWebDoesNotImplementException('devicePhysicalModel');
    }

    get deviceLocalizedModel() {
        return '';
    }

    get deviceModelFamily(): never {
        throw makeWebDoesNotImplementException('deviceModelFamily');
    }

    get clientIdentifier(): ClientIdentifier {
        // We can't directly use the `ClientIdentifier.Unknown` enum member value
        // because we cannot access "ambient const enums" with our TypeScript config.
        // Enum handling is known to be tough in TypeScript and, for reasons like
        // this, they are generally avoided.
        // This returns a value defined on this enum by `@jet/engine`'s type definition
        return 'unknown' as ClientIdentifier.Unknown;
    }

    get clientVersion(): never {
        throw makeWebDoesNotImplementException('clientVersion');
    }

    isOSAtLeast(
        _majorVersion: number,
        _minorVersion: number,
        _patchVersion: number,
    ): boolean {
        return true;
    }
}

export function makeWebDoesNotImplementException(property: keyof NativeHost) {
    return new Error(
        `\`Host\` property \`${property}\` is not implemented for the "web" platform`,
    );
}
