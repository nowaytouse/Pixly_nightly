import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';
import { getMediaConditions } from '@amp/web-app-components/src/utils/getMediaConditions';
import { buildMediaQueryStore } from '@amp/web-app-components/src/stores/media-query';

const { BREAKPOINTS } = ArtworkConfig.get();

const mediaQueryStore = buildMediaQueryStore(
    'medium',
    getMediaConditions(BREAKPOINTS, { offset: 260 }),
);

export default mediaQueryStore;
