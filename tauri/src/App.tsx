import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Cpu } from "./types/cpu";
import { VscDebugStart } from "react-icons/vsc";
import { VscDebugPause } from "react-icons/vsc";
import { VscDebugStepOver } from "react-icons/vsc";
import "./App.css";

const MAX_MEM_ADDR = 0xffff;

function getDisplayMemory(
  cpu: Cpu | null,
  currAddr: number | undefined,
  memDistance: number = 5
) {
  if (cpu !== null && currAddr !== undefined) {
    const start = Math.max(0, currAddr - memDistance);
    const end = Math.min(MAX_MEM_ADDR, currAddr + memDistance);
    return cpu.memory.slice(start, end);
  }
  return [];
}

function convertToHex(num: number | undefined) {
  if (num !== undefined) return `0x${num.toString(16).toUpperCase()}`;
  return null;
}

function App() {
  const [cpuState, setCpuState] = useState<Cpu | null>(null);
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const displayMemory = getDisplayMemory(cpuState, cpuState?.programCounter);
  const debugPlayChanged = () => setIsPlaying((prev) => !prev);

  // Initialize the CPU
  useEffect(() => {
    const initializeCpu = async () => {
      const state: Cpu = await invoke("get_cpu_state");
      setCpuState(state);
    }
    initializeCpu();
  }, []);

  return (
    <main className="bg-slate-700 flex divide-x-2 divide-black w-screen h-screen">
      <section className="flex-1">
        <div className="flex flex-col m-8 outline-2 outline-slate-600 rounded">
          <h3 className="text-white font-bold w-full p-1">Memory</h3>
          <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white">
            {displayMemory.map((addr) => {
              return (
                <p>
                  {convertToHex(addr)}: {convertToHex(cpuState?.memory[addr])}
                </p>
              );
            })}
          </div>
        </div>
      </section>
      <section className="flex-1">
        <div className="flex gap-2 m-8 mb-0 p-2 outline-2 outline-slate-600 rounded">
          {isPlaying ? (
            <VscDebugPause size={20} onClick={debugPlayChanged} className="text-white" />
          ) : (
            <VscDebugStart size={20} onClick={debugPlayChanged} className="text-white" />
          )}
          <VscDebugStepOver onClick={async () => setCpuState(await invoke("execute_instruction"))} className="text-white" />
        </div>
        <div className="flex flex-col m-8 mt-4 outline-2 outline-slate-600 rounded">
          <h3 className="text-white font-bold w-full p-1">CPU State</h3>
          <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white">
            <h3>Registers:</h3>
            <div className="flex flex-col gap-3 pl-10">
              <div>
                A:
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerA}
                </span>
              </div>
              <div>
                X:
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerX}
                </span>
              </div>
              <div>
                Y:
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerY}
                </span>
              </div>
              <div>
                PC:
                <span className="bg-slate-800 p-1 rounded">
                  {convertToHex(cpuState?.programCounter)}
                </span>
              </div>
              <div>
                SP:
                <span className="bg-slate-800 p-1 rounded">
                  {convertToHex(cpuState?.stackPointer)}
                </span>
              </div>
            </div>
            <div>
              Status Flag:
              <span className="bg-slate-800 p-1 rounded">
                {cpuState?.status?.toString(2)}
              </span>
            </div>
          </div>
        </div>
      </section>
    </main>
  );
}

export default App;
