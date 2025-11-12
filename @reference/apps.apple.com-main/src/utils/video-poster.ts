import type { Artwork } from '@jet-app/app-store/api/models';
import type { Profile } from '@amp/web-app-components/src/components/Artwork/types';
import type { Size } from '@amp/web-app-components/src/types';
import type { NamedProfile } from 'src/config/components/artwork';
import { buildSrc } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
import { getDataFromProfile } from '@amp/web-app-components/src/components/Artwork/utils/artProfile';

export const buildPoster = (
    preview: Artwork,
    profile: NamedProfile | Profile,
    mediaQuery: string,
): ReturnType<typeof buildSrc> => {
    const profileData = getDataFromProfile(profile);
    const imageAttributes = profileData[mediaQuery as Size] || preview;
    const dpr = typeof window !== 'undefined' ? window.devicePixelRatio : 2;

    return buildSrc(
        preview.template,
        {
            crop: 'sr',
            width: imageAttributes.width * dpr,
            height: imageAttributes.height * dpr,
            fileType: 'webp',
        },
        {},
    );
};
