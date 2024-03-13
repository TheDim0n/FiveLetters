'use client'

import { useState } from "react";

import styles from "./page.module.css";


interface Active {
  row: number | undefined,
  col: number | undefined
}


export default function Home() {
  const grid: string[][] = [
    ['q', 'q', 'q', 'q', 'q'],
    ['.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.'],
  ]

  const [gridState, setGridState] = useState<string[][]>(grid);
  const [active, setActive] = useState<Active>({row: undefined, col: undefined});

  const nextActive = (): Active => {
    let row: number = 0;
    let col: number = 0;
    if (active.row === undefined) row = 0;
    if (active.col == undefined) col = 0;
    if (active.row === 5 && col === 4) {
      return active;
    } else if (col < 4) {
      col += 1;
    } else {
      col = 0;
      row += 1;
    }
    return {col: col, row: row}
  }

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let grid = JSON.parse(JSON.stringify(gridState));
    if (active.row !== undefined && active.col != undefined) {
      grid[active.row][active.col] = e.target.value;
      setGridState(grid);
      if (e.target.value.length === 1) setActive(nextActive());
    }
  }

  const getRow = (row: number, word: string[]) => {
    return word.map((char, col) => { return <input
      key={col}
      className={styles.cell}
      style={{
        gridColumn: col+1,
        gridRow: row+1
      }}
      value={char}
      type="text"
      onChange={onChange}
      onFocus={() => setActive({row: row, col: col})}
      maxLength={1}
    />})
  }

  return <div className={styles.grid}>
    {gridState.map((word, row) => getRow(row, word))}
  </div>
}
