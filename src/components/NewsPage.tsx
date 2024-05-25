import { NewsEntry } from "../data/NewsEntry";
import NewsList from "./NewsList";


interface NewsPageProps {
    newsEntries: NewsEntry[]
}

function NewsPage(props: NewsPageProps) {

    return (
        <div>
            <h1>News</h1>
            <NewsList newsEntries={props.newsEntries} />
        </div>
    );

}

export default NewsPage;

