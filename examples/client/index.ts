import * as _vorpal from 'vorpal';
import * as api from './api';
import { UserType } from './api_trait';

const vorpal = new _vorpal();

const api_fetcher = new api.ApiFetcher();

function report(promise: Promise<any>): Promise<void> {
    return promise
        .then(resp => {
            this.log(resp);
        })
        .catch(err => {
            this.log(err);
        });
}

vorpal
    .command('articles <count>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askArticles(args.count));
    });

vorpal
    .command('user_articles <user_id> <count>', '看 id 是 <user_id> 的使用者有哪些文章')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askUserArticles(args.count, args.user_id));
    });

vorpal
    .command('user_detail <user_id>', '從 id 查使用者訊息')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askUserDetail(args.user_id));
    });

vorpal
    .command('create_user <name> <sentence>', '創建使用者，伺服器會回傳新創使用者的 id')
    .action(function (args) {
        return report.bind(this)(api_fetcher.createUser({ name: args.name, sentence: args.sentence, ty: UserType.Super }));
    });

vorpal
    .command('post <author_id> <title> <content>', '以 author_id 的身份發文')
    .action(function (args) {
        return report.bind(this)(api_fetcher.postArticle({
            author_id: args.author_id,
            title: args.title,
            content: args.content,
            created_time: null
        }));
    });
vorpal
    .command('post_null', '發一篇沒內容的文')
    .action(function (args) {
        return report.bind(this)(api_fetcher.postArticle(null));
    });
vorpal
    .command('whoami', '查登入狀態')
    .action(function (args) {
        return report.bind(this)(api_fetcher.whoAmI());
    });
vorpal
    .command('login <user_id>', '登入')
    .action(function (args) {
        return report.bind(this)(api_fetcher.login(args.user_id));
    });

vorpal
    .delimiter('$')
    .show();
