// Translated from CpuDTO struct in /tauri/src-tauri/src
export type Cpu = {
    registerA: number,
    registerX: number,
    registerY: number,
    stackPointer: number,
    programCounter: number,
    status: number,
    memory: number[],
}