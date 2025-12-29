import "./App.css";

function App() {
  return <main className="bg-slate-700 flex divide-x-2 divide-black w-screen h-screen">
    <section className="flex-1">
      <div className="flex flex-col m-8 outline-2 outline-slate-600 rounded">
        <h3 className="text-white font-bold w-full p-1">
          Memory
        </h3>
        <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white">

        </div>
      </div>
    </section>
    <section className="flex-1">
      <div className="flex flex-col m-8 outline-2 outline-slate-600 rounded">
        <h3 className="text-white font-bold w-full p-1">
          CPU State
        </h3>
        <div className="bg-slate-600 flex flex-col gap-3 p-2 text-white">
          <h3>Registers:</h3>
          <div className="flex flex-col gap-3 pl-10">
            <div>A: <span className="bg-slate-800 p-1 rounded">Test</span></div>
            <div>X: <span className="bg-slate-800 p-1 rounded">Test</span></div>
            <div>Y: <span className="bg-slate-800 p-1 rounded">Test</span></div>
            <div>PC: <span className="bg-slate-800 p-1 rounded">Test</span></div>
          </div>
          <div>Status Flag: <span className="bg-slate-800 p-1 rounded">Test</span></div>
        </div>
      </div>
    </section>
  </main>
}

export default App;
