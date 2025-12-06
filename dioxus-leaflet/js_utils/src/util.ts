import type { L } from "./types";

export async function setup(): Promise<typeof L> {
    let l = (window as any).L as typeof L;
    while (!l) {
        await wait(100);
        l = (window as any).L as typeof L;
    }
    return l;
}

export function wait(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
}