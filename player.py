# -*- coding=utf8 -*-
# here is connect to rust exec and save load functions

import random
from subprocess import Popen, PIPE

random.seed()

syms   = u'0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzйцукенгшщзхъфывапролджэячсмитьбюЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЁЯЧСМИТЬБЮ'

bigSyms = syms + u'ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿĀāĂăĄąĆćĈĉĊċČčĎďĐđĒēĔĕĖėĘęĚěĜĝĞğĠġĢģĤĥĦħĨĩĪīĬĭĮįİıĲĳĴĵĶķĸĹĺĻļĽľĿŀŁłŃńŅņŇňŉŊŋŌōŎŏŐőŒœŔŕŖŗŘřŚśŜŝŞşŠšŢţŤťŦŧŨũŪūŬŭŮůŰűŲųŴŵŶŷŸŹźŻżŽžſƀƁƂƃƄƅƆƇƈƉƊƋƌƍƎƏƐƑƒƓƔƕƖƗƘƙƚƛƜƝƞƟƠơƢƣƤƥƦƧƨƩƪƫƬƭƮƯưƱƲƳƴƵƶƷƸƹƺƻƼƽƾƿǍǎǏǐǑǒǓǔǕǖǗǘǙǚǛǜǝǞǟǠǡǢǣǤǥǦǧǨǩǪǫǬǭǮǯǰǴǵǶǷǸǹǺǻǼǽǾǿȀȁȂȃȄȅȆȇȈȉȊȋȌȍȎȏȐȑȒȓȔȕȖȗȘșȚțȜȝȞȟȠȡȢȣȤȥȦȧȨȩȪȫȬȭȮȯȰȱȲȳȴȵȶȷȸȹȺȻȼȽȾȿɀɁɂɃɄɅɆɇɈɉɊɋɌɍɎɏɐɑɒɓɔɕɖɗɘəɚɛɜɝɞɟɠɡɢɣɤɥɦɧɨɩɪɫɬɭɮɯɰɱɲɳɴɵɶɷɸɹɺɻɼɽɾɿʀʁʂʃʄʅʆʇʈʉʊʋʌʍʎʏʐʑʒʓʔʕʖʗʘʙʚʛʜʝʞʟʠʡʢʣʤʥʦʧʨʩʪʫ'

secret = u'gazon'
aiExec = './exec'

alphabet = {}
def initA():
	i = 0
	for i1 in range(5):
		for i2 in range(5):
			for i3 in range(5):
				alphabet[(i1,i2,i3)] = syms[i]
				alphabet[syms[i]] = (i1,i2,i3)
				i += 1
initA()

def toSyms(nums):
	assert (len(nums) % 3 == 0)
	acc = u''
	for i in range(len(nums) / 3):
		a = i * 3
		b = a + 1
		c = b + 1
		acc = acc + alphabet[(nums[a],nums[b],nums[c])]
	return acc

def toNums(syms):
	acc = []
	for sym in syms:
		(a,b,c) = alphabet[sym]
		acc = acc + [a,b,c]
	return acc

