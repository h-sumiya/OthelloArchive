from pathlib import Path


class DataPath:
    PARENT = Path(__file__).parent / "datas"

    def id(id: int) -> Path:
        return DataPath.PARENT / f"{id}.bin"


class ModelPath:
    PARENT = Path(__file__).parent / "models"

    def id(id: int) -> Path:
        return ModelPath.PARENT / f"{id}.pth"
