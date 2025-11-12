import { MEDIA_API_JWT } from '~/config/media-api';

export class WebMediaTokenService implements MediaTokenService {
    refreshToken(): Promise<string> {
        return Promise.resolve(MEDIA_API_JWT);
    }

    resetToken(): void {
        // No-op; every request uses the same token for the "web" platform
    }
}
