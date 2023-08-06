import type { Load } from '@sveltejs/kit';
import { compile } from 'mdsvex';

const MOCK_RESPONSE_FROM_API = `
## Lorem

Lorem is currently extended with the following plugins.
Instructions on how to use them in your application are linked below.
This line is very loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooon.

| Plugin | README |
| ------ | ------ |
| Dropbox | [plugins/dropbox/README.md](Link) |
| Medium | [plugins/medium/README.md](Link) |
| Google Analytics | [plugins/googleanalytics/README.md](Link) |
`;

export const load: Load = async ({ params }) => {
    console.log(params.slug)
   
    const res = MOCK_RESPONSE_FROM_API; // Get data with eg. `fetch`

    const compiledResponse = await compile(res);

    return { content: compiledResponse?.code };
};