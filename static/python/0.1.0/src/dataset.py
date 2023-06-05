class Dataset:
    def __init__(self, name: str, version: str):
        self.table_by_name = {}
        self.name = name
        self.version = version

    def add_table(self, table) -> None:
        if table.name in self.table_by_name:
            raise ValueError(f"Table named {table.name} already exists")
        self.table_by_name[table.name] = table

    def get_table(self, name: str):
        return self.table_by_name.get(name)
    