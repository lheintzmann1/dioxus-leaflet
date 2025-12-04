window.DioxusLeaflet = class DioxusLeaflet {
    static _maps = new Map();
    static _objects = new Map();
    static _popups = new Map();

    static async registerOnClickHandlerMapAsync(recv, send) {
        let {map_id} = await recv();
        const map = this._maps.get(map_id);
        map.on('click', (e) => {
            send(e.latlng);
        })

    }
}