import { AgendaElectKind } from "@/api/bindings/AgendaElectKind";

export function electKindToString(kind: AgendaElectKind): string {
  switch (kind) {
    case "ForOrAgainst":
      return "For Or Against";
    case "StrategyCard":
      return "Strategy Card";
    case "PlanetWithTrait":
      return "Planet";
    case "CulturalPlanet":
      return "Planet";
    case "HazardousPlanet":
      return "Planet";
    case "IndustrialPlanet":
      return "Planet";
    default:
      return kind;
  }
}
