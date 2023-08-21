import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        const response = await fetch("http://backend:8080/postsummary/".concat(params.slug))
        if (response.ok) {
  		    let data = await response.json();	
            const respone_project = await fetch("http://backend:8080/projectsummary/".concat(data.project_id))
            if (respone_project.ok) {
                let data_project = await respone_project.json();	
                return {post: data, project: data_project};	
            }
		}
    }
};