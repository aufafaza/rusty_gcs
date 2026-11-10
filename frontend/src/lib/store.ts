export interface StateModel {
  system_id: number;
  connected: boolean;
  latitude_deg: number | null;
  longitude_deg: number | null;
  altitude_m: number | null;
}

class State { 
  private store: StateModel | null;

  constructor(initialState: StateModel | null = null){
    this.store = initialState;
  }

  get() : StateModel | null{ 
    return this.store;  
  } 

} 

export const state = new State();
