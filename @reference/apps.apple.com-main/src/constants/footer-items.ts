import type { FooterItem } from '@amp/web-app-components/src/components/Footer/types';

export const items: FooterItem[] = [
    {
        id: 'terms-of-use',
        url: 'AMP.Shared.Footer.TermsOfUse.URL',
        locKey: 'AMP.Shared.Footer.TermsOfUse.Text',
    },
    {
        id: 'privacy-policy',
        url: 'ASE.Web.AppStore.Shared.Footer.PrivacyPolicy.URL',
        locKey: 'ASE.Web.AppStore.Shared.Footer.PrivacyPolicy.Text',
    },
    {
        id: 'cookie-policy',
        url: 'AMP.Shared.Footer.CookiePolicy.URL',
        locKey: 'AMP.Shared.Footer.CookiePolicy.Text',
    },
    {
        id: 'get-help',
        url: 'ASE.Web.AppStore.Shared.Footer.GetHelp.URL',
        locKey: 'ASE.Web.AppStore.Shared.Footer.GetHelp.Text',
    },
];
