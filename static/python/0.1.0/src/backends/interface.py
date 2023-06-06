from typing import List, Dict


class Backend:
    async def compile(self, query) -> str:
        pass

    async def execute(self, query: Table) -> List[Dict]:
        pass
