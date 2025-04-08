export interface Publisher {
  id: number;
  name: string;
  description?: string;
}

export interface Region {
  id: number;
  name: string;
  parent_id?: number;
}

export interface Stakeholder {
  id: number;
  name: string;
  type_name: string;
}

export interface Sector {
  id: number;
  name: string;
}

export interface ScenarioListItem {
  id: number;
  title: string;
  type_name: string;
  temperature_target?: string;
  description?: string;
  publisher?: string;
  published_date?: string;
  target_year?: number;
}

export interface ScenarioDetail {
  id: number;
  title: string;
  type_name: string;
  temperature_target?: string;
  description?: string;
  published_date?: string;
  target_year?: number;
  publisher?: Publisher;
  regions: Region[];
  stakeholders: Stakeholder[];
  sectors: Sector[];
}

export interface ScenarioFilters {
  publisher_id?: number;
  region_id?: number;
  stakeholder_id?: number;
  sector_id?: number;
  type_name?: string;
  temperature_target?: string;
  year_from?: number;
  year_to?: number;
}

export interface FilterOptions {
  publishers: Publisher[];
  regions: Region[];
  stakeholders: Stakeholder[];
  sectors: Sector[];
  types: string[];
  temperature_targets: string[];
}