class Board:

	def __init__(self):
		self.diff = 5
		self.data = [[' ' for i in range(0,8)] for _ in range(0,8)]

	def init(self, diff):
		self.diff = diff
		for x in range(8):
			for y in range(5,8):
				if x % 2 != y % 2:
					self.data[y][x] = 'w'
		for x in range(8):
			for y in range(3):
				if x % 2 != y % 2:
					self.data[y][x] = 'b'

	def load(self,text,userId):
		valid = text[1]
		userM = text[0] # now it is diff
		text  = text[2:]
		if valid != syms[hash(userM + text + secret) % len(syms)]:
			return (False, 'VALIDERR')
		#if syms[userId % len(syms)] != userM:
		#	return (False, 'NOTUGAME')
		self.diff = syms.index(userM)
		data = [[' ' for i in range(0,8)] for _ in range(0,8)]
		try:
			nums = toNums(text)
			i = 0
			for x in range(8):
				for y in range(8):
					if x % 2 != y % 2:
						data[y][x] = (' wWbB')[nums[i]]
						i += 1
			self.data = data
			return (True, None)
		except:
			return (False, 'UNPACKERR')

	def save(self,userId):
		nums = []
		for x in range(8):
			for y in range(8):
				if x % 2 != y % 2:
					c = self.data[y][x]
					nums.append((' wWbB').index(c))
		nums.append(0)
		board = toSyms(nums)
		user  = syms[self.diff]#syms[userId % len(syms)]
		valid = syms[hash(user + board + secret) % len(syms)]
		return user + valid + board

	def get(self,(x,y)):
		return self.data[y][x]

	def put(self,(x,y),v):
		self.data[y][x] = v

	def show(self):
		for line in self.data:
			print reduce(str.__add__, line)
	
	def aiRequest(self):
		acc = ['%d b' % self.diff]
		keys = {'w': ('w','r'), 'W': ('w','k'), 'b': ('b','r'), 'B': ('b','k')}
		for x in range(8):
			for y in range(8):
				c = self.data[y][x]
				if c in keys:
					(hd,tl) = keys[c]
					acc.append(hd)
					acc.append(str(x))
					acc.append(str(y))
					acc.append(tl)
		req = reduce(lambda a,b: a + ' ' + b, acc)
		proc = Popen(aiExec, stdin=PIPE, stdout=PIPE)
		proc.stdin.write(req)
		proc.stdin.flush()
		proc.stdin.close()
		ans = proc.stdout.read()
		try:
			((x1,y1), (x2,y2), dead) = eval(ans)
			obj = self.data[y1][x1]
			if y2 == 7:
				self.data[y2][x2] = 'B'
			else:
				self.data[y2][x2] = obj
			self.data[y1][x1] = ' '
			for x,y in dead:
				self.data[y][x] = ' '
			return True
		except:
			return False
	
	def stepsFor(self,x,y,showDead=False):
		brd = self.data
		cell = brd[y][x]
		steps = []
		eats  = []

		def freeCell(x,y):
			if 0 <= x <= 7 and 0 <= y <= 7:
				return brd[y][x] == ' '
			else:
				return False
		def blackAt(x,y):
			if 0 <= x <= 7 and 0 <= y <= 7:
				c = brd[y][x]
				return c == 'b' or c == 'B'
			else:
				return False
		def tryMove(px,py):
			acc = []
			sx = x
			sy = y
			while True:
				sx += px
				sy += py
				if freeCell(sx,sy):
					acc.append( ([(x,y), (sx,sy)], []) )
				else:
					break
			return acc
		def tryEat(path,px,py,inactive):
			(sx,sy) = path[-1]
			while True:
				sx += px
				sy += py
				if freeCell(sx,sy):
					continue
				elif blackAt(sx,sy) and not ((sx,sy) in inactive): # found!
					inactive = inactive + [(sx,sy)]
					sx += px
					sy += py
					if freeCell(sx,sy):
						steps = []
						eats  = []
						while True:
							print 'AV', sx, sy
							if freeCell(sx,sy):
								locPath = path + [(sx,sy)]
								steps.append( (locPath, inactive) )
								for px1,py1 in [(1,1),(1,-1),(-1,1),(-1,-1)]:
									eats = eats + tryEat(locPath, px1, py1, inactive)
							else:
								break
							sx += px
							sy += py
						return steps if eats == [] else eats
#						path = path + [(sx,sy)]
#						for px,py in [(1,1),(1,-1),(-1,1),(-1,-1)]:
#							acc = acc + tryEat(path,px,py,inactive)
#						if acc == []:
#							return [(path, inactive)]
					else:
						return []
				else:
					return []
		def tryEatSmall(path, inactive):
			(x,y) = path[-1]
			acc = []
			for px,py in [(1,1),(1,-1),(-1,1),(-1,-1)]:
				bx, by = x + px, y + py
				wx, wy = x + px + px, y + py + py
				if freeCell(wx,wy) and blackAt(bx, by) and not ((bx,by) in inactive):
					acc = acc + tryEatSmall(path + [(wx,wy)], inactive + [(bx,by)])
			if acc == []:
				return [(path, inactive)]
			else:
				return acc

		if cell == 'w':
			if freeCell(x-1, y-1): 
				steps.append( ([(x,y), (x-1, y-1)], []) )
			if freeCell(x+1, y-1):
				steps.append( ([(x,y), (x+1, y-1)], []) )
			if blackAt(x-1, y-1) and freeCell(x-2, y-2):
				eats = eats + tryEatSmall( [(x,y), (x-2,y-2)], [(x-1, y-1)] )
			if blackAt(x+1,y-1) and freeCell(x+2, y-2):
				eats = eats + tryEatSmall( [(x,y), (x+2,y-2)], [(x+1, y-1)] )
			if blackAt(x+1,y+1) and freeCell(x+2,y+2):
				eats = eats + tryEatSmall( [(x,y), (x+2,y+2)], [(x+1, y+1)] )
			if blackAt(x-1,y+1) and freeCell(x-2,y+2):
				eats = eats + tryEatSmall( [(x,y), (x-2,y+2)], [(x-1, y+1)] )
		elif cell == 'W':
			steps = tryMove(1,1) + tryMove(1,-1) + tryMove(-1,1) + tryMove(-1,-1)
			eats  = tryEat([(x,y)],1,1,[]) + tryEat([(x,y)],1,-1,[]) + tryEat([(x,y)],-1,1,[]) + tryEat([(x,y)],-1,-1,[])

		if eats == []:
			ans = steps
		else:
			ans = eats
		if showDead:
			return ans
		else:
			return map(lambda l: l[0], ans)

	def available(self):
		steps = []
		eat = []
		for x in range(8):
			for y in range(8):
				if self.data[y][x] == 'w' or self.data[y][x] == 'W':
					ways = self.stepsFor(x,y,showDead=True)
					stat = None
					for _,dead in ways:
						if dead == []:
							stat = 'M'
						else:
							stat = 'E'
							break
					if stat:
						if stat == 'M':
							steps.append((x,y))
						else:
							eat.append((x,y))
		if eat == []:
			return steps
		else:
			return eat
					
