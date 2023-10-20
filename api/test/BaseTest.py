import unittest


class BaseTest(unittest.TestCase):
    def assertLists(self, expected: [any], actual: [any]):
        if type(expected) != type(actual):
            raise ValueError(f"Lists must have the same type.")

        self.assertEqual(len(expected), len(actual))

        for element in expected:
            self.assertIn(element, actual)
