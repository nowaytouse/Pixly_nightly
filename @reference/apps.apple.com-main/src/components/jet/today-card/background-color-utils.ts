import { type Optional, isSome } from '@jet/environment/types/optional';
import type {
    Color,
    TodayCardMedia,
    TodayCardMediaWithArtwork,
} from '@jet-app/app-store/api/models';

import { isTodayCardMediaBrandedSingleApp } from './media/TodayCardMediaBrandedSingleApp.svelte';
import { isTodayCardMediaAppEvent } from './media/TodayCardMediaAppEvent.svelte';
import { isTodayCardMediaWithArtwork } from './media/TodayCardMediaWithArtwork.svelte';

const DEFAULT_COLOR: Color = {
    type: 'named',
    name: 'defaultBackground',
};

function getBackgroundFromMediaWithArtwork(
    media: TodayCardMediaWithArtwork,
): Optional<Color> {
    return (
        media.videos[0]?.preview.backgroundColor ??
        media.artworks[0]?.backgroundColor
    );
}

/**
 * Onyx App Store alternative to the `bestBackgroundColor` method that exists on
 * the {@linkcode TodayCardMedia} type
 *
 * This is necessary because the functions on those class instances are not
 * carried over to the client when serializing the view-model, making them
 * impossible to call in a consistent way from our codebase
 */
export function bestBackgroundColor(media: Optional<TodayCardMedia>): Color {
    if (isSome(media)) {
        if (isTodayCardMediaAppEvent(media)) {
            return media.tintColor;
        }

        if (isTodayCardMediaBrandedSingleApp(media)) {
            return (
                getBackgroundFromMediaWithArtwork(media) ??
                media.icon.backgroundColor ??
                DEFAULT_COLOR
            );
        }

        if (isTodayCardMediaWithArtwork(media)) {
            return getBackgroundFromMediaWithArtwork(media) ?? DEFAULT_COLOR;
        }
    }

    return DEFAULT_COLOR;
}
