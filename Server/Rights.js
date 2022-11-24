import Queries from "./queries";
export default class Rights {

    constructor() {
    }

    async getRights(userpk) {
        return await Queries.getItems("TUserXRight",null,"FKUser",userpk, null);
    }
}