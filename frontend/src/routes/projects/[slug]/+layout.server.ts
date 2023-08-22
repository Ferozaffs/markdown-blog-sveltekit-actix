import type { Load } from '@sveltejs/kit';
import { SERVER_API_URL } from '$env/static/private'

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        const response = await fetch(SERVER_API_URL.concat("/projectsummary/").concat(params.slug))
        if (response.ok) {
  		    let data = response.json();	
            return data;	
		}
    }
};