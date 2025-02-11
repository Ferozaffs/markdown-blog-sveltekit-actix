import type { Load } from '@sveltejs/kit';
import { compile } from 'mdsvex';
import { SERVER_API_URL } from '$env/static/private'

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        let query = SERVER_API_URL.concat("/projectcontent/").concat(params.slug);
        const response = await fetch(query);
        if (response.ok) {
            let data = await response.text();	  
            const compiledResponse = await compile(data);
            return {content: compiledResponse?.code, slug: params.slug };
        }
    }

};
