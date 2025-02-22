export interface paths {
    "/accounts/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["LoginData"];
                };
            };
            responses: {
                /** @description Returned JWT. Valid for a week */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["LoginReturnData"];
                    };
                };
                401: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/accounts/refresh": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned JWT. Valid for an hour */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                403: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/accounts/register": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "multipart/form-data": components["schemas"]["CreateAccount"];
                };
            };
            responses: {
                /** @description Returned JWT and user. Valid for a week */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["LoginReturnData"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned books */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["BooksTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "multipart/form-data": components["schemas"]["CreateBook"];
                };
            };
            responses: {
                /** @description Book successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Book"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/slug/{slug}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Slug of the item to search for */
                    slug: components["schemas"]["Slug"];
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned book based on the slug */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Book"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/title/{title}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Title of the item to search for */
                    title: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned books based on the title */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["BooksTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned book based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Book"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Book successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/characters": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned characters based on the book id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["BookCharacter"][];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/characters/{character_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    character_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Character association successfully added */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    character_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Character association successfully removed */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/genres": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned genres based on the book id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["GenresTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/genres/{genre_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    genre_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Genre association successfully added */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    genre_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Genre association successfully removed */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/involved": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned people involved based on the book id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Involved"][];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/involved/{person_id}/{role_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    person_id: number;
                    role_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Involved association successfully added */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    person_id: number;
                    role_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Involved association successfully removed */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/statistic": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned book statistic based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["BookStatistic"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/themes": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned themes based on the book id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["ThemesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/books/{id}/themes/{theme_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    theme_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Theme association successfully added */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    theme_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Theme association successfully removed */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/characters": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned characters */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["CharactersTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "multipart/form-data": components["schemas"]["CreateCharacter"];
                };
            };
            responses: {
                /** @description Character successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Character"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/characters/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned characters based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["CharactersTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/characters/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned character based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Character"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Character successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/franchises": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned franchises */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["FranchisesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["CreateFranchise"];
                };
            };
            responses: {
                /** @description Franchise successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Franchise"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/franchises/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned franchises based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["FranchisesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/franchises/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned franchise based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Franchise"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Franchise successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/genres": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned genres */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["GenresTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["CreateGenre"];
                };
            };
            responses: {
                /** @description Genre successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Genre"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/genres/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned genres based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["GenresTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/genres/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned genre based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Genre"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Genre successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/people": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned people */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["PeopleTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "multipart/form-data": components["schemas"]["CreatePerson"];
                };
            };
            responses: {
                /** @description Person successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Person"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/people/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned people based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["PeopleTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/people/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned person based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Person"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Person successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/roles": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned roles */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["RolesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["CreateRole"];
                };
            };
            responses: {
                /** @description Role successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Role"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/roles/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned roles based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["RolesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/roles/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned role based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Role"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Role successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/themes": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned themes */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["ThemesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["CreateTheme"];
                };
            };
            responses: {
                /** @description Theme successfully created */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Theme"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/themes/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned themes based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["ThemesTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/themes/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned theme based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["Theme"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item to delete */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Theme successfully deleted */
                204: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned users */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["UsersTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/name/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: {
                    /** @description The current page */
                    page?: number | null;
                    /** @description The amount of items to query */
                    count?: number | null;
                };
                header?: never;
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned users based on the name */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["UsersTotal"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/username/{username}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Name of the item to search for */
                    name: string;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned user based on the username */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["User"] | null;
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Returned user based on the id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["User"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}/books": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description User books based on the user id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["UserBook"][];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                };
                cookie?: never;
            };
            requestBody: {
                content: {
                    "application/json": components["schemas"]["CreateUserBook"];
                };
            };
            responses: {
                /** @description Book association successfully added */
                201: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["UserBook"];
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                403: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}/books/{book_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: {
                    /** @description The language of the items */
                    "Accept-Language"?: string | null;
                };
                path: {
                    /** @description Id of the item */
                    id: number;
                    book_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description User book based on the user id and book id */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "application/json": components["schemas"]["UserBook"] | null;
                    };
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                404: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete: {
            parameters: {
                query?: never;
                header?: never;
                path: {
                    /** @description Id of the item */
                    id: number;
                    book_id: number;
                };
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                /** @description Book association successfully removed */
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content?: never;
                };
                400: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                403: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
                500: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/plain": string;
                    };
                };
            };
        };
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        Book: {
            characters: components["schemas"]["BookCharacter"][];
            franchise?: components["schemas"]["Franchise"] | null;
            genres: components["schemas"]["Genre"][];
            /** Format: int32 */
            id: number;
            involved: components["schemas"]["Involved"][];
            /** Format: date */
            published?: `${number}-${number}-${number}` | null;
            slug: components["schemas"]["Slug"];
            statistic: components["schemas"]["BookStatistic"];
            themes: components["schemas"]["Theme"][];
            translations: components["schemas"]["BookAvailableTranslations"];
        };
        BookAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["BookTranslation"];
            };
        };
        BookCharacter: {
            character: components["schemas"]["Character"];
        };
        BookStatistic: {
            /** Format: date */
            added: `${number}-${number}-${number}`;
            /** Format: int32 */
            favorites: number;
            /** Format: int32 */
            members: number;
            /** Format: int32 */
            popularity: number;
            /** Format: int32 */
            rank: number;
            rating: components["schemas"]["Rating"];
        };
        BookTranslation: {
            cover: components["schemas"]["Image"];
            description?: string | null;
            title: string;
        };
        BooksTotal: {
            items: components["schemas"]["Book"][];
            total: number;
        };
        Character: {
            /** Format: date */
            birthday?: `${number}-${number}-${number}` | null;
            /** Format: int32 */
            height_cm?: number | null;
            /** Format: int32 */
            id: number;
            image?: components["schemas"]["Image"] | null;
            translations: components["schemas"]["CharacterAvailableTranslations"];
        };
        CharacterAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["CharacterTranslation"];
            };
        };
        CharacterTranslation: {
            description?: string | null;
            first_name?: string | null;
            last_name?: string | null;
            name: string;
        };
        CharactersTotal: {
            items: components["schemas"]["Character"][];
            total: number;
        };
        CreateAccount: {
            account: components["schemas"]["CreateAccountData"];
            profile_picture?: components["schemas"]["CreateImage"] | null;
        };
        CreateAccountData: {
            email: components["schemas"]["Email"];
            password: components["schemas"]["Password"];
            user: components["schemas"]["CreateUserData"];
        };
        CreateBook: {
            book: components["schemas"]["CreateBookData"];
            covers: components["schemas"]["CreateImage"][];
        };
        CreateBookData: {
            characters?: number[] | null;
            /** Format: int32 */
            franchise?: number | null;
            genres?: number[] | null;
            involved?: components["schemas"]["InvolvedId"][] | null;
            /** Format: date */
            published?: `${number}-${number}-${number}` | null;
            slug: components["schemas"]["Slug"];
            themes?: number[] | null;
            translations: {
                [key: string]: components["schemas"]["CreateBookTranslation"];
            };
        };
        CreateBookTranslation: {
            cover: components["schemas"]["CreateCover"];
            description?: string | null;
            title: string;
        };
        CreateCharacter: {
            character: components["schemas"]["CreateCharacterData"];
            image?: components["schemas"]["CreateImage"] | null;
        };
        CreateCharacterData: {
            /** Format: date */
            birthday?: `${number}-${number}-${number}` | null;
            /** Format: int32 */
            height_cm?: number | null;
            translations: {
                [key: string]: components["schemas"]["CreateCharacterTranslation"];
            };
        };
        CreateCharacterTranslation: {
            description?: string | null;
            first_name?: string | null;
            last_name?: string | null;
            name: string;
        };
        CreateCover: {
            ImageIndex: number;
        } | {
            ReuseFromLanguage: components["schemas"]["Language"];
        };
        CreateFranchise: {
            translations: {
                [key: string]: components["schemas"]["CreateFranchiseTranslation"];
            };
        };
        CreateFranchiseTranslation: {
            name: string;
        };
        CreateGenre: {
            translations: {
                [key: string]: components["schemas"]["CreateGenreTranslation"];
            };
        };
        CreateGenreTranslation: {
            name: string;
        };
        /** Format: binary */
        CreateImage: Blob;
        CreatePerson: {
            image?: components["schemas"]["CreateImage"] | null;
            person: components["schemas"]["CreatePersonData"];
        };
        CreatePersonData: {
            /** Format: date */
            birthday?: `${number}-${number}-${number}` | null;
            first_name?: string | null;
            /** Format: int32 */
            height_cm?: number | null;
            last_name?: string | null;
            name: string;
            translations: {
                [key: string]: components["schemas"]["CreatePersonTranslation"];
            };
        };
        CreatePersonTranslation: {
            description?: string | null;
        };
        CreateRole: {
            translations: {
                [key: string]: components["schemas"]["CreateRoleTranslation"];
            };
        };
        CreateRoleTranslation: {
            name: string;
        };
        CreateTheme: {
            translations: {
                [key: string]: components["schemas"]["CreateThemeTranslation"];
            };
        };
        CreateThemeTranslation: {
            name: string;
        };
        CreateUser: {
            profile_picture?: components["schemas"]["CreateImage"] | null;
            user: components["schemas"]["CreateUserData"];
        };
        CreateUserBook: {
            /** Format: int32 */
            book_id: number;
            /** Format: int32 */
            chapters?: number | null;
            favorite: boolean;
            /** Format: date */
            finished?: `${number}-${number}-${number}` | null;
            /** Format: int32 */
            pages?: number | null;
            review?: string | null;
            score?: components["schemas"]["Score"] | null;
            /** Format: date */
            start?: `${number}-${number}-${number}` | null;
            status: components["schemas"]["UserStatus"];
        };
        CreateUserData: {
            description?: string | null;
            name: string;
        };
        Email: string;
        Franchise: {
            /** Format: int32 */
            id: number;
            translations: components["schemas"]["FranchiseAvailableTranslations"];
        };
        FranchiseAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["FranchiseTranslation"];
            };
        };
        FranchiseTranslation: {
            name: string;
        };
        FranchisesTotal: {
            items: components["schemas"]["Franchise"][];
            total: number;
        };
        Genre: {
            /** Format: int32 */
            id: number;
            translations: components["schemas"]["GenreAvailableTranslations"];
        };
        GenreAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["GenreTranslation"];
            };
        };
        GenreTranslation: {
            name: string;
        };
        GenresTotal: {
            items: components["schemas"]["Genre"][];
            total: number;
        };
        Image: {
            /** Format: int32 */
            id: number;
            versions: components["schemas"]["ImageData"][];
        };
        ImageData: {
            /** Format: int32 */
            height: number;
            uri: string;
            /** Format: int32 */
            width: number;
        };
        Involved: {
            person: components["schemas"]["Person"];
            roles: components["schemas"]["Role"][];
        };
        InvolvedId: {
            /** Format: int32 */
            person_id: number;
            /** Format: int32 */
            role_id: number;
        };
        /** @enum {string} */
        Language: "EN" | "DE" | "JA" | "ES" | "DA" | "NL" | "KO";
        LoginData: {
            email: string;
            password: string;
        };
        LoginReturnData: {
            token: string;
            user: components["schemas"]["User"];
        };
        Password: string;
        PeopleTotal: {
            items: components["schemas"]["Person"][];
            total: number;
        };
        Person: {
            /** Format: date */
            birthday?: `${number}-${number}-${number}` | null;
            first_name?: string | null;
            /** Format: int32 */
            height_cm?: number | null;
            /** Format: int32 */
            id: number;
            image?: components["schemas"]["Image"] | null;
            last_name?: string | null;
            name: string;
            translations: components["schemas"]["PersonAvailableTranslations"];
        };
        PersonAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["PersonTranslation"];
            };
        };
        PersonTranslation: {
            description?: string | null;
        };
        Rating: {
            /** Format: int32 */
            amount: number;
            /** Format: int32 */
            id: number;
            /** Format: float */
            score?: number | null;
        };
        Role: {
            /** Format: int32 */
            id: number;
            translations: components["schemas"]["RoleAvailableTranslations"];
        };
        RoleAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["RoleTranslation"];
            };
        };
        RoleTranslation: {
            name: string;
        };
        RolesTotal: {
            items: components["schemas"]["Role"][];
            total: number;
        };
        Score: number;
        Slug: string;
        Theme: {
            /** Format: int32 */
            id: number;
            translations: components["schemas"]["ThemeAvailableTranslations"];
        };
        ThemeAvailableTranslations: {
            available_languages: components["schemas"]["Language"][];
            translations: {
                [key: string]: components["schemas"]["ThemeTranslation"];
            };
        };
        ThemeTranslation: {
            name: string;
        };
        ThemesTotal: {
            items: components["schemas"]["Theme"][];
            total: number;
        };
        User: {
            deleted: boolean;
            description?: string | null;
            /** Format: int32 */
            id: number;
            /** Format: date */
            joined: `${number}-${number}-${number}`;
            name: string;
            profile_picture?: components["schemas"]["Image"] | null;
        };
        UserBook: {
            book: components["schemas"]["Book"];
            /** Format: int32 */
            chapters?: number | null;
            favorite: boolean;
            /** Format: date */
            finished?: `${number}-${number}-${number}` | null;
            /** Format: int32 */
            pages?: number | null;
            review?: string | null;
            score?: components["schemas"]["Score"] | null;
            /** Format: date */
            start?: `${number}-${number}-${number}` | null;
            status: components["schemas"]["UserStatus"];
        };
        /** @enum {string} */
        UserStatus: "NotStarted" | "Ongoing" | "Finished" | "Paused";
        UsersTotal: {
            items: components["schemas"]["User"][];
            total: number;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export type operations = Record<string, never>;
