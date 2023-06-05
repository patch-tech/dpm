from typing import List, Dict

class Backend:
    async def compile(self, query) -> str:
        pass

    async def execute(self, query) -> List[Dict]:
        pass
    