#!/usr/bin/env -S deno run --allow-net=crates.io,static.crates.io
import { Untar } from "https://deno.land/std/archive/tar.ts";
import { inflate } from "/home/jakob/dev/deno/contrib/compress/mod.ts";
import { basename } from "https://deno.land/std@0.69.0/path/mod.ts";
import { parse as parseToml } from "https://deno.land/std@0.69.0/encoding/toml.ts";

const CRATES_IO = "https://crates.io";
const CRATES_API = `${CRATES_IO}/api/v1`;

const errIfNotOk = (res: Response): Response => {
  if (res.ok) return res;
  else throw new Error(`got status ${res.status}: ${res.statusText}`);
};

interface Dependency {
  id: number;
  crate: string;
  num: string;
  downloads: number;
}
const reverseDependencies = (
  crate: string,
  amount: number,
): Promise<Dependency[]> =>
  fetch(`${CRATES_API}/crates/${crate}/reverse_dependencies?per_page=${amount}`)
    .then(errIfNotOk)
    .then((res) => res.json())
    .then((json) => json["versions"] as Dependency[]);

const downloadCrate = (crate: string): Promise<Untar> =>
  fetch(`${CRATES_API}/crates/${crate}`)
    .then(errIfNotOk)
    .then((res) => res.json())
    .then((json) => {
      const latestVersion = json["crate"]["versions"][0];
      const dlPath = json["versions"]
        .find((v: any) => v["id"] === latestVersion)["dl_path"];

      return fetch(`${CRATES_IO}${dlPath}`)
        .then(errIfNotOk)
        .then(async (res) => {
          const blob = await res.blob();
          const targz = new Uint8Array(await blob.arrayBuffer());
          const tar = inflate(targz);
          const archive = new Untar(new Deno.Buffer(tar));
          return archive;
        });
    });

const findCargoToml = async (
  archive: Untar,
): Promise<Record<string, unknown> | undefined> => {
  for await (const entry of archive) {
    const filename = basename(entry.fileName);
    if (filename !== "Cargo.toml") continue;

    const content = await Deno.readAll(entry);
    const contentString = new TextDecoder().decode(content);

    return parseToml(contentString);
  }
};

async function isProcMacroDependency(crate: string): Promise<boolean> {
  const crateContent = await downloadCrate(crate);

  let cargoToml;
  try {
    cargoToml = await findCargoToml(crateContent) as any;
  } catch (e) {
    return false;
  }

  if (!cargoToml) return false;

  try {
    const isProcMacroCrate = cargoToml["lib"]?.["proc-macro"] ?? false;
    return isProcMacroCrate;
  } catch (e) {
    return false;
  }
}

const filterParallel = <T>(
  values: T[],
  filterFn: (value: T) => Promise<boolean>,
): Promise<T[]> => {
  return Promise.all(values.map(async (value) => {
    const include = await filterFn(value);
    return { value, include };
  }))
    .then((values) =>
      values
        .filter((entry) => entry.include)
        .map((entry) => entry.value)
    );
};

const potentialMacroCrates = await reverseDependencies("quote", 50);
potentialMacroCrates.sort((a, b) => b.downloads - a.downloads);

const procMacroPackages = await filterParallel(
  potentialMacroCrates,
  (info) => isProcMacroDependency(info.crate),
);

for (const crate of procMacroPackages) {
  console.log(`${crate.downloads} ${crate.crate}`);
}
