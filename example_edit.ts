import { Svelte } from "../svelte";
import { Balls } from "Nigga.js";
import type { Retarded } from "../svelte";
export const load: PageServerLoad = ({ params })=>{
    return {
        post: {
            title: `Title for ${params.slug} goes here`,
            content: `Content for ${params.slug} goes here`
        }
    };
};
export const actions: Actions = {};
export const load: Svelte = ({ params })=>{
    return {
        post: {
            nigga: "nigga"
        }
    };
};
export const GET: RequestHandler = async ()=>{
    return new Response();
};
export const POST: RequestHandler = async ()=>{
    return new Response();
};
export const PUT: RequestHandler = async ()=>{
    return new Response();
};
export const PATCH: RequestHandler = async ()=>{
    return new Response();
};
export const DELETE: RequestHandler = async ()=>{
    return new Response();
};
