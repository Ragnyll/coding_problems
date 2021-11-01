#  Given a time in

#  -hour AM/PM format, convert it to military (24-hour) time.

#  Note: - 12:00:00AM on a 12-hour clock is 00:00:00 on a 24-hour clock.
#  - 12:00:00PM on a 12-hour clock is 12:00:00 on a 24-hour clock.
import unittest

def time_conversion(s: str) -> str:
    """Given a 12 hour converts it to a 24 hr time.

    :param s: str representing a 12hr time
    :rtype: str
    """
    is_am = False

    t_fields = s.split(':')
    if t_fields[-1][-2:] == 'AM':
        is_am = True

    if is_am and t_fields[0] == '12':
        t_fields[0] = '00'
    elif t_fields[0] == '12':
        t_fields[0] = '12'
    elif not is_am:
        t_fields[0] = str(int(t_fields[0]) + 12)

    # drop the AM or PM string
    t_fields[-1] = t_fields[-1][0:-2]

    return ':'.join(t_fields)


class TestTimeConversion(unittest.TestCase):
    def test_time_conversion_am(self):
        self.assertEqual(time_conversion('07:05:45AM'), '07:05:45');

    def test_time_conversion_pm(self):
        self.assertEqual(time_conversion('07:05:45PM'), '19:05:45');

    def test_time_conversion_12am(self):
        self.assertEqual(time_conversion('12:05:45AM'), '00:05:45');

    def test_time_conversion_12pm(self):
        self.assertEqual(time_conversion('12:05:45PM'), '12:05:45');
