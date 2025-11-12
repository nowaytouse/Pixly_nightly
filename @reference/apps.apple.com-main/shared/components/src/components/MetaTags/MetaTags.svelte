<script lang="ts">
    import { LTR_MARK, RTL_MARK } from '@amp/web-app-components/src/constants';
    import type { Locale } from '@amp/web-app-components/src/types';
    import type {
        SeoData,
        HreflangTag,
    } from '@amp/web-app-components/src/components/MetaTags/types';
    import type { ImageURLParams } from '@amp/web-app-components/src/components/Artwork/types';
    import { buildSrcSeo } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
    import { serializeJSONData } from '@amp/web-app-components/src/utils/sanitize';

    export let seoData: SeoData | undefined = undefined;
    export let locale: Locale;
    export let origin: string;
    export let pageDir: string;
    export let defaultTitle: string;
    export let hreflangTags: HreflangTag[] | null = null;

    // Music's Classical Bridge prefers to use a different canonical
    // for rel=canonical tags than the page url. Uses page url as fallback.
    $: canonicalUrl = seoData?.canonicalUrl ?? seoData?.url;
    $: pageTitle = seoData?.pageTitle ?? defaultTitle;
    $: formattedLocale = locale.language.replace(/-/g, '_') || null;
    $: directionMarker = pageDir === 'rtl' ? RTL_MARK : LTR_MARK;

    function processSocialImage(
        artworkUrl: string,
        imgParams: ImageURLParams,
    ): string | undefined {
        if (artworkUrl.startsWith('/')) {
            artworkUrl = `${origin}${artworkUrl}`;
        }
        return buildSrcSeo(artworkUrl, imgParams);
    }

    $: ogImageUrl = !!seoData?.artworkUrl
        ? processSocialImage(seoData.artworkUrl, {
              width: seoData.width,
              height: seoData.height,
              crop: seoData.crop,
              fileType: seoData.fileType,
              quality: seoData.quality,
          })
        : null;
    $: twitterImageUrl = !!seoData?.artworkUrl
        ? processSocialImage(seoData.artworkUrl, {
              width: seoData.twitterWidth,
              height: seoData.twitterHeight,
              crop: seoData.twitterCropCode,
              fileType: seoData.fileType,
              quality: seoData.quality,
          })
        : null;

    $: sanitizedSchemaContent = !!seoData?.schemaContent
        ? serializeJSONData(seoData.schemaContent)
        : null;

    $: sanitizedBreadcrumbSchemaContent = !!seoData?.breadcrumbSchemaContent
        ? serializeJSONData(seoData.breadcrumbSchemaContent)
        : null;
</script>

