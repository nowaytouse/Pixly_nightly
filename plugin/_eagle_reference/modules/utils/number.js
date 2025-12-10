module.exports = class {
    // 格式化數字
    static format = (size, digits = 1) => {
        const sizes = [
            i18next.t('units.fileSize.bytes'),
            i18next.t('units.fileSize.kilobytes'),
            i18next.t('units.fileSize.megabytes'),
            i18next.t('units.fileSize.gigabytes'),
            i18next.t('units.fileSize.terabytes'),
            i18next.t('units.fileSize.petabytes'),
            i18next.t('units.fileSize.exabytes'),
            i18next.t('units.fileSize.zettabytes'),
            i18next.t('units.fileSize.yottabytes')
        ];
        let i = 0;

        const divisor = eagle.app.isMac ? 1000 : 1024;

        for (let j = 0; j < 2; j++) {
            if (size >= divisor) {
                size /= divisor;
                i++;
            }
        }

        const digit = size.toLocaleString('en-US', { maximumFractionDigits: digits });

        return `${digit} ${sizes[i]}`;
    };
};
