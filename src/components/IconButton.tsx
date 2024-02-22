import React from "react";
import styles from "../css-modules/IconButton.module.css"

interface IconButtonPropsProps {
    icon: React.ReactNode;
    onClick: () => void;
}

function IconButtonProps(props: IconButtonPropsProps) {

    return (
        <button className={styles.noButton} onClick={() => props.onClick()}>
            {props.icon}
        </button >
    );

}

export default IconButtonProps;