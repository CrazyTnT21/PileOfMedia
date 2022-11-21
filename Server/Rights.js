export default class Rights {
    queries;

    constructor(quer) {
        this.queries = quer;
    }

    async getRights(userpk) {
        return await this.queries.getItems("TUserXRight",null,"FKUser",userpk, null);
    }
}