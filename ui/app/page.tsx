'use client'

import { useState, useEffect } from "react";
import classNames from 'classnames';
import { invoke } from '@tauri-apps/api'

import { sessionEmpty } from "./mock";

import styles from "./page.module.css";
import { Attemption, LetterStatus, GameSession } from "./types.d";


export default function Home() {

  const [newAttemption, setNewAttemption] = useState<string>('');
  const [currentSession, setCurrentSession] = useState<GameSession>(sessionEmpty);

  useEffect(() => {
    invoke<GameSession>('get_actual_session')
      .then(result => setCurrentSession(result))
      .catch(console.error)
  }, [])

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (value.length === 1) setNewAttemption(newAttemption + value);
    else if (value.length === 0 && newAttemption.length > 0) {
      setNewAttemption(newAttemption.substring(0, newAttemption.length - 1))
    };
    
  }

  const onSubmit = () => {
    console.log("submit")
  }

  const getAttemption = (attemption: Attemption, index: number, isActive: boolean) => {
    return <div key={index} className={styles.row}>{Array(5).fill(null).map((_, i) => {
      console.log(index);
      const status = attemption.statuses.at(i) || LetterStatus.Undefined;
      let style = undefined;
      if (status == LetterStatus.Undefined) style = styles.undefined;
      else if (status == LetterStatus.NotFound) style = styles.notFound;
      else if (status == LetterStatus.InUncorrectPosition) style = styles.inUncorrectPosition;
      else if (status == LetterStatus.InCorrectPosition) style = styles.inCorrectPosition;
      if (isActive) {
        return <input
          key={i}
          type="text"
          maxLength={1}
          className={classNames(styles.undefined, styles.cell, styles.input)}
          onChange={onChange}
        />
      }
      return <div key={i} className={classNames(style, styles.cell)}>{attemption.word.at(i)}</div>
    })}</div>
  }

  return (<div className={classNames(styles.container)}>
      <div className={styles.grid}>
        {currentSession?.attemptions.map((attemption, i) => getAttemption(attemption, i, i+1 == currentSession.current_attempt))}
      </div>
      <div
        className={classNames(styles.checkBtn, "noselect", newAttemption.length < 5 ? styles.checkBtnDisabled : styles.checkBtnAllowed)}
        onClick={newAttemption.length === 5 ? onSubmit : undefined}
      >Проверить</div>
    </div>)
}
