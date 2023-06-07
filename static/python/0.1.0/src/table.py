from typing import List, Union, Tuple, Dict

from backends.factory import makeBackend
from backends.interface import Backend
from dataset import Dataset
from fieldExpr import BooleanFieldExpr, FieldExpr

Direction = Union[Literal["ASC"], Literal["DESC"]]
Ordering = Tuple[FieldExpr, Direction]
Selector = Union[str, FieldExpr]


class Table:
    def __init__(
        self,
        dataset: Dataset,
        name: str,
        fields: List[FieldExpr],
        backend: Backend = None,
        source: str = None,
        filterExpr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Tuple[Union[str, FieldExpr], str]] = None,
        limitTo: int = 1000,
    ):
        self.backend = backend
        self.dataset = dataset
        self.source = source
        self.name = name
        self.fields = fields
        self.filterExpr = filterExpr
        self.selection = selection.copy() if selection else None
        self.ordering = ordering.copy() if ordering else None
        self.limitTo = limitTo

        self.nameToField = {field.name: field for field in self.fields}

    def copy(
        self,
        name: str = None,
        fields: List[FieldExpr] = None,
        filterExpr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Tuple[Union[str, FieldExpr], str]] = None,
        limitTo: int = None,
    ) -> "Table":
        return Table(
            backend=self.backend,
            dataset=self.dataset,
            source=self.source,
            name=name or self.name,
            fields=fields or self.fields,
            filterExpr=filterExpr or self.filterExpr,
            selection=selection or self.selection,
            ordering=ordering or self.ordering,
            limitTo=limitTo or self.limitTo,
        )

    def selectedFieldExpr(self, selector: Union[str, FieldExpr]) -> FieldExpr:
        if isinstance(selector, FieldExpr):
            return selector
        elif selector in self.nameToField:
            return self.nameToField[selector]
        else:
            raise ValueError(f"Unknown field selector {selector}")

    def getOrMakeBackend(self) -> Backend:
        if self.backend is None:
            self.backend = makeBackend(self)
        return self.backend

    def filter(self, expr: BooleanFieldExpr) -> "Table":
        return self.copy(filterExpr=expr)

    def select(self, *selection: Union[str, FieldExpr]) -> "Table":
        selectExprs = [self.selectedFieldExpr(s) for s in selection]
        return self.copy(selection=selectExprs)

    def orderBy(self, *ordering: Tuple[Direction, Ordering]) -> "Table":
        orderingExpr = [(self.selectedFieldExpr(sel), dir) for sel, dir in ordering]
        return self.copy(ordering=orderingExpr)

    def limit(self, n: int) -> "Table":
        return self.copy(limitTo=n)

    async def compile(self) -> str:
        backend = self.getOrMakeBackend()
        if backend is not None:
            return await backend.compile(self)
        else:
            raise ValueError("Failed to find a suitable backend to compile this query")

    async def execute(self) -> List[Dict[str, Union[int, str, bool]]]:
        backend = self.getOrMakeBackend()
        if backend is not None:
            return await backend.execute(self)
        else:
            raise ValueError("Failed to find a suitable backend to execute this query")
