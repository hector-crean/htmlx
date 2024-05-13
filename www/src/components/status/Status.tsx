"use client";

import { useStatus } from "@/liveblocks-configs/flow-room.config";
import styles from "./Status.module.css";

export function Status() {
  const status = useStatus();

  return (
    <div className={styles.status} data-status={status}>
      <div className={styles.status_circle} />
      <div className={styles.status_text}>{status}</div>
    </div>
  );
}
