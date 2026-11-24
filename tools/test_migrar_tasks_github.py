"""Teste offline do `aplicar` de migrar-tasks-github.py: GitHub falso em memória.

Prova a idempotência (AGENTS.md §3): segunda rodada não duplica Issue e não apaga
andamento gravado depois da primeira; rodada interrompida no meio termina na seguinte.

Uso: python -I tools/test_migrar_tasks_github.py
"""
from __future__ import annotations

import importlib.util
import itertools
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "migrar", Path(__file__).with_name("migrar-tasks-github.py"))
migrar = importlib.util.module_from_spec(spec)
spec.loader.exec_module(migrar)


class GitHubFalso:
    def __init__(self, falhar_na_issue: int | None = None, falhar_no_fechamento: bool = False):
        self.falhar_no_fechamento = falhar_no_fechamento
        self.ids = itertools.count(1)
        self.labels: set[str] = set()
        self.issues: list[dict] = []
        self.project: dict | None = None
        self.fields: dict[str, dict] = {}
        self.items: dict[str, dict] = {}  # issue node_id -> {"id", "valores": {campo: valor}}
        self.falhar_na_issue = falhar_na_issue

    def gh(self, args, body=None):
        if args[:2] == ["api", "--paginate"]:
            path = args[-1]
            if "/labels" in path:
                return [[{"name": n} for n in self.labels]]
            if "/issues" in path:
                filtro = "labels=task" in path
                return [[i for i in self.issues if not filtro or "task" in i["labels"]]]
        if args[:3] == ["api", "-X", "POST"] and args[3].endswith("/labels"):
            self.labels.add(body["name"])
            return {}
        if args[:3] == ["api", "-X", "POST"] and args[3].endswith("/issues"):
            if self.falhar_na_issue is not None and len(self.issues) == self.falhar_na_issue:
                raise RuntimeError("falha simulada")
            n = next(self.ids)
            issue = {"number": n, "node_id": f"I{n}", "title": body["title"], "state": "open",
                     "labels": list(body["labels"])}
            self.issues.append(issue)
            return issue
        if args[:3] == ["api", "-X", "PATCH"]:
            if self.falhar_no_fechamento:
                raise RuntimeError("falha simulada no fechamento")
            num = int(args[3].rsplit("/", 1)[1])
            next(i for i in self.issues if i["number"] == num)["state"] = body["state"]
            return {}
        raise AssertionError(f"chamada inesperada: {args}")

    def gql(self, query, **v):
        if "projectsV2(first" in query:
            nodes = [self.project] if self.project else []
            return {"user": {"id": "U1", "projectsV2": {"nodes": nodes}}}
        if "createProjectV2(" in query:
            self.project = {"id": "P1", "title": v["t"]}
            return {"createProjectV2": {"projectV2": self.project}}
        if "createProjectV2Field" in query:
            i = v["i"]
            f = {"id": f"F{next(self.ids)}", "name": i["name"]}
            if i["dataType"] == "SINGLE_SELECT":
                f["options"] = [{"id": f"O{o['name']}", "name": o["name"]} for o in i["singleSelectOptions"]]
            self.fields[i["name"]] = f
            return {}
        if "fields(first" in query:
            return {"node": {"fields": {"nodes": list(self.fields.values())}}}
        if "addProjectV2ItemById" in query:
            item = self.items.setdefault(v["c"], {"id": f"IT{v['c']}", "valores": {}})
            return {"addProjectV2ItemById": {"item": {"id": item["id"]}}}
        if "fieldValues(first" in query:
            item = next(x for x in self.items.values() if x["id"] == v["i"])
            return {"node": {"fieldValues": {"nodes": [
                {"field": {"name": n}} for n in item["valores"]]}}}
        if "updateProjectV2ItemFieldValue" in query:
            item = next(x for x in self.items.values() if x["id"] == v["i"])
            nome = next(n for n, f in self.fields.items() if f["id"] == v["f"])
            item["valores"][nome] = next(iter(v["v"].values()))
            return {}
        raise AssertionError(f"query inesperada: {query[:60]}")

    def estado(self, tid):
        issue = next(i for i in self.issues if i["title"].startswith(tid))
        return self.items[issue["node_id"]]["valores"]


def plano():
    def p(tid, estado, fechar=False):
        return {"id": tid, "titulo": f"{tid} — x", "rotulos": ["task", "fase:M1"], "corpo": "c",
                "campos": {"Estado": estado, "Onda": 1, "Voltas": 0, "Branch": None}, "fechar": fechar}
    return [p("TASK-E01-01", "Pronta"), p("TASK-E01-02", "Integrada", fechar=True), p("TASK-E01-03", "Rascunho")]


class AplicarIdempotente(unittest.TestCase):
    def rodar(self, falso):
        migrar.gh, migrar.gql = falso.gh, falso.gql
        migrar.aplicar(plano())

    def test_primeira_rodada_cria_e_preenche(self):
        f = GitHubFalso()
        self.rodar(f)
        self.assertEqual(len(f.issues), 3)
        self.assertEqual(f.estado("TASK-E01-01")["Estado"], "OPronta")
        self.assertEqual(f.estado("TASK-E01-01")["Onda"], 1.0)
        self.assertEqual([i["state"] for i in f.issues], ["open", "closed", "open"])

    def test_segunda_rodada_nao_duplica_nem_apaga_andamento(self):
        f = GitHubFalso()
        self.rodar(f)
        f.estado("TASK-E01-01").update(Estado="OEm implementação", Voltas=2.0)
        reaberta = next(i for i in f.issues if i["title"].startswith("TASK-E01-02"))
        reaberta["state"] = "open"  # mantenedor reabriu: a migração não fecha de novo
        self.rodar(f)
        self.assertEqual(len(f.issues), 3)
        self.assertEqual(f.estado("TASK-E01-01")["Estado"], "OEm implementação")
        self.assertEqual(f.estado("TASK-E01-01")["Voltas"], 2.0)
        self.assertEqual(reaberta["state"], "open")

    def test_issue_sem_rotulo_e_achada_pelo_titulo(self):
        f = GitHubFalso()
        self.rodar(f)
        for i in f.issues:
            i["labels"] = []
        self.rodar(f)
        self.assertEqual(len(f.issues), 3)

    def test_rodada_interrompida_termina_na_seguinte(self):
        f = GitHubFalso(falhar_na_issue=1)
        with self.assertRaises(RuntimeError):
            self.rodar(f)
        self.assertEqual(len(f.issues), 1)
        f.falhar_na_issue = None
        self.rodar(f)
        self.assertEqual(sorted(i["title"][:11] for i in f.issues),
                         ["TASK-E01-01", "TASK-E01-02", "TASK-E01-03"])
        self.assertEqual(f.estado("TASK-E01-03")["Estado"], "ORascunho")

    def test_queda_no_fechamento_termina_na_seguinte(self):
        f = GitHubFalso(falhar_no_fechamento=True)
        with self.assertRaises(RuntimeError):
            self.rodar(f)
        f.falhar_no_fechamento = False
        self.rodar(f)
        integrada = next(i for i in f.issues if i["title"].startswith("TASK-E01-02"))
        self.assertEqual(integrada["state"], "closed")
        self.assertEqual(f.estado("TASK-E01-02")["Estado"], "OIntegrada")


if __name__ == "__main__":
    unittest.main()
