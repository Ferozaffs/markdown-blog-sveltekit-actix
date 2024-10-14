import type { Load } from '@sveltejs/kit';
import { SERVER_API_URL } from '$env/static/private'

export const load: Load = async ({ params }) => {
    if (params.slug != undefined) {
        const response = await fetch(SERVER_API_URL.concat("/postsummary/").concat(params.slug))
        if (response.ok) {
            let data = await response.json();	
            try {
                const respone_project = await fetch(SERVER_API_URL.concat("/projectsummary/").concat(data.project_id))
                if (respone_project.ok) {
                    let data_project = await respone_project.json();	
                    return {post: data, project: data_project};	
                } 
            } catch (error) {
                return { post: data, project: null };
            }
		}
    }
};