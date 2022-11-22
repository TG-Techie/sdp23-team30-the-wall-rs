class Option:
    def unwrap(self):
        if isinstance(self, Some):
            return self.value
        else:
            raise ValueError("")


class Some(Option):
    def __init__(self, value):
        self.value = value


class None_(Option):
    pass


a: Option = None_()
a: Option = Some(5)
