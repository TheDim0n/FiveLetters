'use client'

import { useState, useEffect } from "react";
import classNames from 'classnames';
import { invoke } from '@tauri-apps/api'

import { sessionEmpty } from "./mock";

import styles from "./page.module.css";
import { Attemption, LetterStatus, GameSession } from "./types.d";


export default function Home() {

  const [wordNotFound, setWordNotFound] = useState<boolean>(false);
  const [needLoad, setNeedLoad] = useState<boolean>(true);
  const [newAttemption, setNewAttemption] = useState<string>(' '.repeat(5));
  const [currentSession, setCurrentSession] = useState<GameSession>(sessionEmpty);

  useEffect(() => {
    invoke<GameSession>('get_actual_session')
      .then(result => setCurrentSession(result))
      .catch(console.error)
  }, [needLoad])

  const onChange = (value: string, index: number) => {
    setWordNotFound(false);
    let word = newAttemption;
    if (value.length === 1) {
      word = newAttemption.substring(0, index) + value + newAttemption.substring(index + 1);
    }
    else if (value.length === 0) {
      word = newAttemption.substring(0, index) + ' ' + newAttemption.substring(index + 1);
      
    };
    setNewAttemption(word);
  }

  const onSubmit = () => {
    invoke('save_attemption', {
      value: newAttemption,
      wordId: currentSession.id,
      number: currentSession.current_attempt
    }).then(() => {
      setWordNotFound(false);
      setNeedLoad(!needLoad);
      setNewAttemption('');
    }).catch(() => setWordNotFound(true));
  }

  const setNext = () => {
    invoke('set_next_session').then(() => {
      setNeedLoad(!needLoad);
    })
  }

  const getAttemption = (attemption: Attemption, index: number, isActive: boolean) => {
    return <div key={index} className={styles.row}>{Array(5).fill(null).map((_, i) => {
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
          autoComplete="off"
          maxLength={1}
          className={classNames(styles.undefined, styles.cell, styles.input)}
          onChange={e => onChange(e.target.value, i)}
        />
      }
      return <div key={i} className={classNames(style, styles.cell)}>{attemption.word.at(i)}</div>
    })}</div>
  }

  return (<div className={classNames(styles.container)}>
      <div className={styles.grid}>
        {currentSession?.attemptions.map((attemption, i) => getAttemption(attemption, i, i+1 == currentSession.current_attempt))}
      </div>
      {!currentSession.completed && currentSession.current_attempt <= 6 && <div
        className={classNames(styles.Btn, "noselect", newAttemption.replaceAll(' ', '').length < 5 ? styles.checkBtnDisabled : styles.checkBtnAllowed)}
        onClick={newAttemption.length === 5 ? onSubmit : undefined}
      >Проверить</div>}
      {(currentSession.completed || currentSession.current_attempt > 6) && <div>
        <div
          className={classNames(styles.Btn, styles.nextBtnAllowed, "noselect")}
          onClick={setNext}
        >Продолжить
        </div>
        {!currentSession.completed && <div className={classNames(styles.answer)}>Ответ: {currentSession.target}</div>}
      </div>}
      {wordNotFound && <div className={classNames(styles.answer)}>Слово не найдено</div>}
    </div>)
}
