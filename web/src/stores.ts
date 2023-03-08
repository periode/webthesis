import { writable } from 'svelte/store'
 import { browser } from '$app/environment';

export const current_heading = writable('')

const theme = browser ? window.localStorage.theme as string : '';
export const dark_mode = writable(theme)