const ROUND_TO = 10;
const SIZE_INCREMENT = 1000;
const UNITS = ['byte', 'KB', 'MB', 'GB'];

/**
 * Converts a byte count into a scaled value with a unit label (e.g. KB, MB, GB).
 *
 * @param {number} bytes - The number of bytes.
 * @returns {{ count: number, unit: string }} Scaled value and its corresponding unit.
 */
export function getFileSizeParts(bytes: number) {
    let index = 0;

    while (bytes >= SIZE_INCREMENT && index < UNITS.length - 1) {
        bytes /= SIZE_INCREMENT;
        index++;
    }

    const count = Math.round(bytes * ROUND_TO) / ROUND_TO;
    const unit = UNITS[index];

    return { count, unit };
}
