from typing import List, Dict
from table import Table

class Backend:
    async def compile(self, query: Table) -> str:
        pass

    async def execute(self, query: Table) -> List[Dict]:
        pass
