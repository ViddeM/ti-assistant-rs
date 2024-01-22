export interface NameSortObj {
  name: string;
}

export function nameSort(a: NameSortObj, b: NameSortObj): number {
  return stringSort(a.name, b.name);
}

export function stringSort(a: string, b: string): number {
  return a.toLocaleLowerCase().localeCompare(b.toLocaleLowerCase());
}
