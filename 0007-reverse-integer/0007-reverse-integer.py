class Solution:
	def reverse(self, x: int) -> int:
		if x==0:
			return 0
		s=str(x)
		s=s[::-1]
		if x<0:
			s="-"+s.rstrip("-")
		if len(s)==10:
			if s>"2147483647" :
				return 0
		if len(s)==11:
			if s>"-2147483648" :
				return 0
		return int(s)