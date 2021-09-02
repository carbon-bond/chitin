export type Option<T> = T | undefined | null;
export type Result<T, E> = {
    'Ok': T
} | {
    'Err': E
};
export type Article = {     author_id: number; title: string; content: string; created_time:     string| null };
export enum UserType { Super = "Super", Nobody = "Nobody" };
export type User = { name: string; sentence: string; ty: UserType };
export type Test = { test: string };
export type Test2 = { test: string };
type Fetcher = (query: Object) => Promise<string>;
export abstract class RootQuery {
    fetchResult: Fetcher;
    UserQuery: UserQuery;
    constructor(fetcher: Fetcher) {
        this.fetchResult = fetcher;
        // 其他 router
        this.UserQuery = new UserQuery(fetcher);
    }
    async askArticles(count: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "AskArticles": { count } }));
    }
    async postArticle(article: Option<Article>): Promise<Result<null, string>> {
        return JSON.parse(await this.fetchResult({ "PostArticle": { article } }));
    }
    async createUser(user: User): Promise<Result<number, string>> {
        return JSON.parse(await this.fetchResult({ "CreateUser": { user } }));
    }
    async test(test: Test): Promise<Result<Test, string>> {
        return JSON.parse(await this.fetchResult({ "Test": { test } }));
    }
}

class UserQuery {
    fetchResult: Fetcher;
    UserDetailQuery: UserDetailQuery;
    constructor(fetcher: Fetcher) {
        this.fetchResult = fetcher;
        this.UserDetailQuery = new UserDetailQuery(fetcher);
    }
    async askUserArticles(user_id: number, count: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "AskUserArticles": { user_id, count } } }));
    }
    async login(user_id: number): Promise<Result<null, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "Login": { user_id } } }));
    }
    async whoAmI(): Promise<Result<number, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "WhoAmI": {  } } }));
    }
}

class UserDetailQuery {
    fetchResult: Fetcher;
    constructor(fetcher: Fetcher) {
        this.fetchResult = fetcher;
    }
    async askUserDetail(user_id: number): Promise<Result<Option<User>, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "UserDetail": { "AskUserDetail": { user_id } } } }));
    }
}