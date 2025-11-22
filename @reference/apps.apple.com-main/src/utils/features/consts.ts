/**
 * This file is a place for defining any Feature Flag Constants used throughout the app
 * In order to keep the API Interface same for Build Time vs Runtime Feature Flags
 * We have ensured that all the flags have to be defined in this file
 */
// Actual Feature Flag Values have to be defined in the /apps/app-store/featureFlags.external.cjs
// BUILD BASED FEATURE FLAGS DUMMY FLAG DEFINITIONS TO FIX THE NAME OF THE FEATURE FLAGS TO BE USED
// Values of the BUILD BASED FLAGS will decide if they are evaluated to true in DEV mode
export const __FF_SHOW_RADAR = 'r01234e98765';

export const __FF_SHOW_LOC_KEYS = 'ffShowLocKeys';

export const __FF_ARYA = 'asha123e7z124';
