import React from 'react';
import NewsItem from './NewsItem';
import styles from '../css-modules/NewsList.module.css';
import { NewsEntry } from '../data/NewsEntry';

interface NewsListProps {
    newsEntries: NewsEntry[];
}

const NewsList: React.FC<NewsListProps> = ({ newsEntries }) => {
    return (
        <div className={styles.newsList}>
            {newsEntries.map((entry, index) => (
                <NewsItem key={index} entry={entry} />
            ))}
        </div>
    );
};

export default NewsList;