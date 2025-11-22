/**
 * `AppStoreKit` `Storage` implementation for the "web" client
 *
 * Note: The `AppStoreKit` `Storage` interface is declared as a global, which has the (presumably
 * accidental) side-effect of implicitly being merged with the DOM library's own `Storage` interface
 * (like `localStorage`), since interfaces declared in the same scope are merged together by TypeScript.
 * There's no way to tell TypeScript that we only care about the `AppStoreKit` part of it, so
 * satifying TypeScript here means that we need to implement both interfaces.
 */
export class WebStorage extends Map<string, string> implements Storage {
    /* == "DOM" `Storage` Interface == */

    get length() {
        return this.size;
    }

    getItem(key: string): string | null {
        return this.get(key) ?? null;
    }

    key(_index: number): string | null {
        throw new Error('Method not implemented.');
    }

    removeItem(key: string): void {
        this.delete(key);
    }

    setItem(key: string, value: string): void {
        this.set(key, value);
    }

    /* == AppStoreKit `Storage` Interface == */

    storeString(aString: string, key: string): void {
        this.set(key, aString);
    }

    retrieveString(key: string): string {
        // Fallback value designed based on how the ObjectGraph `StorageWrapper` handles that specific value
        // https://github.pie.apple.com/app-store/ios-appstore-app/blob/1761d575b8dc3d7a63e7e36f3320cf9245be9f37/src/foundation/wrappers/storage.ts#L13
        return this.get(key) ?? '<null>';
    }
}
