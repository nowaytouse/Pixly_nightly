const IS_RGB = /^rgba?\(\s*[\d.]+\s*%?\s*(,\s*[\d.]+\s*%?\s*){2,3}\)$/;
const IS_HEX = /^([0-9a-f]{3}){1,2}$/i;

// eslint-disable-next-line import/prefer-default-export
export const deriveBackgroundColor = (str: string | null): string => {
    const background = str?.replace('#', '');

    if (IS_HEX.test(background)) {
        return `#${background}`;
    }

    if (IS_RGB.test(background)) {
        return background;
    }
    return '';
};
