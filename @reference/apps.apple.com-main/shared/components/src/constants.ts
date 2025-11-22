// eslint-disable-next-line import/prefer-default-export
export const TEXT_DIRECTION = {
    LTR: 'ltr',
    RTL: 'rtl',
} as const;

// https://www.fileformat.info/info/unicode/char/200e/index.htm
// these are unicode characters in four hexadecimal digits
export const LTR_MARK = '\u200e';
export const RTL_MARK = '\u200f';

export const PLAY_STATES = {
    PLAY: 'play',
    PAUSE: 'pause',
    BUFFER: 'buffer',
    PLAYING: 'playing',
} as const;

// eslint-disable-next-line import/prefer-default-export
export const SEARCH_EVENTS = {
    MAKE_SEARCH_QUERY_FROM_SUGGESTION: 'makeSearchQueryFromSuggestion',
    MAKE_SEARCH_QUERY_FROM_INPUT: 'makeSearchQueryFromInput',
    CLICKED_OUTSIDE_SUGGESTIONS: 'clickedOutsideSuggestions',
    CLICKED_OUTSIDE: 'clickedOutside',
    RESET_SEARCH_INPUT: 'resetSearchInput',
    SUGGESTION_CLICKED: 'suggestionClicked',
    SUGGESTION_FOCUSED: 'suggestionFocused',
    SEARCH_INPUT_HAS_FOCUS: 'searchInputHasFocus',
    MENU_ITEM_CLICK: 'menuItemClick',
    SHOW_SEARCH_SUGGESTIONS: 'showSearchSuggestions',
    CLEAR: 'clear',
} as const;

/**
 * Locations where `SearchInput` component `clear` event can be called from.
 *
 * @remarks
 * clear event can be triggered from two different locations
 * rerturn object provides a way to distinguish between
 * call points.
 *
 */
export enum ClearEventLocation {
    Cancel = 'cancel',
    Input = 'input',
}

export enum PopoverAnchorPositioning {
    Top = 'top',
    Bottom = 'bottom',
    Left = 'left',
    Right = 'right',
}
