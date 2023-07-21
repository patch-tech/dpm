from __future__ import annotations
from typing import List, Union, Tuple, Dict, Literal

from .backends.factory import make_backend
from .backends.interface import Backend
from .field_expr import BooleanFieldExpr, FieldExpr

Direction = Union[Literal["ASC"], Literal["DESC"]]
Ordering = Tuple[FieldExpr, Direction]


class Table:
    def __init__(
        self,
        dataset_name: str,
        dataset_version: str,
        name: str,
        fields: List[FieldExpr],
        backend: Backend = None,
        source: str = None,
        filter_expr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Ordering] = None,
        limit_to: int = 1000,
    ):
        self.backend = backend
        self.dataset_name = dataset_name
        self.dataset_version = dataset_version
        self.source = source
        self.name = name
        self.fields = fields
        self.filter_expr = filter_expr
        self.selection = selection.copy() if selection else None
        self.ordering = ordering.copy() if ordering else None
        self.limit_to = limit_to

        self.name_to_field = {field.name: field for field in self.fields}
        self.get_or_make_backend()

    def copy(
        self,
        name: str = None,
        fields: List[FieldExpr] = None,
        filter_expr: BooleanFieldExpr = None,
        selection: List[FieldExpr] = None,
        ordering: List[Ordering] = None,
        limit_to: int = None,
    ) -> "Table":
        return Table(
            backend=self.backend,
            dataset_name=self.dataset_name,
            dataset_version=self.dataset_version,
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
            raise ValueError(f'Unknown field selector "{selector}"')

    def order_by_expr(self, selector: Union[str, FieldExpr]) -> FieldExpr:
        """Returns a field expression identified by `selector`.
        If `selector` is a `FieldExpr` returns it.
        If `selector` is a table field name, or an alias of a selection,
        then returns that field expression."""
        try:
            return self.selected_field_expr(selector)
        except ValueError as _:
            if isinstance(selector, str) and self.selection:
                f = next(iter([x for x in self.selection if x.alias == selector]), None)
                if f:
                    return f
        raise ValueError(f'Unknown field selector "{selector}"')

    def get_or_make_backend(self) -> Backend:
        if self.backend is None:
            self.backend = make_backend(self)
        return self.backend

    def filter(self, expr: BooleanFieldExpr) -> "Table":
        """
        Sets the filter expression for the table.

        Example usage:
        >>> query = MyTable.select(
        >>>    name,
        >>>    price
        >>>  ).filter(price > 9.99)

        Args:
            expr: Boolean expression to filter by.

        Returns:
            A copy of the table with the filter set.
        """
        return self.copy(filter_expr=expr)

    def select(self, *selection: Union[str, FieldExpr]) -> "Table":
        """
        Sets the fields to select from the table. Accepts a mix of field expressions and field name strings.

        Example usage:
        >>> query = MyTable.select(
        >>>   name,
        >>>   'CATEGORY',
        >>>   sale_date.month.with_alias('saleMonth'),
        >>>   price.avg().with_alias('meanPrice')
        >>> ).limit(10)

        Args:
            selection: Fields to select. Accepts both field expressions or field name strings.

        Returns:
            A copy of the table with the field selection set.
        """
        select_exprs = [self.selected_field_expr(s) for s in selection]
        return self.copy(selection=select_exprs)

    def order_by(
        self, *ordering: List[Tuple[Union[str, FieldExpr], Ordering]]
    ) -> "Table":
        """
        Set the table's ordering columns with their sort direction. The column selectors can be field expressions or strings that refer to table field aliases of selected fields.

        Example usage:
        >>> query = MyTable.select(
        >>>   name,
        >>>   'CATEGORY',
        >>>   sale_date.month.with_alias('saleMonth'),
        >>>   price.avg().with_alias('meanPrice')
        >>> ).order_by(['meanPrice', 'DESC'], [sale_date.month, 'ASC']).limit(10)

        Args:
            ordering: (selector, direction) pairs. The selector can be a field expression or a string referring to a table field alias. The direction can be either 'ASC' for ascending or 'DESC' for descending.

        Returns:
            A copy of the table with the ordering set.
        """
        ordering_expr = [(self.order_by_expr(sel), dir) for sel, dir in ordering]
        return self.copy(ordering=ordering_expr)

    def limit(self, n: int) -> "Table":
        """
        Sets the row limit on the table.

        Example usage:
        >>> table.limit(n)

        Args:
            n: The limit value.

        Returns:
            A copy of the table with the limit set to 'n'.
        """
        return self.copy(limit_to=n)

    async def compile(self) -> str:
        """
        Compiles the table expression into a query string on its execution backend.
        For example, it returns a Snowsql string for a table expression with a Snowflake execution backend.

        Returns:
            The compiled query string.
        """
        backend = self.get_or_make_backend()
        if backend is not None:
            return await backend.compile(self)
        else:
            raise ValueError("Failed to find a suitable backend to compile this query")

    async def execute(self) -> List[Dict]:
        """
        Executes the table expression on its execution backend and resolves to the results.

        Returns:
            The result of executing the table expression on its execution backend.
        """
        backend = self.get_or_make_backend()
        if backend is not None:
            return await backend.execute(self)
        else:
            raise ValueError("Failed to find a suitable backend to execute this query")
