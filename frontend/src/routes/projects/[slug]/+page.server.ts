import type { Load } from '@sveltejs/kit';
import { compile } from 'mdsvex';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        let query = "http://127.0.0.1:8080/projectcontent/".concat(params.slug);
        const response = await fetch(query);
        if (response.ok) {
            let data = await response.text();	  
            const compiledResponse = await compile(data);
            return { content: compiledResponse?.code };
        }
    }

};