import type { Load } from '@sveltejs/kit';
import { compile } from 'mdsvex';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        let query = "http://localhost:8080/projectcontent/".concat(params.slug);
        const response = await fetch(query);
        if (response.ok) {
            let data = await response.text();	  
            const compiledResponse = await compile(data);
            return {title: "asdf", status: 0, content: compiledResponse?.code };
        }
    }

};