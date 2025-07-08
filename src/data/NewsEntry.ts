export class NewsEntry {
    title: string;
    info: string;
    body: string;
    image_url: string;
    href: string;

    constructor(title: string, info: string, body: string, image_url: string, href: string) {
        this.title = title;
        this.info = info;
        this.body = body;
        this.image_url = image_url;
        this.href = href;
    }
}