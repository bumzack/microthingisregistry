import jquery from "jquery";
import {Backend, Frontend} from "./common";

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

window.$ = window.jQuery = jquery;

jquery(document).ready(() => {
    console.log("yo");
    load_backendservers();
    load_frontendservers();
})

const load_backendservers = (): void => {
    // const url = "http://microthingisregistry.bumzack.at/api/backend";
    const url = "http://localhost:3030/api/backend";
    jquery.ajax({url: url}).done(data => {
        const sources = data as Array<Backend>;
        console.log(`backend server ${JSON.stringify(sources, null, 4)} `);
        const b = render_server_backends(sources);
        console.log(`b ${b}`);
        jquery("#server_backend").empty();
        jquery("#server_backend").append("<h4>Backend Services</h4>");
        jquery("#server_backend").append(b);
    });
}


const load_frontendservers = (): void => {
    // const url = "http://microthingisregistry.bumzack.at/api/frontend";
    // const url = "http://localhost:3030/api/backend";
    const url = "http://localhost:3030/api/frontend";

    jquery.ajax({url: url}).done(data => {
        const sources = data as Array<Frontend>;
        console.log(`frontend server ${JSON.stringify(sources, null, 4)} `);
        const b = render_server_frontends(sources);
        console.log(`b ${b}`);
        jquery("#server_frontend").empty();
        jquery("#server_frontend").append("<h4>FrontEnd Services</h4>");
        jquery("#server_frontend").append(b);
    });
}

const render_server_backends = (backends: Array<Backend>): string => {
    let rows = backends.map(b => {
        return render_backend_row(b);
    })
        .join("\n");

    return `
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">service name</th>
                        <th scope="col">URL</th>
                        <th scope="col">OpenAPI client URL</th>
                        <th scope="col">OpenAPI client (first 200 charachters from JSON)</th>
                     </tr>
                </thead>
                <tbody>
                    ${rows}
                </tbody>
            </table>
        </div>
    `;
}


const render_backend_row = (backend: Backend): string => {
    let client = backend.openapiclient?.substring(0, 200);
    return `
        <tr>
            <td>${backend.id}</td>
            <td>${backend.microservice_id}</td>
            <td>${backend.service_url}</td>
            <td>${backend.openapi_url}</td>
            <td>${client}</td>
        </tr>    
    `;
}


const render_server_frontends = (frontends: Array<Frontend>): string => {
    let rows = frontends.map(b => {
        return render_frontend_row(b);
    })
        .join("\n");

    return `
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">service name</th>
                        <th scope="col">URL</th>
                     </tr>
                </thead>
                <tbody>
                    ${rows}
                </tbody>
            </table>
        </div>
    `;
}


const render_frontend_row = (frontend: Frontend): string => {
    return `
        <tr>
            <td>${frontend.id}</td>
            <td>${frontend.microservice_id}</td>
            <td>${frontend.service_url}</td>
        </tr>    
    `;
}


export {};


