import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <button
      onClick={() => setCount((c) => c + 1)}
      class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
    >
      Count: {count()}
    </button>
  );
}
