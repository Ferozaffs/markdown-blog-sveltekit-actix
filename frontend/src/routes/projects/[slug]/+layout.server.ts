import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        const response = await fetch("http://127.0.0.1:8080/projectsummary/".concat(params.slug))
        if (response.ok) {
  		    let data = response.json();	
            return data;	
		} 	
    }

};