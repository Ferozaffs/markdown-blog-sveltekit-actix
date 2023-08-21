import type { Load } from '@sveltejs/kit';
import { compile } from 'mdsvex';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        let query = "http://backend:8080/postcontent/".concat(params.slug);
        const response = await fetch(query);
        if (response.ok) {
            let data = await response.text();	  
            const compiledResponse = await compile(data);
            return {content: compiledResponse?.code };
        }
    }
};
