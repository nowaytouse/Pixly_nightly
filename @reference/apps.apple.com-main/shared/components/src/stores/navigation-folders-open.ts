import { type Writable, writable } from 'svelte/store';

type FolderState = Writable<boolean>;
const folderStates = new Map<string, FolderState>();

export function subscribeFolderOpenState(
    id: string,
    defaultState?: boolean,
): FolderState {
    let stateById = folderStates.get(id);
    if (!stateById) {
        folderStates.set(id, writable(defaultState ?? false));
        stateById = folderStates.get(id);
    }

    return stateById;
}

export function resetFoldersOpenState() {
    folderStates.clear();
}
