import { useEffect, useState } from "react";

export class State<T> {
  private data: T;

  private counter: number = 0;
  private readonly listeners: Map<number, (_: T) => void> = new Map();

  constructor(initial: T) {
    this.data = initial;
  }

  registerListener(listener: (data: T) => void) {
    const newId = ++this.counter;

    this.listeners.set(newId, listener);

    return {
      unregister: () => {
        this.listeners.delete(newId);
      }
    }
  }

  updateValueViaCallback(cb: (curr: T) => T) {
    this.value = cb(this.data);
  }

  set value(data: T) {
    this.data = data;

    this.listeners.forEach((fn) => {
      try {
        fn(this.data);
      } catch (e) {
        console.warn(e);
      }
    });
  }

  get current(): T {
    return this.data;
  }
}

export default function useStateData<T>(state: State<T>): T {
  const [data, updateData] = useState(state.current);

  useEffect(() => {
    const subscription = state.registerListener(updateData);

    const latestValue = state.current;

    if (data !== latestValue) {
      updateData(latestValue);
    }

    return subscription.unregister;
  }, []);

  return data
}