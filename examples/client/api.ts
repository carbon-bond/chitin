import * as api_trait from './api_trait';
import * as fetch from 'node-fetch';

async function fetchResult(query: Object): Promise<string> {
    const response = await fetch.default('http://localhost:9090/api', {
        body: JSON.stringify(query),
        method: 'POST'
    });
    const text = await response.text();
    return (text);
}

export const api_fetcher = new api_trait.RootQuery(fetchResult);