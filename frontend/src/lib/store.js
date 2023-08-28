import { writable } from "svelte/store";

export const currentContent = writable("posts");
export const currentTags = writable([]);