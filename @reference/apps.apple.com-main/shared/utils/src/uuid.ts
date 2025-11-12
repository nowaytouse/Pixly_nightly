/**
 * Generate a variant 1 UUIDv4.
 *
 * @return the UUID
 */
export function generateUuid(): string {
    return 'xxxxxxxx-xxxx-4xxx-Vxxx-xxxxxxxxxxxx'.replace(
        /[xV]/g,
        (placeholder) => {
            let nibble = (Math.random() * 16) | 0;

            if (placeholder === 'V') {
                // Per RFC, the two MSB of byte 8 must be 0b10 (0x8).
                // 0x3 (0b11) masks out the bottom two bits.
                // See: https://tools.ietf.org/html/rfc4122.html#section-4.1.1
                nibble = (nibble & 0x3) | 0x8;
            }

            return nibble.toString(16);
        },
    );
}
