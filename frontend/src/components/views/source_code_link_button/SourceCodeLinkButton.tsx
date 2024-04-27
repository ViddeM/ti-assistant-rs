import { faGitAlt } from "@fortawesome/free-brands-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styles from "./SourceCodeLinkButton.module.scss";
import Link from "next/link";

export const SourceCodeLinkButton = () => {
  return (
    <Link
      href="https://radicle.vidarmagnusson.com/nodes/seed.vidarmagnusson.com/rad:zAnrL7P4SoAYGQfhoa4T96zjGKAq"
      className={styles.sourceCodeButton}
      target="_blank"
    >
      <FontAwesomeIcon icon={faGitAlt} />
    </Link>
  );
};
