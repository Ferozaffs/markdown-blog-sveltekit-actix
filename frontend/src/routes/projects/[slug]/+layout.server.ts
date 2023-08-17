import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        const response = await fetch("http://localhost:8080/projectsummary/".concat(params.slug))
        if (response.ok) {
  		    let data = response.json();	
            return data;	
		} 	
    }

};