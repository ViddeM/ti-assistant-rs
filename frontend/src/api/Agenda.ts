export type AgendaKind = "Law" | "Directive";

export type AgendaElectKind =
  | "ForOrAgainst"
  | "Player"
  | "StrategyCard"
  | "Law"
  | "SecretObjective"
  | "Planet"
  | "PlanetWithTrait"
  | "CulturalPlanet"
  | "HazardousPlanet"
  | "IndustrialPlanet";

export interface AgendaElect {
  electKind: AgendaElectKind;
  value: string;
}
