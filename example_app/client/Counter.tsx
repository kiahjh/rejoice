import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <button
      onClick={() => setCount((c) => c + 1)}
      class="bg-blue-600 px-4 py-2 my-2 text-white rounded-md"
    >
      Count: {count()}
    </button>
  );
}
