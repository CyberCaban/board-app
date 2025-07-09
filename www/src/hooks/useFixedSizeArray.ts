import { useState, useCallback } from "react";

export function useFixedSizeArray<T>(maxSize: number, initialValue: T[] = []) {
  const [array, setArray] = useState<T[]>(initialValue);

  const addItem = useCallback(
    (item: T) => {
      setArray((prevArray) => {
        const newArray = [...prevArray, item];
        if (newArray.length > maxSize) {
          return newArray.slice(1);
        }
        return newArray;
      });
    },
    [maxSize]
  );

  const clearArray = useCallback(() => {
    setArray([]);
  }, []);

  return { array, addItem, clearArray };
}