<svelte:head>
    {#if pageTitle}
        <!--directionMarker forces the direction so we don't get "....More from "some rtl text""-->
        <title>{directionMarker}{pageTitle}</title>
    {/if}

    {#if !!seoData}
        <!-- Begin General -->
        <!-- NOTE: If configuring robots tags, use one of these options, but not both -->
        {#if seoData.noFollow}
            <!-- Use this when you do not want your page indexed or your links followed -->
            <meta name="robots" content="noindex, nofollow" />
        {:else if seoData.noIndex}
            <!-- Use this when you want your links followed but not have the page indexed -->
            <meta name="robots" content="noindex" />
        {/if}

        {#if seoData.description}
            <meta name="description" content={seoData.description} />
        {/if}

        {#if seoData.keywords}
            <meta name="keywords" content={seoData.keywords} />
        {/if}

        {#if canonicalUrl}
            <link rel="canonical" href={canonicalUrl} />
        {/if}

        {#if hreflangTags}
            {#each hreflangTags as langTag}
                {#if langTag}
                    <link
                        rel="alternate"
                        href={langTag.path}
                        hreflang={langTag.tag}
                    />
                {/if}
            {/each}
        {/if}
        <!-- End General -->

        {#if !!seoData.oembedData?.url}
            <link
                rel="alternate"
                type="application/json+oembed"
                href={`${origin}/api/oembed?url=${encodeURIComponent(
                    seoData.oembedData.url,
                )}`}
                title={seoData.oembedData.title ?? ''}
            />
        {/if}

        <!-- Begin Apple-specific meta tags -->
        {#if seoData.appleStoreId}
            <meta name="al:ios:app_store_id" content={seoData.appleStoreId} />
        {/if}

        {#if seoData.appleStoreName}
            <meta name="al:ios:app_name" content={seoData.appleStoreName} />
        {/if}

        {#if seoData.appleContentId}
            <meta name="apple:content_id" content={seoData.appleContentId} />
        {/if}

        {#if seoData.appleTitle}
            <meta name="apple:title" content={seoData.appleTitle} />
        {/if}

        {#if seoData.appleDescription}
            <meta name="apple:description" content={seoData.appleDescription} />
        {/if}
        <!-- End Apple-specific meta tags -->

        <!-- Begin OpenGraph (FaceBook, Slack, etc) -->
        {#if seoData.socialTitle}
            <meta property="og:title" content={seoData.socialTitle} />
        {/if}

        {#if seoData.socialDescription}
            <meta
                property="og:description"
                content={seoData.socialDescription}
            />
        {/if}

        {#if seoData.siteName}
            <meta property="og:site_name" content={seoData.siteName} />
        {/if}

        {#if seoData.url}
            <meta property="og:url" content={seoData.url} />
        {/if}

        {#if ogImageUrl}
            <meta property="og:image" content={ogImageUrl} />
            <meta property="og:image:secure_url" content={ogImageUrl} />

            {#if seoData.imageAltTitle}
                <meta property="og:image:alt" content={seoData.imageAltTitle} />
            {:else if seoData.socialTitle}
                <meta property="og:image:alt" content={seoData.socialTitle} />
            {/if}

            {#if seoData.width}
                <meta
                    property="og:image:width"
                    content={seoData.width.toString()}
                />
            {/if}

            {#if seoData.height}
                <meta
                    property="og:image:height"
                    content={seoData.height.toString()}
                />
            {/if}

            {#if seoData.fileType}
                <meta
                    property="og:image:type"
                    content={`image/${seoData.fileType}`}
                />
            {/if}
        {/if}

        {#if seoData.ogType}
            <meta property="og:type" content={seoData.ogType} />
        {/if}

        {#if seoData.socialTitle && formattedLocale}
            <meta property="og:locale" content={formattedLocale} />
        {/if}

        {#if $$slots['extendedOpenGraphData']}
            <slot name="extendedOpenGraphData" />
        {/if}
        <!-- End OpenGraph -->

        <!-- Begin Twitter -->
        {#if seoData.socialTitle}
            <meta name="twitter:title" content={seoData.socialTitle} />
        {/if}

        {#if seoData.socialDescription}
            <meta
                name="twitter:description"
                content={seoData.socialDescription}
            />
        {/if}

        {#if seoData.twitterSite}
            <meta name="twitter:site" content={seoData.twitterSite} />
        {/if}

        {#if twitterImageUrl}
            <meta name="twitter:image" content={twitterImageUrl} />

            {#if seoData.imageAltTitle}
                <meta
                    name="twitter:image:alt"
                    content={seoData.imageAltTitle}
                />
            {:else if seoData.socialTitle}
                <meta name="twitter:image:alt" content={seoData.socialTitle} />
            {/if}
        {/if}

        {#if seoData.twitterCardType}
            <meta name="twitter:card" content={seoData.twitterCardType} />
        {/if}
        <!-- End Twitter -->

        <!-- Begin schema.org -->
        {#if $$slots['schemaOrganizationData']}
            <slot name="schemaOrganizationData" />
        {/if}

        {#if seoData.schemaName && sanitizedSchemaContent}
            {@html `
                <script id=${seoData.schemaName} type="application/ld+json">
                    ${sanitizedSchemaContent}
                </script>
                `}
        {/if}
        <!-- End schema.org -->

        <!-- Begin breadcrumb schema -->
        {#if seoData.breadcrumbSchemaName && sanitizedBreadcrumbSchemaContent}
            {@html `
                <script id=${seoData.breadcrumbSchemaName} name=${seoData.breadcrumbSchemaName} type="application/ld+json">
                    ${sanitizedBreadcrumbSchemaContent}
                </script>
                `}
        {/if}
        <!-- End breadcrumb schema -->
    {/if}
</svelte:head>
