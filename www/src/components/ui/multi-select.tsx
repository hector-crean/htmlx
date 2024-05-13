"use client";

import { X } from "lucide-react";
import * as React from "react";

import { Badge } from "@/components/ui/badge";

type MultiSelectProps<T extends string> = {
  options: Array<T>;
  selected: Array<T>;
  onSelectedChange: (selected: Array<T>) => void;
};
export function MultiSelect<T extends string>({
  options,
  selected,
  onSelectedChange,
}: MultiSelectProps<T>) {
  const handleUnselect = React.useCallback(
    (option: T) => {
      onSelectedChange(selected.filter((s) => s !== option));
    },
    [selected]
  );

  const handleSelect = React.useCallback(
    (option: T) => {
      onSelectedChange([...selected, option]);
    },
    [selected]
  );

  return (
    <div className="group border border-input px-3 py-2 text-sm ring-offset-background rounded-md focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2">
      <div className="flex gap-1 flex-wrap">
        {options.map((option) => {
          return (
            <Badge
              key={option}
              variant={selected.includes(option) ? "default" : "secondary"}
              onClick={() => handleSelect(option)}
            >
              {option}
              {selected.includes(option) && (
                <button
                  className={`ml-1 ring-offset-background rounded-full outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 `}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      handleUnselect(option);
                    }
                  }}
                  onMouseDown={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                  }}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    handleUnselect(option);
                  }}
                >
                  <X className="h-3 w-3 text-muted-foreground hover:text-foreground" />
                </button>
              )}
            </Badge>
          );
        })}
      </div>
    </div>
  );
}
