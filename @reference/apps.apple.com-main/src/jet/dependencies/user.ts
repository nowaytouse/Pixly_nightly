/**
 * Create an "unauthenticated" {@linkcode User} representation
 *
 * The property values below match the way that `AppStoreKit` will define the `user`
 * when the session is not authenticated.
 */
export function makeUnauthenticatedUser(): User {
    return {
        accountIdentifier: undefined,
        dsid: undefined,
        firstName: undefined,
        // Note: this property is `true` for the native apps but `false` makes
        // more sense in the context of the "web" client
        isFitnessAppInstallationAllowed: false,
        isManagedAppleID: false,
        isOnDevicePersonalizationEnabled: false,
        isUnderThirteen: false,
        katanaId: undefined,
        lastName: undefined,
        treatmentGroupIdOverride: undefined,
        userAgeIfAvailable: undefined,

        onDevicePersonalizationDataContainerForAppIds(appIds) {
            return {
                personalizationData: {},
                metricsData: {},
            };
        },
    };
}
