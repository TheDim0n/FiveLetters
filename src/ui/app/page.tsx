'use client'

import { useState } from "react";
import classNames from 'classnames';

import { sessionCompleted } from "./mock";

import styles from "./page.module.css";
import { Attemption, LetterStatus } from "./types.d";


export default function Home() {

  const getAttemption = (attemption: Attemption, key: number) => {
    return <div key={key} className={styles.row}>{Array(5).fill(null).map((_, i) => {
      const status = attemption.statuses.at(i) || LetterStatus.Undefined;
      let style = undefined;
      if (status == LetterStatus.Undefined) style = styles.undefined;
      else if (status == LetterStatus.NotFound) style = styles.notFound;
      else if (status == LetterStatus.InUncorrectPosition) style = styles.inUncorrectPosition;
      else if (status == LetterStatus.InCorrectPosition) style = styles.inCorrectPosition;
      return <div key={i} className={classNames(style, styles.cell)}>{attemption.word.at(i)}</div>
    })}</div>
  }

  return <div className={styles.grid}>
    {sessionCompleted.attemptions.map((attemption, i) => getAttemption(attemption, i))}
  </div>
}
