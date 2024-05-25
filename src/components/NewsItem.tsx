import React from 'react';
import styles from '../css-modules/NewsItem.module.css';
import { NewsEntry } from '../data/NewsEntry';

interface NewsItemProps {
    entry: NewsEntry;
}

const NewsItem: React.FC<NewsItemProps> = ({ entry }) => {
    return (
        <div className={styles.newsItem}>
            <img src={entry.image_url} alt={entry.title} className={styles.image} />
            <div className={styles.content}>
                <h2 className={styles.title}>{entry.title}</h2>
                <p className={styles.info}>{entry.info}</p>
                <p className={styles.body}>{entry.body}</p>
            </div>
        </div>
    );
};

export default NewsItem;