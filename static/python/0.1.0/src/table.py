from typing import List, Union, Tuple, Dict, Literal

from backends.factory import make_backend
from backends.interface import Backend
from dataset import Dataset
from field_expr import BooleanFieldExpr, FieldExpr

Direction = Union[Literal["ASC"], Literal["DESC"]]
Ordering = Tuple[FieldExpr, Direction]


class Table:
    def __init__(
        self,
        dataset: Dataset,
        name: str,
        fields: List[FieldExpr],
        backend: Backend = None,
        source: str = None,
        filter_expr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Tuple[Union[str, FieldExpr], str]] = None,
        limit_to: int = 1000,
    ):
        self.backend = backend
        self.dataset = dataset
        self.source = source
        self.name = name
        self.fields = fields
        self.filter_expr = filter_expr
        self.selection = selection.copy() if selection else None
        self.ordering = ordering.copy() if ordering else None
        self.limit_to = limit_to

        self.name_to_field = {field.name: field for field in self.fields}

    def copy(
        self,
        name: str = None,
        fields: List[FieldExpr] = None,
        filter_expr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Tuple[Union[str, FieldExpr], str]] = None,
        limit_to: int = None,
    ) -> "Table":
        return Table(
            backend=self.backend,
            dataset=self.dataset,
            source=self.source,
            name=name or self.name,
            fields=fields or self.fields,
            filter_expr=filter_expr or self.filter_expr,
            selection=selection or self.selection,
            ordering=ordering or self.ordering,
            limit_to=limit_to or self.limit_to,
        )

    def selected_field_expr(self, selector: Union[str, FieldExpr]) -> FieldExpr:
        if isinstance(selector, FieldExpr):
            return selector
        elif selector in self.name_to_field:
            return self.name_to_field[selector]
        else:
            raise ValueError(f"Unknown field selector {selector}")

    def get_or_make_backend(self) -> Backend:
        if self.backend is None:
            self.backend = make_backend(self)
        return self.backend

    def filter(self, expr: BooleanFieldExpr) -> "Table":
        return self.copy(filter_expr=expr)

    def select(self, *selection: Union[str, FieldExpr]) -> "Table":
        select_exprs = [self.selected_field_expr(s) for s in selection]
        return self.copy(selection=select_exprs)

    def order_by(self, *ordering: Tuple[Direction, Ordering]) -> "Table":
        ordering_expr = [(self.selected_field_expr(sel), dir) for sel, dir in ordering]
        return self.copy(ordering=ordering_expr)

    def limit(self, n: int) -> "Table":
        return self.copy(limit_to=n)

    async def compile(self) -> str:
        backend = self.get_or_make_backend()
        if backend is not None:
            return await backend.compile(self)
        else:
            raise ValueError("Failed to find a suitable backend to compile this query")

    async def execute(self) -> List[Dict]:
        backend = self.get_or_make_backend()
        if backend is not None:
            return await backend.execute(self)
        else:
            raise ValueError("Failed to find a suitable backend to execute this query")
