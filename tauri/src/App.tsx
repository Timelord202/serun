import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Cpu } from "./types/cpu";
import "./App.css";

function App() {
  const [cpuState, setCpuState] = useState<Cpu | null>(null);

  return (
    <main className="bg-slate-700 flex divide-x-2 divide-black w-screen h-screen">
      <section className="flex-1">
        <div className="flex flex-col m-8 outline-2 outline-slate-600 rounded">
          <h3 className="text-white font-bold w-full p-1">Memory</h3>
          <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white"></div>
        </div>
      </section>
      <section className="flex-1">
        <div className="flex flex-col m-8 outline-2 outline-slate-600 rounded">
          <h3 className="text-white font-bold w-full p-1">CPU State</h3>
          <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white">
            <h3>Registers:</h3>
            <div className="flex flex-col gap-3 pl-10">
              <div>
                A:{" "}
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerA}
                </span>
              </div>
              <div>
                X:{" "}
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerX}
                </span>
              </div>
              <div>
                Y:{" "}
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.registerY}
                </span>
              </div>
              <div>
                PC:{" "}
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.programCounter}
                </span>
              </div>
              <div>
                SP:{" "}
                <span className="bg-slate-800 p-1 rounded">
                  {cpuState?.stackPointer}
                </span>
              </div>
            </div>
            <div>
              Status Flag:{" "}
              <span className="bg-slate-800 p-1 rounded">
                {cpuState?.status?.toString(2)}
              </span>
            </div>
          </div>
        </div>
        <button
          className="text-white ml-8"
          onClick={async () => {
            setCpuState(await invoke("execute_instruction"));
          }}
        >
          Execute Instruction
        </button>
      </section>
    </main>
  );
}

export default App;
