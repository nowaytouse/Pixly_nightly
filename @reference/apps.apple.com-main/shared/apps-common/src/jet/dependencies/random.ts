import type { Random as IRandom } from '@jet/environment';
import { generateUuid } from '@amp/web-apps-utils';

export class Random implements IRandom {
    nextBoolean(): boolean {
        // See: https://stashweb.sd.apple.com/projects/AS/repos/jet-infrastructure/browse/Frameworks/JetEngine/JetEngine/JavaScript/Stack/Native%20APIs/JSRandomObject.swift?at=e90a88fa061f5cb6b9536d29a7ffd67e5db942db#41
        return Math.random() < 0.5;
    }

    nextNumber(): number {
        // See: https://stashweb.sd.apple.com/projects/AS/repos/jet-infrastructure/browse/Frameworks/JetEngine/JetEngine/JavaScript/Stack/Native%20APIs/JSRandomObject.swift?at=e90a88fa061f5cb6b9536d29a7ffd67e5db942db#45
        return Math.random();
    }

    nextUUID(): string {
        return generateUuid();
    }
}
