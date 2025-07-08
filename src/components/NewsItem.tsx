import React from 'react';
import styles from '../css-modules/NewsItem.module.css';
import { NewsEntry } from '../data/NewsEntry';

interface NewsItemProps {
    entry: NewsEntry;
}

const NewsItem: React.FC<NewsItemProps> = ({ entry }) => {
    return (
        <a href={`https://godotengine.org/${entry.href}`} target="_blank" rel="noopener noreferrer">
            <div className={styles.newsItem}>
                <img src={entry.image_url} alt={entry.title} className={styles.image} />
                <div className={styles.content}>
                    <h2 className={styles.title}>{entry.title}</h2>
                    <p className={styles.info}>{entry.info}</p>
                    <p className={styles.body + " " + styles['body-height']}>{entry.body}</p>
                </div>
            </div>
        </a>

    );
};

export default NewsItem;