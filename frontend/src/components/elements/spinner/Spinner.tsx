import styles from "./Spinner.module.scss";

export const Spinner = () => {
  return (
    <div>
      <svg
        className={styles.spinnerSvg}
        width="140"
        height="140"
        viewBox="0 0 280 280"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g>
          <line
            x1="59.9833"
            y1="140.333"
            x2="219.978"
            y2="139"
            stroke="#fff"
            stroke-width="4"
          />
          <circle cx="60" cy="140" r="5" fill="#fff" />
          <circle cx="220" cy="139" r="5" fill="#fff" />
        </g>
        <path
          className={styles.circle}
          d="M109.957 122.655L140 105.309L170.043 122.655V157.345L140 174.691L109.957 157.345V122.655Z"
          stroke="#fff"
          stroke-width="4"
        />
        <circle
          className={styles.circle}
          cx="140"
          cy="140"
          r="13"
          stroke="#f5f779"
          stroke-width="4"
        />
        <circle
          className={styles.circle}
          cx="110"
          cy="192"
          r="13"
          stroke="#f7a78f"
          stroke-width="4"
        />
        <circle
          className={styles.circle}
          cx="85"
          cy="232"
          r="8"
          stroke="#82c7c5"
          stroke-width="4"
        />
        <circle
          className={styles.circle}
          cx="170"
          cy="88"
          r="13"
          stroke="#82c7c5"
          stroke-width="4"
        />
        <circle
          className={`${styles.circle} ${styles.circleS}`}
          cx="110"
          cy="192"
          r="5"
          fill="#f7a78f"
        />
        <circle
          className={`${styles.circle} ${styles.circleS}`}
          cx="185"
          cy="61"
          r="5"
          fill="#f5d77b"
        />
      </svg>
    </div>
  );
};
